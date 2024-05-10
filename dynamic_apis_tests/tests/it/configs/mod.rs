use dao::dao::models::get_db_connection;
use dao::utils::utils::{
    get_chain_id, get_client_version, get_env_var, get_full_db_path, get_protocol_version,
};
use rusqlite::Connection;

pub struct Configs {
    pub rpc_url: String,
    pub network: String,
    pub conn: Connection,
    pub chain_id: String,
    pub client_version: String,
    pub protocol_version: String,
}

impl Configs {
    pub fn load() -> Result<Configs, rusqlite::Error> {
        let rpc_url = format!("{}", get_env_var(&"RPC_URL".to_string()).unwrap());
        let _wss_rpc_url = format!("wss://{}", get_env_var(&"RPC_URL".to_string()).unwrap());
        let api_key = get_env_var(&"AURORA_PLUS_API_KEY".to_string()).unwrap();
        let url = format!("{}", rpc_url);
        let full_db_path = get_full_db_path().unwrap();
        let network = get_env_var(&"NETWORK_NAME".to_string()).unwrap();
        let chain_id = get_chain_id(&network).unwrap().to_string();
        let client_version = get_client_version(&network).unwrap();
        let protocol_version = get_protocol_version(&network).unwrap().to_string();
        let conn = get_db_connection(&full_db_path).unwrap();
        Ok(Configs {
            rpc_url: url,
            network,
            conn,
            chain_id,
            client_version,
            protocol_version,
        })
    }
}
