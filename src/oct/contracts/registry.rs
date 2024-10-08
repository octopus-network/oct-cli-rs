use crate::near::rpc::client::Client;
use appchain_registry::types::AppchainStatus;
use near_primitives::types::AccountId;
use crate::oct::contracts::NearContract;
use serde_json::json;

pub struct RegistryContract<'s> {
    pub account_id: AccountId,
    pub client: &'s Client,
}

impl<'s> NearContract<'s> for RegistryContract<'s> {
    fn get_account_id(&self) -> &AccountId {
        &self.account_id
    }

    fn get_client(&self) -> &'s Client {
        self.client
    }
}

impl<'s> RegistryContract<'s> {
    pub fn new(account_id: AccountId, client: &'s Client) -> Self {
        return Self { account_id, client };
    }

    pub async fn get_appchain_ids(&self) -> anyhow::Result<Vec<String>> {
        self.client
            .view(
                self.account_id.clone(),
                "get_appchain_ids".to_string(),
                json!({}).to_string().into_bytes(),
            )
            .await
            .map(|e| e.json().unwrap())
    }

    pub async fn get_appchain_status_of(
        &self,
        appchain_id: String,
    ) -> anyhow::Result<AppchainStatus> {
        self.client
            .view(
                self.account_id.clone(),
                "get_appchain_status_of".to_string(),
                json!({ "appchain_id": appchain_id })
                    .to_string()
                    .into_bytes(),
            )
            .await
            .map(|e| e.json().unwrap())
    }
}
