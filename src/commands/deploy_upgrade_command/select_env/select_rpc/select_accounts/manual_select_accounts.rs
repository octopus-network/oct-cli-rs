use std::collections::HashMap;
use dialoguer::Input;
use crate::CliResult;
use crate::near::rpc::client::Client;
use crate::near::types::{NearAccountWithKey, NearEnv};
use crate::near::util::{get_accounts_from_path, get_default_near_account_dir_path};

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct ManualSelectAccounts {
    #[interactive_clap(long)]
    pub account_ids: String,
    #[interactive_clap(named_arg)]
    pub upgrade: super::upgrade::Upgrade
}

impl ManualSelectAccounts {

    pub fn input_account_ids(
        _context: &()
    )-> color_eyre::eyre::Result<String> {
        Ok(Input::new()
            .with_prompt("Enter account list split by ','. eg: a.testnet,b.testnet,c.testnet")
            .interact_text()?)
    }

    pub async fn process(self, connection_config: NearEnv, client: Client) -> CliResult {

        let buf = get_default_near_account_dir_path(&connection_config);
        let mut all_accounts: HashMap<String, NearAccountWithKey> = get_accounts_from_path(buf.as_path())?
            .into_iter()
            .map(|e| (e.account_id.to_string().clone(), e))
            .collect();

        let mut selected_account = vec![];
        for account in self.account_ids.split(",") {
            if !all_accounts.contains_key(account) {
                panic!("Failed to find account {} in your system.", account);
            }
            selected_account.push(all_accounts.remove(account).unwrap().into());
        }
        self.upgrade.process(connection_config,selected_account, client).await

    }

}