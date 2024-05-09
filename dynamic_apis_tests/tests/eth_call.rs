use dao::dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use serde::{Deserialize, Serialize};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod configs;
use configs::Configs;

#[derive(Serialize, Deserialize, Debug)]
struct MessageCallParams {
    from: String,
    to: String,
}

impl MessageCallParams {
    pub fn new(from: String, to: String) -> MessageCallParams {
        MessageCallParams { from, to }
    }
}

#[tokio::test]
async fn test_eth_call() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let from_address_content: String = task
        .get_test_data_content_by_group_index(0, "deployer_address".to_string())
        .unwrap();

    let to_address_content: String = task
        .get_test_data_content_by_group_index(0, "destination_address".to_string())
        .unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url.clone())?;
    let msg = MessageCallParams::new(from_address_content.clone(), to_address_content.clone());
    let params = rpc_params![msg, "latest".to_string()];
    // info!("msg: {:?}",params);
    let response: Result<String, _> = client.request("eth_call", params).await;
    info!("response: {:?}", response.unwrap());
    Ok(())
}

#[tokio::test]
async fn test_eth_call_eoa() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    info!("calling precompile SHA256 from EOA");
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let to_address_content: String = task
        .get_test_data_content_by_group_index(0, "destination_address".to_string())
        .unwrap();
    let sha256_precompile_addr = "0x0000000000000000000000000000000000000002".to_string();
    let msg = MessageCallParams::new(to_address_content.clone(), sha256_precompile_addr);
    let params = rpc_params![msg, "latest".to_string()];
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let response: Result<String, _> = client.request("eth_call", params).await;
    info!(
        "SHA256 Precompile contract response: {:?}",
        response.unwrap()
    );
    Ok(())
}
