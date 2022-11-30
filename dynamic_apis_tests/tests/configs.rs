use dao::dao::models::get_db_connection;
use dao::utils::utils::{get_env_var, get_full_db_path};
use rusqlite::Connection;

pub struct Configs {
    pub rpc_url: String,
    pub wss_url: String,
    pub network: String,
    pub conn: Connection,
    pub runs_table: String,
}

impl Configs {
    pub fn load() -> Result<Configs, rusqlite::Error> {
        let rpc_url = format!("https://{}", get_env_var(&"RPC_URL".to_string()).unwrap());
        let wss_rpc_url = format!("wss://{}", get_env_var(&"RPC_URL".to_string()).unwrap());
        let api_key = get_env_var(&"API_KEY".to_string()).unwrap();
        let url = format!("{}{}", rpc_url, api_key);
        let full_db_path = get_full_db_path().unwrap();
        Ok(Configs {
            rpc_url: url,
            wss_url: wss_rpc_url,
            network: get_env_var(&"NETWORK_NAME".to_string())
                .unwrap_or("mainnet_aurora_plus".to_string()),
            conn: get_db_connection(&full_db_path).unwrap(),
            runs_table: get_env_var(&"RUNS_TABLE".to_string())
                .unwrap_or("aurora_relayer_test_runs".to_string()),
        })
    }
}
