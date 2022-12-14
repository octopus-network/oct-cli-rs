pub const ONE_TERA_GAS: u64 = 10u64.pow(12);
pub const ONE_GIGA_GAS: u64 = 10u64.pow(9);
pub const ONE_NEAR: u128 = 10u128.pow(24);

// near official rpc

pub const NEAR_OFFICIAL_MAINNET_RPC_URL: &str = "https://rpc.mainnet.near.org";
pub const NEAR_OFFICIAL_TESTNET_RPC_URL: &str = "https://rpc.testnet.near.org";

// block pi rpc

pub const BLOCKPI_MAINNET_RPC_URL: &str = "https://public-rpc.blockpi.io/http/near";
pub const BLOCKPI_TESTNET_RPC_URL: &str = "https://public-rpc.blockpi.io/http/near-testnet";

pub const TESTNET_API_SERVER_URL: &str = "https://public-rpc.blockpi.io/http/near-testnet";
pub const TESTNET_ARCHIVAL_API_SERVER_URL: &str = "https://public-rpc.blockpi.io/http/near-testnet";
pub const MAINNET_API_SERVER_URL: &str = "https://rpc.mainnet.near.org";
pub const MAINNET_ARCHIVAL_API_SERVER_URL: &str = "https://archival-rpc.mainnet.near.org";
pub const BETANET_API_SERVER_URL: &str = "https://rpc.betanet.near.org";
// NOTE: There is no dedicated archival RPC server for betanet by design
pub const BETANET_ARCHIVAL_API_SERVER_URL: &str = "https://rpc.betanet.near.org";

pub const TESTNET_WALLET_URL: &str = "https://wallet.testnet.near.org";
pub const MAINNET_WALLET_URL: &str = "https://wallet.mainnet.near.org";
pub const BETANET_WALLET_URL: &str = "https://wallet.betanet.near.org";

pub const TESTNET_TRANSACTION_URL: &str = "https://explorer.testnet.near.org/transactions/";
pub const MAINNET_TRANSACTION_URL: &str = "https://explorer.mainnet.near.org/transactions/";
pub const BETANET_TRANSACTION_URL: &str = "https://explorer.betanet.near.org/transactions/";

pub const DIR_NAME_KEY_CHAIN: &str = ".near-credentials/default/";
pub const DIR_NAME_TESTNET: &str = ".near-credentials/testnet/";
pub const DIR_NAME_MAINNET: &str = ".near-credentials/mainnet/";
pub const DIR_NAME_BETANET: &str = ".near-credentials/betanet/";
pub const DIR_NAME_CUSTOM: &str = ".near-credentials/default/";
