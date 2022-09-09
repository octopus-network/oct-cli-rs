pub mod account;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};
use crate::near::types::{NearEnv};

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = super::ServerContext)]
/// Select accounts to execute upgrade
pub enum SelectAccounts {
    #[strum_discriminants(strum(message = "Input accounts need to upgrade(eg: a.testnet,b.testnet,c.testnet),make sure you have been login these accounts in this system"))]
    ManualSelectAccounts(self::account::manual_select_accounts::ManualSelectAccounts),
    /// Provide data for the server https://rpc.mainnet.near.org
    #[strum_discriminants(strum(message = "Select all accounts located in ~/.near-credentials"))]
    DefaultDirectory(self::account::default_directory::DefaultDirectory),
    #[strum_discriminants(strum(message = "Select all accounts located in custom directory."))]
    CustomDirectory(self::account::custom_directory::CustomDirectory),
}

impl SelectAccounts {
    pub async fn process(self, connection_config: NearEnv) -> crate::CliResult {
        Ok(match self {
            SelectAccounts::ManualSelectAccounts(single_account) => { single_account.process(connection_config).await?}
            SelectAccounts::DefaultDirectory(default_directory) => {default_directory.process(connection_config).await?}
            SelectAccounts::CustomDirectory(custom_directory) => {custom_directory.process(connection_config).await?}
        })
    }
}
