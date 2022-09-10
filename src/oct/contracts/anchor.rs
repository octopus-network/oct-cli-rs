
use near_crypto::InMemorySigner;
use near_primitives::types::AccountId;
use near_primitives::views::FinalExecutionOutcomeView;

use crate::near::rpc::client::Client;

pub struct AnchorContract<'s> {
    pub account_id: AccountId,
    pub client: &'s Client
}

impl<'s> AnchorContract<'s> {
    
    pub async fn deploy(
        &self,
        signer: &InMemorySigner,
        wasm: Vec<u8>,
    ) ->anyhow::Result<FinalExecutionOutcomeView>{
        self.client.deploy(signer, wasm).await
    }

    pub async fn deploy_and_init(
        &self,
        signer: &InMemorySigner,
        wasm: Vec<u8>,
        method_name: String,
        args: Vec<u8>
    ) ->anyhow::Result<FinalExecutionOutcomeView>{
        self.client.deploy_and_init(signer, wasm, method_name, args).await
    }

}

#[tokio::test]
pub async fn deploy_test() -> anyhow::Result<()> {

    use crate::near::util::print_transaction_status;

    let client = Client::new("https://public-rpc.blockpi.io/http/near-testnet");

    let signer = crate::near::types::NearAccountWithKey::from_file(&std::path::Path::new("/Users/xushenbao/.near-credentials/testnet/anchorxsb.testnet.json")).unwrap();

    let code = std::fs::read(&std::path::Path::new("/Users/xushenbao/project/blockchian/octopus/oct-cli-rs/res/appchain_anchor_v2.0.0.wasm")).unwrap();

    let anchor_contract = AnchorContract {
        account_id: "anchorxsb.testnet".parse().unwrap(),
        client: &client
    };
    let outcome = anchor_contract.deploy_and_init(
        &signer.into(),
        code,
        "new".to_string(),
        serde_json::json!({
            "appchain_id": "appchain_id",
        "appchain_registry": "appchain_registry",
        "oct_token": "oct",
        }).to_string().into_bytes()
    ).await?;

    print_transaction_status(outcome, crate::near::types::NearEnv::Testnet);

    Ok(())

}