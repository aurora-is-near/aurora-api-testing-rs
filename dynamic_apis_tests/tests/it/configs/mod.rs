use dao::models::get_db_connection;
use dao::utils::{
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
        let rpc_url = match get_env_var("RPC_URL") {
            Some(value) => value,
            None => panic!("Environment variable RPC_URL is not set"),
        };
        let api_key = match get_env_var("AURORA_PLUS_API_KEY") {
            Some(value) => value,
            None => "".to_owned(),
        };
        let network = match get_env_var("NETWORK_NAME") {
            Some(value) => value,
            None => panic!("Environment variable NETWORK_NAME is not set"),
        };
        let url = format!("{}{}", rpc_url, api_key);
        let full_db_path = get_full_db_path().unwrap();
        let chain_id = get_chain_id(&network).unwrap().to_string();
        let client_version = get_client_version(&network).unwrap();
        let protocol_version = get_protocol_version(&network).unwrap().to_string();
        let conn = get_db_connection(&full_db_path)?;
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
