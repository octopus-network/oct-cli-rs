use crate::near::constants::{BLOCKPI_MAINNET_RPC_URL, BLOCKPI_TESTNET_RPC_URL, NEAR_OFFICIAL_MAINNET_RPC_URL, NEAR_OFFICIAL_TESTNET_RPC_URL};
use crate::near::types::NearEnv;

pub enum RpcProvider {
    NearOfficial,
    BlockPi,
}

impl RpcProvider {

    pub fn get_rpc_by_env(&self, env: &NearEnv) -> &str {
        match self {
            RpcProvider::NearOfficial => {
                match env {
                    NearEnv::Testnet => { NEAR_OFFICIAL_TESTNET_RPC_URL}
                    NearEnv::Mainnet => { NEAR_OFFICIAL_MAINNET_RPC_URL}
                }

            }
            RpcProvider::BlockPi => {
                match env {
                    NearEnv::Testnet => {BLOCKPI_TESTNET_RPC_URL}
                    NearEnv::Mainnet => {BLOCKPI_MAINNET_RPC_URL}
                }
            }
        }
    }
}

