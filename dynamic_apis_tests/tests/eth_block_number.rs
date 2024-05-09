use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use std::cmp::Ordering;
use std::i64;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod configs;
use configs::Configs;

#[tokio::test]
async fn test_eth_block_number() -> anyhow::Result<()> {
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let data_contents: Vec<String> = task
        .get_test_data_content_array("receipt".to_string())
        .unwrap();
    let receipts = TransactionReceipt::load(data_contents).unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    for i in 0..receipts.len() {
        let params = rpc_params![];
        let response: Result<String, _> = client.request("eth_blockNumber", params).await;
        let block_number = response.unwrap();
        let len = block_number.len();
        let live_block_number = i64::from_str_radix(&block_number[2..len], 16).unwrap();
        let receipt_block_number = receipts[i].block_number as i64;
        let result = live_block_number.cmp(&receipt_block_number);
        info!(receipt_block_number, live_block_number);
        assert_eq!(Ordering::Greater, result);
        info!(
            "Live block number: {}, receipt block number: {}",
            live_block_number, receipt_block_number
        );
    }
    Ok(())
}
