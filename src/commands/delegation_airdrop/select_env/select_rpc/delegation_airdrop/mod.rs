use crate::near::types::NearEnv;
use crate::near::util::{get_accounts_from_path, get_default_near_account_dir_path};
use crate::oct::contracts::anchor::AnchorContract;
use crate::CliResult;
use crate::{near::rpc::client::Client, oct::contracts::fungible_token::FungibleTokenContract};
use appchain_anchor::types::{AppchainValidator, FTDepositMessage};
use dialoguer::Input;
use near_crypto::InMemorySigner;
use near_primitives::types::AccountId;
use near_sdk::json_types::U128;
use std::{collections::HashSet, fs, str::FromStr};

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct DelegationAirdrop {
    pub anchor_account: String,
    pub oct_fund_account: String,
    pub airdrop_amount: String,
    pub account_list_file: String,
}

impl DelegationAirdrop {
    //
    pub fn input_anchor_account(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok(Input::new()
            .with_prompt("What is the anchor account you want to perform airdrop?")
            .interact_text()?)
    }
    //
    pub fn input_oct_fund_account(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok(Input::new()
            .with_prompt("What is the OCT fund account you want to perform airdrop? (You need the private key of this account for signing transfer transactions.)")
            .interact_text()?)
    }
    //
    pub fn input_airdrop_amount(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok(Input::new()
            .with_prompt("What is the airdrop amount to accounts of the airdrop?")
            .interact_text()?)
    }
    //
    pub fn input_account_list_file(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok(Input::new()
            .with_prompt("What is the account list file of the airdrop?")
            .interact_text()?)
    }
    //
    pub async fn process(self, connection_config: NearEnv, client: Client) -> CliResult {
        let result = get_accounts_from_path(
            get_default_near_account_dir_path(&connection_config).as_path(),
        )?;
        let possible_signers: Vec<InMemorySigner> = result.into_iter().map(|e| e.into()).collect();
        let oct_token_account = match connection_config {
            NearEnv::Mainnet => {
                "f5cfbc74057c610c8ef151a439252680ac68c6dc.factory.bridge.near".to_string()
            }
            NearEnv::Testnet => "oct.beta_oct_relay.testnet".to_string(),
        };
        let oct_token = FungibleTokenContract {
            account_id: AccountId::try_from(oct_token_account).unwrap(),
            client: &client,
        };
        let fund_balance = oct_token
            .ft_balance_of(
                &AccountId::try_from(self.oct_fund_account.clone()).map_err(|err| {
                    color_eyre::Report::msg(format!(
                        "Invalid anchor account: '{}'. {}",
                        self.anchor_account, err
                    ))
                })?,
            )
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Failed to get OCT balance of fund account '{}'. {}",
                    self.oct_fund_account, err
                ))
            })?;
        let anchor = AnchorContract {
            account_id: AccountId::try_from(self.anchor_account.clone()).map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Invalid anchor account: '{}'. {}",
                    self.anchor_account, err
                ))
            })?,
            client: &client,
        };
        let bytes = fs::read(self.account_list_file.clone()).map_err(|err| {
            color_eyre::Report::msg(format!(
                "Failed to read airdrop account list file '{}'. {}",
                self.account_list_file, err
            ))
        })?;
        let file_content = String::from_utf8_lossy(&bytes);
        let mut airdrop_accounts_set: HashSet<AccountId> = HashSet::new();
        for line in String::from_utf8_lossy(&bytes).lines() {
            if !airdrop_accounts_set.insert(AccountId::try_from(line.to_string()).map_err(
                |err| {
                    color_eyre::Report::msg(format!("Invalid airdrop account '{}'. {}", line, err))
                },
            )?) {
                println!("Duplicated airdrop account '{}'.", line);
            };
        }
        if airdrop_accounts_set.len() != file_content.lines().count() {
            return Err(color_eyre::Report::msg(
                "Duplicated account(s) existed in airdrop account list file. Processing stopped.",
            ));
        }
        let airdrop_amount =
            serde_json::from_str::<U128>(format!("\"{}\"", self.airdrop_amount).as_str()).unwrap();
        assert!(
            fund_balance.0 >= airdrop_amount.0 * airdrop_accounts_set.len() as u128,
            "OCT balance of fund account is not enough."
        );
        let fund_account_id = AccountId::from_str(&self.oct_fund_account).unwrap();
        let mut fund_account_signer: Option<InMemorySigner> = None;
        for signer in possible_signers {
            if signer.account_id.eq(&fund_account_id) {
                fund_account_signer = Some(signer);
                break;
            }
        }
        if fund_account_signer.is_none() {
            return Err(color_eyre::Report::msg(format!(
                "Missing key for OCT fund account '{}'. Processing stopped.",
                self.oct_fund_account
            )));
        }
        for account in airdrop_accounts_set.iter() {
            let result = airdrop_to(
                account,
                &oct_token,
                &anchor,
                fund_account_signer.as_ref().unwrap(),
                &airdrop_amount,
            )
            .await;
            if result.is_err() {
                return result;
            }
        }
        //
        Ok(())
    }
}

