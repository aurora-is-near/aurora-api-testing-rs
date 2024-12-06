use dao::helpers::{BlockWithTransactionReceipts, TransactionReceipt};
use dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[tokio::test]
async fn test_eth_get_block_by_number() -> anyhow::Result<()> {
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
        let params = rpc_params![receipts[i].block_number.to_string(), true];
        let response: Result<BlockWithTransactionReceipts, _> =
            client.request("eth_getBlockByNumber", params).await;
        let block = response.unwrap();
        assert_eq!(block.hash, receipts[i].block_hash);
        assert_eq!(
            i32::from_str_radix(&block.number[2..block.number.len()], 16).unwrap(),
            receipts[i].block_number
        );
        let tx_hashes: Vec<String> = block
            .transactions
            .into_iter()
            .filter(|t| t.hash == receipts[i].transaction_hash)
            .map(|t| t.hash)
            .collect();
        assert_eq!(tx_hashes[0], receipts[i].transaction_hash);
    }
    Ok(())
}
