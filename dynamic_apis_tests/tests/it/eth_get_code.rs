use dao::helpers::TransactionReceipt;
use dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[tokio::test]
async fn test_eth_get_code() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    for group_id in 0..task.data_groups.len() {
        let bytecode: String = task
            .get_test_data_content_by_group_index(group_id, "contract_bytecode".to_string())
            .unwrap();
        let receipt: String = task
            .get_test_data_content_by_group_index(group_id, "receipt".to_string())
            .unwrap();
        let transactions = TransactionReceipt::load(vec![receipt]).unwrap();
        let contract_address: String = task
            .get_test_data_content_by_group_index(group_id, "contract_address".to_string())
            .unwrap();
        info!(
            "group id: {}, contract address: {:?} @ block number {}",
            group_id,
            contract_address.clone(),
            transactions[0].block_number
        );
        let params = rpc_params![contract_address.clone(), transactions[0].block_number];
        let response: Result<String, _> = client.request("eth_getCode", params).await;
        let res = response.unwrap();
        assert_eq!(bytecode, res);
        let destination_address: String = task
            .get_test_data_content_by_group_index(group_id, "destination_address".to_string())
            .unwrap();
        let params = rpc_params![destination_address.clone(), transactions[0].block_number];
        let response: Result<String, _> = client.request("eth_getCode", params).await;
        let res = response.unwrap();
        info!(
            "group id: {}, destination address: {:?} @ block number {}",
            group_id,
            destination_address.clone(),
            transactions[0].block_number
        );
        assert_eq!(res.len(), 2); // "0x"
    }
    Ok(())
}
