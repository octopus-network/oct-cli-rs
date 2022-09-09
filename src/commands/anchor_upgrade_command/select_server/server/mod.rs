use crate::commands::anchor_upgrade_command::select_server::SelectServerContext;
use crate::near::types::{NearEnv};

mod select_accounts;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ServerContext)]
pub struct Server {
   #[interactive_clap(named_arg)]
   pub select_private: self::select_accounts::SelectAccounts
}

impl Server {
   pub async fn process(self, connection_config: NearEnv) -> crate::CliResult {
      self.select_private.process(connection_config).await
   }
}

#[derive(Clone)]
pub struct ServerContext {
   pub connection_config: NearEnv,
}

impl ServerContext {

}

impl From<SelectServerContext> for ServerContext {
   fn from(item: SelectServerContext) -> Self {
      let connection_config = match item.selected_server {
         super::SelectServerDiscriminants::Testnet => NearEnv::Testnet,
         super::SelectServerDiscriminants::Mainnet => NearEnv::Mainnet,
      };
      Self { connection_config }
   }
}