
use near_crypto::InMemorySigner;
use near_primitives::types::AccountId;
use near_primitives::views::FinalExecutionOutcomeView;
use serde_json::json;

use crate::near::rpc::client::Client;
use crate::oct::contracts::anchor::types::{AnchorStatus, AppchainState, AppchainValidator, ProtocolSettings, RewardHistory, ValidatorSetInfo};
use crate::oct::contracts::NearContract;

pub struct AnchorContract<'s> {
    pub account_id: AccountId,
    pub client: &'s Client
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
    pub fn new(
        account_id: AccountId,
        client: &'s Client
    ) -> Self {
        return Self {
            account_id,
            client
        }
    }

    pub async fn get_anchor_status(&self) -> anyhow::Result<AnchorStatus> {
        self.client.view(
            self.account_id.clone(),
            "get_anchor_status".to_string(),
            json!({}).to_string().into_bytes()).await.map(|e| e.json().unwrap())
    }

    pub async fn get_validator_rewards_of(
        &self,
        start_era: u64,
        end_era: u64,
        validator_id: AccountId,
    ) -> anyhow::Result<Vec<RewardHistory>> {
        self.client.view(
            self.account_id.clone(),
            "get_validator_rewards_of".to_string(),
            json!({
                "start_era": start_era.to_string(),
                "end_era": end_era.to_string(),
                "validator_id": validator_id
            }).to_string().into_bytes()).await.map(|e| e.json().unwrap())
    }

    pub async fn get_validator_list_of(&self, era_number: Option<u64>) -> anyhow::Result<Vec<AppchainValidator>> {
        self.client.view(
            self.account_id.clone(),
            "get_validator_list_of".to_string(),
            json!({
                "era_number": era_number,
            }).to_string().into_bytes()).await.map(|e| e.json().unwrap())
    }

    pub async fn get_protocol_settings(&self) -> anyhow::Result<ProtocolSettings> {
        self.client.view(
            self.account_id.clone(),
            "get_protocol_settings".to_string(),
            json!({}).to_string().into_bytes()).await.map(|e| e.json().unwrap())
    }

    pub async fn get_validator_set_info_of(&self, era_number: u64) -> anyhow::Result<Option<ValidatorSetInfo>> {
        self.client.view(
            self.account_id.clone(),
            "get_validator_set_info_of".to_string(),
            json!({
                "era_number": era_number.to_string()
            }).to_string().into_bytes()).await.map(|e| e.json().unwrap())
    }

    pub async fn get_appchain_state(&self) -> anyhow::Result<AppchainState> {
        self.client.view(
            self.account_id.clone(),
            "get_appchain_state".to_string(),
            json!({}).to_string().into_bytes()).await.map(|e| e.json().unwrap())    }

}