async fn airdrop_to(
    account_id: &AccountId,
    oct_token: &FungibleTokenContract<'_>,
    anchor: &AnchorContract<'_>,
    fund_account: &InMemorySigner,
    amount: &U128,
) -> CliResult {
    //
    // Check whether the account is a valid validator
    //
    let validator_list = anchor.get_validator_list_of(None).await.map_err(|err| {
        color_eyre::Report::msg(format!("Failed to get latest validator list. {}", err))
    })?;
    let mut is_validator = false;
    for validator in &validator_list {
        if validator
            .validator_id
            .eq(&near_sdk::AccountId::from_str(account_id.as_str()).unwrap())
        {
            is_validator = true;
            break;
        }
    }
    if is_validator {
        let deposit_message = FTDepositMessage::IncreaseStake {
            validator_id: Some(near_sdk::AccountId::from_str(account_id.as_str()).unwrap()),
        };
        let result = oct_token
            .ft_transfer_call(
                &fund_account,
                &anchor.account_id,
                amount,
                &Some(serde_json::to_string(&deposit_message).unwrap()),
            )
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Failed to call ft_transfer_call on OCT token contract. {}",
                    err
                ))
            })?;
        println!("Airdrop result for '{}': {:?}", account_id, result);
        return Ok(());
    }
    //
    // Check whether the account is a valid delegator
    //
    let mut delegated_validator: Option<AppchainValidator> = None;
    for validator in &validator_list {
        let delegator_list = anchor
            .get_delegators_of_validator_in_era(
                None,
                AccountId::from_str(validator.validator_id.as_str()).unwrap(),
            )
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Failed to get delegators of validator '{}'. {}",
                    validator.validator_id, err
                ))
            })?;
        for delegator in delegator_list {
            if delegator
                .delegator_id
                .eq(&near_sdk::AccountId::from_str(account_id.as_str()).unwrap())
            {
                delegated_validator = Some(validator.clone());
                break;
            }
        }
        if delegated_validator.is_some() {
            break;
        }
    }
    if let Some(delegated_validator) = delegated_validator {
        let deposit_message = FTDepositMessage::IncreaseDelegation {
            validator_id: near_sdk::AccountId::from_str(delegated_validator.validator_id.as_str())
                .unwrap(),
            delegator_id: Some(near_sdk::AccountId::from_str(account_id.as_str()).unwrap()),
        };
        let result = oct_token
            .ft_transfer_call(
                &fund_account,
                &anchor.account_id,
                amount,
                &Some(serde_json::to_string(&deposit_message).unwrap()),
            )
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Failed to call ft_transfer_call on OCT token contract. {}",
                    err
                ))
            })?;
        println!("Airdrop result for '{}': {:?}", account_id, result);
        return Ok(());
    }
    //
    // Register the account as a new delegator
    //
    let mut target_validator: Option<AppchainValidator> = None;
    let mut target_total_stake: U128 = U128::from(0);
    for validator in &validator_list {
        if validator.can_be_delegated_to {
            if validator.total_stake < target_total_stake || target_total_stake == 0.into() {
                target_validator = Some(validator.clone());
            }
            if let Some(target_validator) = &target_validator {
                target_total_stake = target_validator.total_stake;
            }
        }
    }
    if let Some(target_validator) = target_validator {
        let deposit_message = FTDepositMessage::RegisterDelegator {
            validator_id: near_sdk::AccountId::from_str(target_validator.validator_id.as_str())
                .unwrap(),
            delegator_id: Some(near_sdk::AccountId::from_str(account_id.as_str()).unwrap()),
        };
        let result = oct_token
            .ft_transfer_call(
                &fund_account,
                &anchor.account_id,
                amount,
                &Some(serde_json::to_string(&deposit_message).unwrap()),
            )
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Failed to call ft_transfer_call on OCT token contract. {}",
                    err
                ))
            })?;
        println!("Airdrop result for '{}': {:?}", account_id, result);
        return Ok(());
    } else {
        return Err(color_eyre::Report::msg(format!(
            "There is no validator for '{}' to delegate to.",
            account_id,
        )));
    }
}
