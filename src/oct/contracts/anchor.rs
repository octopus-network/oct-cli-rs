use near_primitives::types::AccountId;
use near_sdk::json_types::U64;
use serde_json::json;

use crate::near::rpc::client::Client;
use crate::oct::contracts::NearContract;
use appchain_anchor::types::{
    AnchorStatus, AppchainDelegator, AppchainState, AppchainValidator, ProtocolSettings,
    RewardHistory, ValidatorSetInfo,
};

pub struct AnchorContract<'s> {
    pub account_id: AccountId,
    pub client: &'s Client,
}

impl<'s> NearContract<'s> for AnchorContract<'s> {
    fn get_account_id(&self) -> &AccountId {
        &self.account_id
    }

    fn get_client(&self) -> &'s Client {
        self.client
    }
}

impl<'s> AnchorContract<'s> {
    pub fn new(account_id: AccountId, client: &'s Client) -> Self {
        return Self { account_id, client };
    }

    pub async fn get_anchor_status(&self) -> anyhow::Result<AnchorStatus> {
        self.client
            .view(
                self.account_id.clone(),
                "get_anchor_status".to_string(),
                json!({}).to_string().into_bytes(),
            )
            .await
            .map(|e| e.json().unwrap())
    }

    pub async fn get_validator_rewards_of(
        &self,
        start_era: u64,
        end_era: u64,
        validator_id: AccountId,
    ) -> anyhow::Result<Vec<RewardHistory>> {
        self.client
            .view(
                self.account_id.clone(),
                "get_validator_rewards_of".to_string(),
                json!({
                    "start_era": start_era.to_string(),
                    "end_era": end_era.to_string(),
                    "validator_id": validator_id
                })
                .to_string()
                .into_bytes(),
            )
            .await
            .map(|e| e.json().unwrap())
    }

    pub async fn get_validator_list_of(
        &self,
        era_number: Option<u64>,
    ) -> anyhow::Result<Vec<AppchainValidator>> {
        self.client
            .view(
                self.account_id.clone(),
                "get_validator_list_of".to_string(),
                json!({
                    "era_number": era_number,
                })
                .to_string()
                .into_bytes(),
            )
            .await
            .map(|e| e.json().unwrap())
    }

    pub async fn get_protocol_settings(&self) -> anyhow::Result<ProtocolSettings> {
        self.client
            .view(
                self.account_id.clone(),
                "get_protocol_settings".to_string(),
                json!({}).to_string().into_bytes(),
            )
            .await
            .map(|e| e.json().unwrap())
    }

    pub async fn get_validator_set_info_of(
        &self,
        era_number: u64,
    ) -> anyhow::Result<Option<ValidatorSetInfo>> {
        self.client
            .view(
                self.account_id.clone(),
                "get_validator_set_info_of".to_string(),
                json!({
                    "era_number": era_number.to_string()
                })
                .to_string()
                .into_bytes(),
            )
            .await
            .map(|e| e.json().unwrap())
    }
    //
    pub async fn get_appchain_state(&self) -> anyhow::Result<AppchainState> {
        self.client
            .view(
                self.account_id.clone(),
                "get_appchain_state".to_string(),
                json!({}).to_string().into_bytes(),
            )
            .await
            .map(|e| e.json().unwrap())
    }
    //
    pub async fn get_delegators_of_validator_in_era(
        &self,
        era_number: Option<U64>,
        validator_id: AccountId,
    ) -> anyhow::Result<Vec<AppchainDelegator>> {
        self.client
            .view(
                self.account_id.clone(),
                "get_delegators_of_validator_in_era".to_string(),
                json!({
                    "era_number": era_number,
                    "validator_id": validator_id,
                })
                .to_string()
                .into_bytes(),
            )
            .await
            .map(|e| e.json().unwrap())
    }
}

