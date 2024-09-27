use crate::near::constants::ONE_TERA_GAS;
use crate::near::rpc::client::Client;
use crate::oct::contracts::NearContract;
use near_crypto::InMemorySigner;
use near_primitives::types::{AccountId};
use near_primitives::views::FinalExecutionOutcomeView;
use near_sdk::json_types::U128;
use serde_json::json;

pub struct FungibleTokenContract<'s> {
    pub account_id: AccountId,
    pub client: &'s Client,
}

impl<'s> NearContract<'s> for FungibleTokenContract<'s> {
    fn get_account_id(&self) -> &AccountId {
        &self.account_id
    }

    fn get_client(&self) -> &'s Client {
        self.client
    }
}

impl<'s> FungibleTokenContract<'s> {
    pub fn new(account_id: AccountId, client: &'s Client) -> Self {
        return Self { account_id, client };
    }

    pub async fn ft_balance_of(&self, account_id: &AccountId) -> anyhow::Result<U128> {
        self.client
            .view(
                self.account_id.clone(),
                "ft_balance_of".to_string(),
                json!({ "account_id": account_id }).to_string().into_bytes(),
            )
            .await
            .map(|e| e.json().unwrap())
    }

    pub async fn ft_transfer_call(
        &self,
        signer: &InMemorySigner,
        receiver_id: &AccountId,
        amount: &U128,
        msg: &Option<String>,
    ) -> anyhow::Result<FinalExecutionOutcomeView> {
        self.client
            .call(
                signer,
                &self.account_id,
                "ft_transfer_call".to_string(),
                json!({
                    "receiver_id": receiver_id,
                    "amount": amount,
                    "msg": msg
                })
                .to_string()
                .into_bytes(),
                ONE_TERA_GAS * 200,
                1,
            )
            .await
    }
}
