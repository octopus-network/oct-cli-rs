use crate::near::constants::ONE_TERA_GAS;
use crate::near::rpc::client::Client;
use itertools::Itertools;
use near_crypto::InMemorySigner;
use near_primitives::types::AccountId;
use near_primitives::views::FinalExecutionOutcomeView;
use serde_json::json;
use std::ops::Mul;

pub const STATE_CLEANUP_WASM: &[u8] = include_bytes!("../../../res/state_cleanup.wasm");

pub struct CleanStateContract<'s> {
    pub account_id: AccountId,
    pub client: &'s Client,
}

impl<'s> CleanStateContract<'s> {
    pub async fn deploy(
        &self,
        signer: &InMemorySigner,
    ) -> anyhow::Result<FinalExecutionOutcomeView> {
        self.client
            .deploy(
                signer,
                <[u8] as AsRef<[u8]>>::as_ref(STATE_CLEANUP_WASM).into(),
            )
            .await
    }

    pub async fn clean_up_all(
        &self,
        signer: &InMemorySigner,
    ) -> anyhow::Result<FinalExecutionOutcomeView> {
        let result = self
            .client
            .view_state(signer.account_id.clone(), None, None)
            .await?;
        let keys = result.values.iter().map(|e| e.key.clone()).collect_vec();

        println!(
            "Read account state keys before clean up: \n---\n {:?} \n---\n",
            keys
        );

        self.client
            .call(
                signer,
                &self.account_id,
                "clean".to_string(),
                json!({ "keys": keys }).to_string().into_bytes(),
                ONE_TERA_GAS.mul(300),
                0,
            )
            .await
    }
}
