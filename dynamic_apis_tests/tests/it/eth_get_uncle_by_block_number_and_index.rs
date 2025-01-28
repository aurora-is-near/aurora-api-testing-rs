use dao::helpers::TransactionReceipt;
use dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[allow(clippy::needless_range_loop)]
#[tokio::test]
async fn test_eth_get_uncle_by_block_number_and_index() -> anyhow::Result<()> {
    let _guard = init();
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
    for i in 0..receipts.len() {
        info!("block number: {}", receipts[i].block_number);
        let params = rpc_params![receipts[i].block_number.to_string(), "0x0"];
        let response: Result<Option<String>, _> = client
            .request("eth_getUncleByBlockNumberAndIndex", params)
            .await;
        assert_eq!(response.unwrap(), None);
    }
    Ok(())
}
