use crate::near::contracts::clean_state::CleanStateContract;
use crate::near::rpc::client::Client;
use crate::near::types::NearEnv;
use crate::CliResult;
use dialoguer::Input;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct CleanState {
    confirm: String,
}

impl CleanState {
    fn input_confirm(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok(Input::new()
            .with_prompt("Please confirm you want clean state?[y/n]")
            .interact_text()?)
    }

    pub async fn process(
        self,
        connection_config: NearEnv,
        account_list: Vec<near_crypto::InMemorySigner>,
        client: Client,
    ) -> CliResult {
        if self.confirm.eq("y") || self.confirm.eq("Y") {
            for account in account_list {
                println!("\n---Start clean {} states", account.account_id);

                let clean_contract = CleanStateContract {
                    account_id: account.account_id.clone(),
                    client: &client,
                };

                clean_contract.deploy(&account).await.map_err(|error| {
                    color_eyre::Report::msg(format!(
                        "Failed to deploy clean state contract for account({}), error: {}",
                        account.account_id, error
                    ))
                })?;

                clean_contract
                    .clean_up_all(&account)
                    .await
                    .map_err(|error| {
                        color_eyre::Report::msg(format!(
                            "Failed to clean account({}) states, error: {}",
                            account.account_id, error
                        ))
                    })?;

                let result = client
                    .view_state(account.account_id.clone(), None, None)
                    .await
                    .map_err(|error| {
                        color_eyre::Report::msg(format!(
                            "Failed to view account({}) states, error: {}",
                            account.account_id, error
                        ))
                    })?;
                println!("Show account state after clean up: {:?}", result);

                println!("---End clean {} states\n", account.account_id);
            }
        } else {
            println!("Cancel clean state!");
        }

        Ok(())
    }
}
