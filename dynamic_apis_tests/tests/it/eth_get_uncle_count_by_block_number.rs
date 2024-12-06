use dao::helpers::TransactionReceipt;
use dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

use crate::utils::hex_string_to_i32;

#[tokio::test]
async fn test_eth_get_uncle_count_by_block_number() -> anyhow::Result<()> {
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
        let params = rpc_params![receipts[i].block_number.to_string()];
        let response: Result<String, _> = client
            .request("eth_getUncleCountByBlockNumber", params)
            .await;
        let uncle_count = response.unwrap();
        assert_eq!(hex_string_to_i32(uncle_count.clone()), 0);
        info!("eth_getUncleCountByBlockNumber: {:?}", uncle_count.clone());
    }
    Ok(())
}
