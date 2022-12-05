use near_crypto::InMemorySigner;
use near_primitives::types::AccountId;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::near::constants::{
    BLOCKPI_MAINNET_RPC_URL, BLOCKPI_TESTNET_RPC_URL, DIR_NAME_MAINNET, DIR_NAME_TESTNET,
    MAINNET_ARCHIVAL_API_SERVER_URL, MAINNET_TRANSACTION_URL, MAINNET_WALLET_URL, ONE_NEAR,
    TESTNET_ARCHIVAL_API_SERVER_URL, TESTNET_TRANSACTION_URL, TESTNET_WALLET_URL,
};
use crate::near::rpc::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct NearAccountWithKey {
    pub account_id: AccountId,
    pub public_key: near_crypto::PublicKey,
    pub private_key: near_crypto::SecretKey,
}

impl NearAccountWithKey {
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

impl From<NearAccountWithKey> for InMemorySigner {
    fn from(account: NearAccountWithKey) -> Self {
        InMemorySigner {
            account_id: account.account_id,
            public_key: account.public_key,
            secret_key: account.private_key,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NearEnv {
    Testnet,
    Mainnet,
}

impl NearEnv {
    pub fn rpc_url(&self) -> url::Url {
        match self {
            Self::Testnet => BLOCKPI_TESTNET_RPC_URL.parse().unwrap(),
            Self::Mainnet => BLOCKPI_MAINNET_RPC_URL.parse().unwrap(),
        }
    }

    pub fn init_client(&self) -> Client {
        Client::new(self.rpc_url().as_str())
    }

    pub fn archival_rpc_url(&self) -> url::Url {
        match self {
            Self::Testnet => TESTNET_ARCHIVAL_API_SERVER_URL.parse().unwrap(),
            Self::Mainnet => MAINNET_ARCHIVAL_API_SERVER_URL.parse().unwrap(),
        }
    }

    pub fn wallet_url(&self) -> url::Url {
        match self {
            Self::Testnet => TESTNET_WALLET_URL.parse().unwrap(),
            Self::Mainnet => MAINNET_WALLET_URL.parse().unwrap(),
        }
    }

    pub fn transaction_explorer(&self) -> url::Url {
        match self {
            Self::Testnet => TESTNET_TRANSACTION_URL.parse().unwrap(),
            Self::Mainnet => MAINNET_TRANSACTION_URL.parse().unwrap(),
        }
    }

    pub fn account_dir_name(&self) -> &str {
        match self {
            Self::Testnet => DIR_NAME_TESTNET,
            Self::Mainnet => DIR_NAME_MAINNET,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct NearBalance {
    pub yoctonear_amount: u128,
}

impl NearBalance {
    pub fn from_yoctonear(yoctonear_amount: u128) -> Self {
        Self { yoctonear_amount }
    }

    pub fn to_yoctonear(&self) -> u128 {
        self.yoctonear_amount
    }
}

impl std::fmt::Display for NearBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.yoctonear_amount == 0 {
            write!(f, "0 NEAR")
        } else if self.yoctonear_amount % ONE_NEAR == 0 {
            write!(f, "{} NEAR", self.yoctonear_amount / ONE_NEAR,)
        } else {
            write!(
                f,
                "{}.{} NEAR",
                self.yoctonear_amount / ONE_NEAR,
                format!("{:0>24}", (self.yoctonear_amount % ONE_NEAR)).trim_end_matches('0')
            )
        }
    }
}
