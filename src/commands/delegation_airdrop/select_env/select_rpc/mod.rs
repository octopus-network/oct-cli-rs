mod delegation_airdrop;

use crate::near::rpc::client::Client;
use crate::near::rpc::rpc_provider::RpcProvider;
use crate::near::types::NearEnv;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
///Select NEAR protocol RPC provider
pub enum SelectRpc {
    #[strum_discriminants(strum(message = "Near Official"))]
    Official(self::rpc::Rpc),
    #[strum_discriminants(strum(message = "BlockPi"))]
    BlockPi(self::rpc::Rpc),
}

impl SelectRpc {
    pub async fn process(self, connection_config: NearEnv) -> crate::CliResult {
        match self {
            SelectRpc::Official(rpc) => {
                rpc.process(
                    connection_config.clone(),
                    Client::new(RpcProvider::NearOfficial.get_rpc_by_env(&connection_config)),
                )
                .await
            }
            SelectRpc::BlockPi(rpc) => {
                rpc.process(
                    connection_config.clone(),
                    Client::new(RpcProvider::BlockPi.get_rpc_by_env(&connection_config)),
                )
                .await
            }
        }
    }
}

pub mod rpc {
    use crate::near::rpc::client::Client;
    use crate::near::types::NearEnv;

    #[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
    #[interactive_clap(context = ())]
    pub struct Rpc {
        #[interactive_clap(named_arg)]
        pub delegation_airdrop: super::delegation_airdrop::DelegationAirdrop,
    }

    impl Rpc {
        pub async fn process(self, connection_config: NearEnv, client: Client) -> crate::CliResult {
            self.delegation_airdrop
                .process(connection_config, client)
                .await
        }
    }
}