#[tokio::test]
pub async fn test_get_validator_set_info_of() -> anyhow::Result<()> {
    let client = Client::new("https://public-rpc.blockpi.io/http/near");

    let anchor_contract = AnchorContract {
        account_id: "fusotao.octopus-registry.near".parse().unwrap(),
        client: &client
    };
    let validator_set_info = anchor_contract.get_validator_set_info_of(51).await.unwrap();

    dbg!(&validator_set_info);

    Ok(())
}
#[tokio::test]
pub async fn test_get_protocol_setting() -> anyhow::Result<()> {
    let client = Client::new("https://public-rpc.blockpi.io/http/near");

    let anchor_contract = AnchorContract {
        account_id: "fusotao.octopus-registry.near".parse().unwrap(),
        client: &client
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

    dbg!(&protocol_setting);
    Ok(())
}

#[tokio::test]
pub async fn test_get_anchor_status() -> anyhow::Result<()> {
    let client = Client::new("https://public-rpc.blockpi.io/http/near");

    let anchor_contract = AnchorContract {
        account_id: "fusotao.octopus-registry.near".parse().unwrap(),
        client: &client
    };
    let status = anchor_contract.get_anchor_status().await.unwrap();
    dbg!(&status);
    Ok(())
}

#[tokio::test]
pub async fn test_get_validator_rewards_of() -> anyhow::Result<()> {
    let client = Client::new("https://public-rpc.blockpi.io/http/near");

    let anchor_contract = AnchorContract {
        account_id: "fusotao.octopus-registry.near".parse().unwrap(),
        client: &client
    };
    let rewards = anchor_contract.get_validator_rewards_of(10, 50, "manhmai11.near".parse()?).await.unwrap();
    dbg!(rewards);
    Ok(())
}

#[tokio::test]
pub async fn test_deploy() -> anyhow::Result<()> {

    use crate::near::util::print_transaction_status;

    let client = Client::new("https://public-rpc.blockpi.io/http/near-testnet");

    let signer = crate::near::types::NearAccountWithKey::from_file(&std::path::Path::new("/Users/xushenbao/.near-credentials/testnet/anchorxsb.testnet.json")).unwrap();

    let code = std::fs::read(&std::path::Path::new("/Users/xushenbao/project/blockchian/octopus/oct-cli-rs/res/appchain_anchor_v2.0.0.wasm")).unwrap();

    let anchor_contract = AnchorContract {
        account_id: "anchorxsb.testnet".parse().unwrap(),
        client: &client
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

pub mod types {
    use near_primitives::types::AccountId;
    use crate::*;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct AnchorStatus {
        #[serde(with = "u128_dec_format")]
        pub total_stake_in_next_era: u128,
        #[serde(with = "u64_dec_format")]
        pub validator_count_in_next_era: u64,
        #[serde(with = "u64_dec_format")]
        pub delegator_count_in_next_era: u64,
        pub index_range_of_appchain_notification_history: IndexRange,
        pub index_range_of_validator_set_history: IndexRange,
        pub index_range_of_staking_history: IndexRange,
        pub nonce_range_of_appchain_messages: IndexRange,
        pub index_range_of_appchain_challenges: IndexRange,
        pub permissionless_actions_status: PermissionlessActionsStatus,
        pub asset_transfer_is_paused: bool,
        pub rewards_withdrawal_is_paused: bool,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct IndexRange {
        #[serde(with = "u64_dec_format")]
        pub start_index: u64,
        #[serde(with = "u64_dec_format")]
        pub end_index: u64,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct RewardHistory {
        #[serde(with = "u64_dec_format")]
        pub era_number: u64,
        #[serde(with = "u128_dec_format")]
        pub total_reward: u128,
        #[serde(with = "u128_dec_format")]
        pub unwithdrawn_reward: u128,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct PermissionlessActionsStatus {
        /// The era number that is switching by permissionless actions
        pub switching_era_number: Option<String>,
        /// The era number that is distributing reward by permissionless actions
        pub distributing_reward_era_number: Option<String>,
        ///
        pub processing_appchain_message_nonce: Option<u32>,
        ///
        pub max_nonce_of_staged_appchain_messages: u32,
        ///
        pub latest_applied_appchain_message_nonce: u32,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct AppchainValidator {
        pub validator_id: AccountId,
        pub validator_id_in_appchain: String,
        #[serde(with = "u128_dec_format")]
        pub deposit_amount: u128,
        #[serde(with = "u128_dec_format")]
        pub total_stake: u128,
        #[serde(with = "u64_dec_format")]
        pub delegators_count: u64,
        pub can_be_delegated_to: bool,
        pub is_unbonding: bool,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ProtocolSettings {
        /// A validator has to deposit a certain amount of OCT token to this contract for
        /// being validator of the appchain.
        #[serde(with = "u128_dec_format")]
        pub minimum_validator_deposit: u128,
        /// The minimum amount for a validator to increase or decrease his/her deposit.
        #[serde(with = "u128_dec_format")]
        pub minimum_validator_deposit_changing_amount: u128,
        /// The maximum percent value that the deposit of a validator in total stake
        pub maximum_validator_stake_percent: u16,
        /// The minimum deposit amount for a delegator to delegate his voting weight to
        /// a certain validator.
        #[serde(with = "u128_dec_format")]
        pub minimum_delegator_deposit: u128,
        /// The minimum amount for a delegator to increase or decrease his/her delegation
        /// to a validator.
        #[serde(with = "u128_dec_format")]
        pub minimum_delegator_deposit_changing_amount: u128,
        /// The minimum price (in USD) of total stake in this contract for
        /// booting corresponding appchain
        #[serde(with = "u128_dec_format")]
        pub minimum_total_stake_price_for_booting: u128,
        /// The maximum percentage of the total market value of all NEP-141 tokens to the total
        /// market value of OCT token staked in this contract
        pub maximum_market_value_percent_of_near_fungible_tokens: u16,
        /// The maximum percentage of the total market value of wrapped appchain token to the total
        /// market value of OCT token staked in this contract
        pub maximum_market_value_percent_of_wrapped_appchain_token: u16,
        /// The minimum number of validator(s) registered in this contract for
        /// booting the corresponding appchain and keep it alive.
        #[serde(with = "u64_dec_format")]
        pub minimum_validator_count: u64,
        /// The maximum number of validator(s) registered in this contract for
        /// the corresponding appchain.
        #[serde(with = "u64_dec_format")]
        pub maximum_validator_count: u64,
        /// The maximum number of validator(s) which a delegator can delegate to.
        #[serde(with = "u64_dec_format")]
        pub maximum_validators_per_delegator: u64,
        /// The unlock period (in days) for validator(s) can withdraw their deposit after
        /// they are removed from the corresponding appchain.
        #[serde(with = "u64_dec_format")]
        pub unlock_period_of_validator_deposit: u64,
        /// The unlock period (in days) for delegator(s) can withdraw their deposit after
        /// they no longer delegates their stake to a certain validator on the corresponding appchain.
        #[serde(with = "u64_dec_format")]
        pub unlock_period_of_delegator_deposit: u64,
        /// The maximum number of historical eras that the validators or delegators are allowed to
        /// withdraw their reward
        #[serde(with = "u64_dec_format")]
        pub maximum_era_count_of_unwithdrawn_reward: u64,
        /// The maximum number of valid appchain message.
        /// If the era number of appchain message is smaller than the latest era number minus
        /// this value, the message will be considered as `invalid`.
        #[serde(with = "u64_dec_format")]
        pub maximum_era_count_of_valid_appchain_message: u64,
        /// The percent of commission fees of a validator's reward in an era
        pub validator_commission_percent: u16,
        /// The maximum unprofitable era count for auto-unbonding a validator
        pub maximum_allowed_unprofitable_era_count: u16,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ValidatorSetInfo {
        /// The number of era in appchain.
        #[serde(with = "u64_dec_format")]
        pub era_number: u64,
        /// Total stake of current set
        #[serde(with = "u128_dec_format")]
        pub total_stake: u128,
        /// The validator list for query
        pub validator_list: Vec<AppchainValidator>,
        /// The block height when the era starts.
        #[serde(with = "u64_dec_format")]
        pub start_block_height: u64,
        /// The timestamp when the era starts.
        #[serde(with = "u64_dec_format")]
        pub start_timestamp: u64,
        /// The index of the latest staking history happened in the era of corresponding appchain.
        #[serde(with = "u64_dec_format")]
        pub staking_history_index: u64,
        /// The set of validator id which will not be profited.
        pub unprofitable_validator_ids: Vec<AccountId>,
        /// Total stake excluding all unprofitable validators' stake.
        #[serde(with = "u128_dec_format")]
        pub valid_total_stake: u128,
        // pub processing_status: ValidatorSetProcessingStatus,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub enum AppchainState {
        Registered,
        Auditing,
        InQueue,
        Staging,
        Booting,
        Active,
        Frozen,
        Broken,
        Dead,
    }

}