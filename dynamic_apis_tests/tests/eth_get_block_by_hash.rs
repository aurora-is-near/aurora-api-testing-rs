use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{get_db_connection, TestRun, TestTask};
use dao::utils::utils::{get_env_var, get_full_db_path};
use jsonrpsee_core::{client::ClientT, Error, JsonRawValue};
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use std::cmp::Ordering;
use std::i64;
use tracing::{info, debug, Level};
use tracing_subscriber::FmtSubscriber;

#[path = "configs.rs"]
mod configs;
use configs::Configs;

#[tokio::test]
async fn test_eth_block_number() -> anyhow::Result<()> {
    let configs = Configs::load().unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber);
    let test_run = TestRun::new(&configs.conn, configs.network, configs.runs_table).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let data_contents: Vec<String> = task
        .get_test_data_content_array("receipt".to_string())
        .unwrap();
    let receipts = TransactionReceipt::load(data_contents).unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    for i in 0..receipts.len() {
        info!("Receipt block hash: {}", receipts[i].block_hash.clone());
        let include_tx_object = true;
        let block_hash = receipts[i].block_hash.clone();
        let params = rpc_params![block_hash, include_tx_object];
        let response: Result<String, _> = client.request("eth_getBlockByHash", params).await;
        debug!("r: {:?}", response);
    }
    Ok(())
}