use crate::near::rpc::client::Client;
use crate::near::types::NearEnv;
use crate::near::util::print_transaction_status;
use crate::oct::contracts::anchor::AnchorContract;
use crate::CliResult;
use dialoguer::Input;
use std::path::Path;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
// #[interactive_clap(context = super::super::super::ServerContext)]
#[interactive_clap(context = ())]
pub struct Upgrade {
    pub wasm_path: String,
    pub migrate_method_name: String,
    pub args: String,
}

impl Upgrade {
    pub fn input_wasm_path(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok(Input::new()
            .with_prompt("What is the new wasm path?")
            .interact_text()?)
    }

    pub fn input_migrate_method_name(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok(Input::new()
            .with_prompt("What is the migrate method name?")
            .interact_text()?)
    }

    pub fn input_args(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok(Input::new()
            .with_prompt("Enter args for function?")
            .interact_text()?)
    }

    pub async fn process(
        self,
        connection_config: NearEnv,
        account_list: Vec<near_crypto::InMemorySigner>,
        client: Client,
    ) -> CliResult {
        let code =
            std::fs::read(&Path::new(self.wasm_path.as_str())).expect("Failed to read wasm file");

        let client = connection_config.init_client();

        for signer in account_list {
            println!(
                "---Start {} deploy, wasm is {} , migrate method {}, args: {}",
                signer.account_id, self.wasm_path, self.migrate_method_name, self.args
            );
            let outcome = client
                .deploy_and_init(
                    &signer,
                    code.clone(),
                    self.migrate_method_name.clone(),
                    self.args.clone().into_bytes(),
                )
                .await
                .map_err(|err| {
                    color_eyre::Report::msg(format!(
                        "Failed to deploy anchor with {}, error: {}",
                        signer.account_id, err
                    ))
                })?;
            print_transaction_status(outcome, connection_config.clone());
            println!("---End {} deploy\n", signer.account_id);
        }
        Ok(())
    }
}
