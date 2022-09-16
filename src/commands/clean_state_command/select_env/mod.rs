use crate::near::types::NearEnv;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

pub mod select_rpc;


#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
///Select NEAR environment
pub enum SelectEnv {
    /// Provide data for the server https://rpc.testnet.near.org
    #[strum_discriminants(strum(message = "Testnet"))]
    Testnet(self::env::Env),
    /// Provide data for the server https://rpc.mainnet.near.org
    #[strum_discriminants(strum(message = "Mainnet"))]
    Mainnet(self::env::Env),
}

impl SelectEnv {
    pub async fn process(self) -> crate::CliResult {
        Ok(match self {
            SelectEnv::Testnet(env) => {env.process(NearEnv::Testnet).await?}
            SelectEnv::Mainnet(env) => {env.process(NearEnv::Mainnet).await?}
        })
    }
}

mod env {
    use super::*;

    #[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
    #[interactive_clap(context = ())]
    pub struct Env {
        #[interactive_clap(named_arg)]
        pub select_rpc: super::select_rpc::SelectRpc
    }

    impl Env {
        pub async fn process(self, connection_config: NearEnv ) -> crate::CliResult {
            self.select_rpc.process(connection_config).await
        }
    }
}