use std::ops::Mul;
use itertools::Itertools;
use near_crypto::InMemorySigner;
use near_primitives::types::AccountId;
use near_primitives::views::FinalExecutionOutcomeView;
use serde_json::json;
use crate::near::constants::ONE_TERA_GAS;
use crate::near::rpc::client::Client;

pub const STATE_CLEANUP_WASM: &[u8] = include_bytes!("../../../res/state_cleanup.wasm");

pub struct CleanStateContract<'s> {
    pub account_id: AccountId,
    pub client: &'s Client
}

impl<'s> CleanStateContract<'s> {
    pub async fn deploy(
        &self,
        signer: &InMemorySigner,
    ) ->anyhow::Result<FinalExecutionOutcomeView>{
        self.client.deploy(signer, STATE_CLEANUP_WASM.as_ref().into()).await
    }

    pub async fn clean_up_all(
        &self,
        signer: &InMemorySigner,
    )-> anyhow::Result<FinalExecutionOutcomeView> {

        let result = self.client.view_state(signer.account_id.clone(), None, None).await?;
        let keys = result.values.iter().map(|e| e.key.clone()).collect_vec();

        println!("Read account state before clean up: {:?}", result);

        self.client.call(
            signer,
            &self.account_id,
            "clean".to_string(),
            json!({
                "keys": keys
            }).to_string().into_bytes(),
            ONE_TERA_GAS.mul(300),
            0
        ).await
    }
}