#[tokio::test]
pub async fn test_get_validator_set_info_of() -> anyhow::Result<()> {
    let client = Client::new("https://public-rpc.blockpi.io/http/near");

    let anchor_contract = AnchorContract {
        account_id: "fusotao.octopus-registry.near".parse().unwrap(),
        client: &client,
    };
    let validator_set_info = anchor_contract.get_validator_set_info_of(51).await.unwrap();

    println!("{}", serde_json::to_string(&validator_set_info).unwrap());

    Ok(())
}
#[tokio::test]
pub async fn test_get_protocol_setting() -> anyhow::Result<()> {
    let client = Client::new("https://public-rpc.blockpi.io/http/near");

    let anchor_contract = AnchorContract {
        account_id: "fusotao.octopus-registry.near".parse().unwrap(),
        client: &client,
    };
    let protocol_setting = anchor_contract.get_protocol_settings().await.unwrap();

    let raw = r#"
        {
  "minimum_validator_deposit": "5000000000000000000000",
  "minimum_validator_deposit_changing_amount": "1000000000000000000000",
  "maximum_validator_stake_percent": 25,
  "minimum_delegator_deposit": "200000000000000000000",
  "minimum_delegator_deposit_changing_amount": "100000000000000000000",
  "minimum_total_stake_price_for_booting": "100000000000",
  "maximum_market_value_percent_of_near_fungible_tokens": 100,
  "maximum_market_value_percent_of_wrapped_appchain_token": 67,
  "minimum_validator_count": "4",
  "maximum_validator_count": "60",
  "maximum_validators_per_delegator": "16",
  "unlock_period_of_validator_deposit": "21",
  "unlock_period_of_delegator_deposit": "21",
  "maximum_era_count_of_unwithdrawn_reward": "84",
  "maximum_era_count_of_valid_appchain_message": "7",
  "validator_commission_percent": 20,
  "maximum_allowed_unprofitable_era_count": 3
}
    "#;
    // let protocol_setting: ProtocolSettings = serde_json::from_str(raw).unwrap();

    println!("{}", serde_json::to_string(&protocol_setting).unwrap());
    Ok(())
}

#[tokio::test]
pub async fn test_get_anchor_status() -> anyhow::Result<()> {
    let client = Client::new("https://public-rpc.blockpi.io/http/near");

    let anchor_contract = AnchorContract {
        account_id: "fusotao.octopus-registry.near".parse().unwrap(),
        client: &client,
    };
    let status = anchor_contract.get_anchor_status().await.unwrap();
    println!("{}", serde_json::to_string(&status).unwrap());
    Ok(())
}

#[tokio::test]
pub async fn test_get_validator_rewards_of() -> anyhow::Result<()> {
    let client = Client::new("https://public-rpc.blockpi.io/http/near");

    let anchor_contract = AnchorContract {
        account_id: "fusotao.octopus-registry.near".parse().unwrap(),
        client: &client,
    };
    let rewards = anchor_contract
        .get_validator_rewards_of(10, 50, "manhmai11.near".parse()?)
        .await
        .unwrap();
    println!("{}", serde_json::to_string(&rewards).unwrap());
    Ok(())
}

#[tokio::test]
pub async fn test_deploy() -> anyhow::Result<()> {
    use crate::near::util::print_transaction_status;

    let client = Client::new("https://public-rpc.blockpi.io/http/near-testnet");

    let signer = crate::near::types::NearAccountWithKey::from_file(&std::path::Path::new(
        "/Users/xushenbao/.near-credentials/testnet/anchorxsb.testnet.json",
    ))
    .unwrap();

    let code = std::fs::read(&std::path::Path::new(
        "/Users/xushenbao/project/blockchian/octopus/oct-cli-rs/res/appchain_anchor_v2.0.0.wasm",
    ))
    .unwrap();

    let anchor_contract = AnchorContract {
        account_id: "anchorxsb.testnet".parse().unwrap(),
        client: &client,
    };
    // let outcome = anchor_contract.deploy_and_init(
    //     &signer.into(),
    //     code,
    //     "new".to_string(),
    //     serde_json::json!({
    //         "appchain_id": "appchain_id",
    //     "appchain_registry": "appchain_registry",
    //     "oct_token": "oct",
    //     }).to_string().into_bytes()
    // ).await?;
    //
    // print_transaction_status(outcome, crate::near::types::NearEnv::Testnet);

    Ok(())
}
