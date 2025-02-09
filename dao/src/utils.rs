pub use dotenv::dotenv;
pub use std::collections::HashMap;
pub use std::{env, path::Path, path::PathBuf};

#[allow(clippy::unreadable_literal)]
const MAINNET_AURORA_CHAIN_ID: i64 = 1313161554;
#[allow(clippy::unreadable_literal)]
const TESTNET_AURORA_CHAIN_ID: i64 = 1313161555;
#[allow(clippy::unreadable_literal)]
const SEPOLIA_CHAIN_ID: i64 = 11155111;
const ROPSTEN_CHAIN_ID: i64 = 3;
const GOERLI_CHAIN_ID: i64 = 5;

#[must_use]
pub fn load_env_file() -> HashMap<String, String> {
    dotenv().ok();
    let env_vars: HashMap<String, String> = env::vars()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    env_vars
}

#[must_use]
pub fn get_env_var(var: &str) -> Option<String> {
    let vars = load_env_file();
    Some(vars.get(var)?.to_string())
}

#[must_use]
pub fn get_full_db_path() -> Option<PathBuf> {
    let db_dir = "../relayer-test-data-generator/db/test-data.sqlite3".to_string();
    Some(Path::join(
        env::current_dir().unwrap().as_path(),
        Path::new(&db_dir).to_str().unwrap(),
    ))
}

#[must_use]
pub fn get_chain_id(network_name: &str) -> Option<i64> {
    match network_name {
        "sepolia" => Some(SEPOLIA_CHAIN_ID),
        "ropsten" => Some(ROPSTEN_CHAIN_ID),
        "goerli" => Some(GOERLI_CHAIN_ID),
        "testnet_aurora_plus" | "testnet_aurora" => Some(TESTNET_AURORA_CHAIN_ID),
        "mainnet_aurora_plus"
        | "wss_mainnet_aurora_plus"
        | "mainnet_aurora_plus_rpc_url"
        | "new_mainnet_aurora_plus"
        | "mainnet_aurora" => Some(MAINNET_AURORA_CHAIN_ID),
        _ => Some(0),
    }
}

#[must_use]
pub fn get_client_version(network_name: &str) -> Option<String> {
    static AURORA_WEB3_CLIENT_VERSION: &str = "Aurora";
    match network_name {
        "ropsten" => Some("erigon/2022.99.99/linux-amd64/go1.18.3".to_string()), // https://rpc.ankr.com/eth_ropsten
        "goerli" => Some("Geth/v1.10.23-omnibus-b38477ec/linux-amd64/go1.18.5".to_string()), // infura
        "sepolia"
        | "testnet_aurora_plus"
        | "testnet_aurora"
        | "mainnet_aurora_plus"
        | "wss_mainnet_aurora_plus"
        | "mainnet_aurora_plus_rpc_url"
        | "new_mainnet_aurora_plus"
        | "mainnet_aurora" => Some(AURORA_WEB3_CLIENT_VERSION.to_string()),
        _ => Some(String::new()),
    }
}

#[must_use]
pub fn get_protocol_version(network: &str) -> Option<i32> {
    match network {
        "ropsten" => Some(0), // this one is unknown!
        "goerli"
        | "sepolia"
        | "testnet_aurora_plus"
        | "testnet_aurora"
        | "mainnet_aurora_plus"
        | "wss_mainnet_aurora_plus"
        | "mainnet_aurora_plus_rpc_url"
        | "new_mainnet_aurora_plus"
        | "mainnet_aurora" => Some(65), // 0x41
        _ => None,
    }
}
