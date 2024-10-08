use dao::dao::helpers::{Transaction, TransactionReceipt};
use dao::dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[tokio::test]
async fn test_eth_get_transaction_by_block_hash_and_index() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url.clone())?;
    let mut last_block_hash = String::from("0x0");
    for group_id in 0..task.data_groups.len() {
        let receipt: String = task
            .get_test_data_content_by_group_index(group_id, "receipt".to_string())
            .unwrap();
        let transactions = TransactionReceipt::load(vec![receipt]).unwrap();
        let params = rpc_params![
            transactions[0].block_hash.clone(),
            transactions[0].transaction_index
        ];
        let response: Result<Option<Transaction>, _> = client
            .request("eth_getTransactionByBlockHashAndIndex", params)
            .await;
        let res = response.unwrap();
        info!(
            "result: {:?}, blockhash: {}, transactionIndex: {}",
            res,
            transactions[0].block_hash.clone(),
            transactions[0].transaction_index
        );
        assert_eq!(res.unwrap().block_hash, transactions[0].block_hash);
        last_block_hash = transactions[0].block_hash.clone();
    }
    if configs.rpc_url.clone().contains("aurora") {
        info!("assert no transaction @ invalid block hash");
        let block_hash = "0x0";
        let transaction_index = 0;
        let params = rpc_params![block_hash, transaction_index];
        let response: Result<Option<Transaction>, jsonrpsee_core::Error> = client
            .request("eth_getTransactionByBlockHashAndIndex", params)
            .await;
        assert!(
            response.is_err(),
            "Expected an error response, but got {:?}",
            response
        );
        info!("assert no transaction @ invalid transaction index");
        let invalid_transaction_index = 50;
        let params = rpc_params![last_block_hash, invalid_transaction_index];
        let response: Result<Option<Transaction>, _> = client
            .request("eth_getTransactionByBlockHashAndIndex", params)
            .await;
        assert_eq!(response.unwrap().is_none(), true);
    }
    Ok(())
}
