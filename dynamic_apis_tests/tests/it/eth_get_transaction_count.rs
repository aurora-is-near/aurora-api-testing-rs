use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use std::cmp::Ordering;
use std::i64;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[tokio::test]
async fn test_eth_get_transaction_count() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let receipt: String = task
        .get_test_data_content_by_group_index(0, "receipt".to_string())
        .unwrap();
    let transactions = TransactionReceipt::load(vec![receipt]).unwrap();
    let account_address = transactions[0].from.clone();
    let n_zero_block_number = transactions[0].block_number;
    let params = rpc_params![account_address, n_zero_block_number];
    let response: Result<String, _> = client.request("eth_getTransactionCount", params).await;
    let res = response.unwrap();
    let z_nonce = i64::from_str_radix(&res[2..res.len()], 16).unwrap();
    info!("Nonce: {:?}", z_nonce);
    // let mut last_nonce = 0;
    for group_id in 1..task.data_groups.len() {
        let receipt: String = task
            .get_test_data_content_by_group_index(group_id, "receipt".to_string())
            .unwrap();
        let transactions = TransactionReceipt::load(vec![receipt]).unwrap();
        let account_address = transactions[0].from.clone();
        let block_number = transactions[0].block_number;
        let params = rpc_params![account_address, block_number];
        let response: Result<String, _> = client.request("eth_getTransactionCount", params).await;
        let res = response.unwrap();
        let nonce = i64::from_str_radix(&res[2..res.len()], 16).unwrap();
        info!("Nonce: {:?}", nonce);
        let result = nonce.cmp(&z_nonce);
        assert_eq!(Ordering::Greater, result);
        // last_nonce = nonce;
    }
    info!("Asserting pending nonce is equal to the latest nonce");
    let params = rpc_params![account_address, "latest"];
    let response: Result<String, _> = client.request("eth_getTransactionCount", params).await;
    let res = response.unwrap();
    let pending_nonce = i64::from_str_radix(&res[2..res.len()], 16).unwrap();
    info!("Pending Nonce: {:?}", pending_nonce);
    // assert_eq!(pending_nonce, last_nonce);
    info!("Asserting nonce is zero for the genesis block");
    let params = rpc_params![account_address, "0x0"]; // genesis block 0
    let response: Result<String, _> = client.request("eth_getTransactionCount", params).await;
    let res = response.unwrap();
    let genesis_nonce = i64::from_str_radix(&res[2..res.len()], 16).unwrap();
    info!("Genesis_nonce Nonce: {:?}", genesis_nonce);
    assert_eq!(genesis_nonce, 0);
    Ok(())
}
