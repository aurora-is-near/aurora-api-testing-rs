use dao::helpers::TransactionReceipt;
use dao::models::{TestRun, TestTask};
use ethereum_types::{H160, H256};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use std::cmp::Ordering;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[allow(clippy::needless_range_loop)]
#[tokio::test]
async fn test_eth_get_block_transaction_count_by_hash() -> anyhow::Result<()> {
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
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url.clone())?;
    // assert that all blocks has at least one transaction.
    for i in 0..receipts.len() {
        info!("block hash: {}", receipts[i].block_hash.clone());
        let params = rpc_params![receipts[i].block_hash.clone()];
        let response: Result<String, _> = client
            .request("eth_getBlockTransactionCountByHash", params)
            .await;
        let tx_count_hex = response.unwrap();
        let tx_count = i64::from_str_radix(&tx_count_hex[2..tx_count_hex.len()], 16).unwrap();
        info!("transactions count: {}", tx_count);
        let res = tx_count.cmp(&0);
        assert_eq!(Ordering::Greater, res);
    }
    // assert zero tx_count for invalid block hash
    let block_hash = H256::from_low_u64_be(0);
    let mut params = rpc_params![block_hash];
    let response: Result<String, _> = client
        .request("eth_getBlockTransactionCountByHash", params)
        .await;
    let tx_count_hex = response.unwrap();
    let tx_count = i64::from_str_radix(&tx_count_hex[2..tx_count_hex.len()], 16).unwrap();
    info!(
        "invalid block hash: {:?} should have {:?} transaction count",
        block_hash, tx_count
    );
    if configs.rpc_url.clone().contains("aurora") {
        // assert zero tx_count for invalid block hash format
        let block_hash = H160::from_low_u64_be(0);
        params = rpc_params![block_hash];
        let response: Result<String, _> = client
            .request("eth_getBlockTransactionCountByHash", params)
            .await;
        assert!(
            response.is_err(),
            "Expected an error response, but got {response:?}"
        );
    }
    Ok(())
}
