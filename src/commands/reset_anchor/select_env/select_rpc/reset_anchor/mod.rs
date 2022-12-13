use crate::near::call_contract_function_and_parse_result;
use crate::near::rpc::client::Client;
use crate::near::types::NearEnv;
use crate::near::util::{get_accounts_from_path, get_default_near_account_dir_path};
use crate::oct::contracts::anchor::AnchorContract;
use crate::CliResult;
use appchain_anchor::types::MultiTxsOperationProcessingResult;
use dialoguer::Input;
use near_crypto::InMemorySigner;
use near_primitives::types::AccountId;
use near_sdk::json_types::U64;
use serde_json::json;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct ResetAnchor {
    pub anchor_account: String,
}

impl ResetAnchor {
    //
    pub fn input_anchor_account(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok(Input::new()
            .with_prompt("What is the anchor contract account you want to reset? (This operation will remove ALL data in the contract. Be careful!)")
            .interact_text()?)
    }
    //
    pub async fn process(self, connection_config: NearEnv, client: Client) -> CliResult {
        let result = get_accounts_from_path(
            get_default_near_account_dir_path(&connection_config).as_path(),
        )?;
        let possible_signers: Vec<InMemorySigner> = result.into_iter().map(|e| e.into()).collect();
        let anchor = AnchorContract {
            account_id: AccountId::try_from(self.anchor_account.clone()).map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Invalid anchor account: '{}'. {}",
                    self.anchor_account, err
                ))
            })?,
            client: &client,
        };
        let mut anchor_signer: Option<InMemorySigner> = None;
        for signer in possible_signers {
            if signer.account_id.eq(&anchor.account_id) {
                anchor_signer = Some(signer);
                break;
            }
        }
        if anchor_signer.is_none() {
            return Err(color_eyre::Report::msg(format!(
                "Missing key for anchor account '{}'. Processing stopped.",
                self.anchor_account
            )));
        }
        //
        let anchor_status = anchor.get_anchor_status().await.map_err(|err| {
            color_eyre::Report::msg(format!(
                "Failed to get status of anchor '{}'. {}",
                self.anchor_account, err
            ))
        })?;
        let mut era_number = anchor_status
            .index_range_of_validator_set_history
            .end_index
            .0;
        loop {
            //
            call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                &anchor.client,
                &anchor.account_id,
                anchor_signer.as_ref().unwrap(),
                "clear_reward_distribution_records".to_string(),
                json!({
                    "era_number": U64::from(era_number),
                })
                .to_string(),
            )
            .await?;
            //
            call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                &anchor.client,
                &anchor.account_id,
                anchor_signer.as_ref().unwrap(),
                "clear_unwithdrawn_rewards".to_string(),
                json!({
                    "era_number": U64::from(era_number),
                })
                .to_string(),
            )
            .await?;
            //
            loop {
                match call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                    &anchor.client,
                    &anchor.account_id,
                    anchor_signer.as_ref().unwrap(),
                    "remove_validator_set_history_of".to_string(),
                    json!({
                        "era_number": U64::from(era_number),
                    })
                    .to_string(),
                )
                .await?
                .unwrap()
                {
                    MultiTxsOperationProcessingResult::NeedMoreGas => continue,
                    MultiTxsOperationProcessingResult::Ok => break,
                    MultiTxsOperationProcessingResult::Error(_) => {
                        return Err(color_eyre::Report::msg(format!(
                        "Failed to finish removing validator set histories. Processing Stopped.",
                    )))
                    }
                }
            }
            //
            if era_number == 0 {
                break;
            } else {
                era_number -= 1;
            }
        }
        //
        loop {
            match call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                &anchor.client,
                &anchor.account_id,
                anchor_signer.as_ref().unwrap(),
                "clear_validator_set_histories".to_string(),
                json!({}).to_string(),
            )
            .await?
            .unwrap()
            {
                MultiTxsOperationProcessingResult::NeedMoreGas => continue,
                MultiTxsOperationProcessingResult::Ok => break,
                MultiTxsOperationProcessingResult::Error(_) => {
                    return Err(color_eyre::Report::msg(format!(
                        "Failed to clear validator set histories. Processing Stopped.",
                    )))
                }
            }
        }
        //
        loop {
            match call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                &anchor.client,
                &anchor.account_id,
                anchor_signer.as_ref().unwrap(),
                "clear_next_validator_set".to_string(),
                json!({}).to_string(),
            )
            .await?
            .unwrap()
            {
                MultiTxsOperationProcessingResult::NeedMoreGas => continue,
                MultiTxsOperationProcessingResult::Ok => break,
                MultiTxsOperationProcessingResult::Error(_) => {
                    return Err(color_eyre::Report::msg(format!(
                        "Failed to clear next validator set. Processing Stopped.",
                    )))
                }
            }
        }
        //
        loop {
            match call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                &anchor.client,
                &anchor.account_id,
                anchor_signer.as_ref().unwrap(),
                "clear_user_staking_histories".to_string(),
                json!({}).to_string(),
            )
            .await?
            .unwrap()
            {
                MultiTxsOperationProcessingResult::NeedMoreGas => continue,
                MultiTxsOperationProcessingResult::Ok => break,
                MultiTxsOperationProcessingResult::Error(_) => {
                    return Err(color_eyre::Report::msg(format!(
                        "Failed to clear user staking histories. Processing Stopped.",
                    )))
                }
            }
        }
        //
        loop {
            match call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                &anchor.client,
                &anchor.account_id,
                anchor_signer.as_ref().unwrap(),
                "clear_unbonded_stakes_and_staking_histories".to_string(),
                json!({}).to_string(),
            )
            .await?
            .unwrap()
            {
                MultiTxsOperationProcessingResult::NeedMoreGas => continue,
                MultiTxsOperationProcessingResult::Ok => break,
                MultiTxsOperationProcessingResult::Error(_) => {
                    return Err(color_eyre::Report::msg(format!(
                        "Failed to clear unbonded stakes and staking histories. Processing Stopped.",
                    )))
                }
            }
        }
        //
        loop {
            match call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                &anchor.client,
                &anchor.account_id,
                anchor_signer.as_ref().unwrap(),
                "clear_validator_profiles".to_string(),
                json!({}).to_string(),
            )
            .await?
            .unwrap()
            {
                MultiTxsOperationProcessingResult::NeedMoreGas => continue,
                MultiTxsOperationProcessingResult::Ok => break,
                MultiTxsOperationProcessingResult::Error(_) => {
                    return Err(color_eyre::Report::msg(format!(
                        "Failed to clear validator profiles. Processing Stopped.",
                    )))
                }
            }
        }
        //
        loop {
            match call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                &anchor.client,
                &anchor.account_id,
                anchor_signer.as_ref().unwrap(),
                "clear_appchain_messages".to_string(),
                json!({}).to_string(),
            )
            .await?
            .unwrap()
            {
                MultiTxsOperationProcessingResult::NeedMoreGas => continue,
                MultiTxsOperationProcessingResult::Ok => break,
                MultiTxsOperationProcessingResult::Error(_) => {
                    return Err(color_eyre::Report::msg(format!(
                        "Failed to clear appchain messages. Processing Stopped.",
                    )))
                }
            }
        }
        //
        loop {
            match call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                &anchor.client,
                &anchor.account_id,
                anchor_signer.as_ref().unwrap(),
                "clear_appchain_notification_histories".to_string(),
                json!({}).to_string(),
            )
            .await?
            .unwrap()
            {
                MultiTxsOperationProcessingResult::NeedMoreGas => continue,
                MultiTxsOperationProcessingResult::Ok => break,
                MultiTxsOperationProcessingResult::Error(_) => {
                    return Err(color_eyre::Report::msg(format!(
                        "Failed to clear appchain notification histories. Processing Stopped.",
                    )))
                }
            }
        }
        //
        loop {
            match call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
                &anchor.client,
                &anchor.account_id,
                anchor_signer.as_ref().unwrap(),
                "clear_appchain_challenges".to_string(),
                json!({}).to_string(),
            )
            .await?
            .unwrap()
            {
                MultiTxsOperationProcessingResult::NeedMoreGas => continue,
                MultiTxsOperationProcessingResult::Ok => break,
                MultiTxsOperationProcessingResult::Error(_) => {
                    return Err(color_eyre::Report::msg(format!(
                        "Failed to clear appchain challenges. Processing Stopped.",
                    )))
                }
            }
        }
        //
        call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
            &anchor.client,
            &anchor.account_id,
            anchor_signer.as_ref().unwrap(),
            "clear_external_assets_registration".to_string(),
            json!({}).to_string(),
        )
        .await?;
        //
        call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
            &anchor.client,
            &anchor.account_id,
            anchor_signer.as_ref().unwrap(),
            "remove_staged_wasm".to_string(),
            json!({}).to_string(),
        )
        .await?;
        //
        call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
            &anchor.client,
            &anchor.account_id,
            anchor_signer.as_ref().unwrap(),
            "clear_contract_level_lazy_option_values".to_string(),
            json!({}).to_string(),
        )
        .await?;
        //
        let key_values = anchor
            .client
            .view_state(anchor.account_id.clone(), None, None)
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Failed to view state of anchor contract '{}'. {}",
                    self.anchor_account, err
                ))
            })?
            .values;
        if key_values.len() > 10 {
            return Err(color_eyre::Report::msg(format!(
                "Too many storage keys remained. Processing stopped.",
            )));
        }
        let mut keys = Vec::new();
        for item in key_values {
            keys.push(base64::encode(item.key));
        }
        //
        #[derive(near_sdk::serde::Serialize)]
        #[serde(crate = "near_sdk::serde")]
        struct Input {
            keys: Vec<String>,
        }
        let args = Input { keys };
        call_contract_function_and_parse_result::<MultiTxsOperationProcessingResult>(
            &anchor.client,
            &anchor.account_id,
            anchor_signer.as_ref().unwrap(),
            "remove_storage_keys".to_string(),
            serde_json::to_string(&args).unwrap(),
        )
        .await?;
        //
        Ok(())
    }
}
