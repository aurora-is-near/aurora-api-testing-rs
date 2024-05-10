use dao::dao::models::get_db_connection;
use dao::utils::{
    get_chain_id, get_client_version, get_env_var, get_full_db_path, get_protocol_version,
};
use rusqlite::Connection;

pub struct Configs {
    pub rpc_url: String,
    pub wss_url: String,
    pub network: String,
    pub conn: Connection,
    pub chain_id: String,
    pub client_version: String,
    pub protocol_version: String,
}

impl Configs {
    pub fn load() -> Result<Configs, rusqlite::Error> {
        let rpc_url = get_env_var("RPC_URL").unwrap().to_string();
        let wss_rpc_url = format!("wss://{}", get_env_var("RPC_URL").unwrap());
        let _api_key = get_env_var("AURORA_PLUS_API_KEY").unwrap();
        let url = rpc_url.to_string();
        let full_db_path = get_full_db_path().unwrap();
        let network = get_env_var("NETWORK_NAME").unwrap();
        let chain_id = get_chain_id(&network).unwrap().to_string();
        let client_version = get_client_version(&network).unwrap();
        let protocol_version = get_protocol_version(&network).unwrap().to_string();
        let conn = get_db_connection(&full_db_path).unwrap();
        Ok(Configs {
            rpc_url: url,
            wss_url: wss_rpc_url,
            network,
            conn,
            chain_id,
            client_version,
            protocol_version,
        })
    }
}
