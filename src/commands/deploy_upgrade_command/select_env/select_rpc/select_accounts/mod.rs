use crate::near::rpc::client::Client;
use crate::near::types::NearEnv;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

pub mod custom_directory;
pub mod default_directory;
pub mod manual_select_accounts;
pub mod upgrade;

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
/// Select accounts to continue
pub enum SelectAccounts {
    #[strum_discriminants(strum(message = "Input accounts(eg: a.testnet,b.testnet,c.testnet),make sure you have been login these accounts in this system"))]
    ManualSelectAccounts(self::manual_select_accounts::ManualSelectAccounts),
    /// Provide data for the server https://rpc.mainnet.near.org
    #[strum_discriminants(strum(message = "Select all accounts located in ~/.near-credentials"))]
    DefaultDirectory(self::default_directory::DefaultDirectory),
    #[strum_discriminants(strum(message = "Select all accounts located in custom directory."))]
    CustomDirectory(self::custom_directory::CustomDirectory),
}

impl SelectAccounts {
    pub async fn process(self, connection_config: NearEnv, client: Client) -> crate::CliResult {
        match self {
            SelectAccounts::ManualSelectAccounts(manual_select_accounts) => {manual_select_accounts.process(connection_config,client).await}
            SelectAccounts::DefaultDirectory(default_directory) => {default_directory.process(connection_config, client).await}
            SelectAccounts::CustomDirectory(custom_directory) => {custom_directory.process(connection_config, client).await}
        }
    }
}