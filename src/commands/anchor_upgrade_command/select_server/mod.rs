use strum::{EnumDiscriminants, EnumIter, EnumMessage};
use crate::near::types::{NearEnv};

pub(crate) mod server;

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = SelectServerContext)]
///Select NEAR protocol RPC server
pub enum SelectServer {
    /// Provide data for the server https://rpc.testnet.near.org
    #[strum_discriminants(strum(message = "Testnet"))]
    Testnet(self::server::Server),
    /// Provide data for the server https://rpc.mainnet.near.org
    #[strum_discriminants(strum(message = "Mainnet"))]
    Mainnet(self::server::Server),
}

impl SelectServer {
    pub async fn process(self) -> crate::CliResult {
        Ok(match self {
            SelectServer::Testnet(server) => {server.process(NearEnv::Testnet).await?}
            SelectServer::Mainnet(server) => {server.process(NearEnv::Mainnet).await?}
        })
    }
}


#[derive(Clone)]
pub struct SelectServerContext {
    selected_server: SelectServerDiscriminants,
}

impl SelectServerContext {
    fn from_previous_context(
        _previous_context: (),
        scope: &<SelectServer as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            selected_server: scope.clone(),
        }
    }
}