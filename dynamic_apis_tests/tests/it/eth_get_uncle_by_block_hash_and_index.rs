use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::configs::Configs;

#[tokio::test]
async fn test_eth_get_uncle_by_block_hash_and_index() -> anyhow::Result<()> {
    let configs = Configs::load().unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let data_contents: Vec<String> = task
        .get_test_data_content_array("receipt".to_string())
        .unwrap();
    let receipts = TransactionReceipt::load(data_contents).unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    for i in 0..receipts.len() {
        info!("block hash: {}", receipts[i].block_hash.to_string());
        let params = rpc_params![receipts[i].block_hash.to_string(), "0x0"];
        let response: Result<Option<String>, _> = client
            .request("eth_getUncleByBlockHashAndIndex", params)
            .await;
        assert_eq!(response.unwrap(), None);
    }
    info!("eth_getUncleByBlockHashAndIndex is not supported in Aurora");
    Ok(())
}
