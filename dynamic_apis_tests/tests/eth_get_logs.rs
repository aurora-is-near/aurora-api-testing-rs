use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{get_db_connection, TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use serde::{Deserialize, Serialize};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[path = "configs.rs"]
mod configs;
use configs::Configs;

#[path = "utils.rs"]
mod utils;
use utils::hex_string_to_i32;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
struct LogFilter {
    topics: Vec<String>,
    address: String,
    from_block: String,
    to_block: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
struct LogResult {
    address: String,
    block_hash: String,
    block_number: String,
    data: String,
    log_index: String,
    removed: bool,
    topics: Vec<String>,
    transaction_hash: String,
    transaction_index: String,
}

#[tokio::test]
async fn test_eth_get_logs() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let test_run = TestRun::new(&configs.conn, configs.network, configs.runs_table).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    for group_id in 0..task.data_groups.len() {
        let receipt: String = task
            .get_test_data_content_by_group_index(group_id, "receipt".to_string())
            .unwrap();
        let transactions = TransactionReceipt::load(vec![receipt]).unwrap();
        for log in &transactions[0].logs {
            let topics: Vec<String> = log.topics.iter().map(|t| t.clone()).collect();
            let log_filter = LogFilter {
                topics: vec![topics[0].clone()],
                address: log.address.to_string(),
                from_block: log.block_number.to_string().clone(),
                to_block: log.block_number.to_string().clone(),
            };
            let params = rpc_params![log_filter];
            info!(
                "Asserting logs for block {:?} and topic: {:?}",
                log.block_number.to_string().clone(),
                topics[0].clone()
            );
            let response: Result<Vec<LogResult>, _> = client.request("eth_getLogs", params).await;
            let res = response.unwrap();
            for log_result in &res {
                // contract address
                assert_eq!(log_result.address, log.address.to_lowercase());
                // block number
                assert_eq!(
                    hex_string_to_i32(log_result.block_number.clone()),
                    log.block_number
                );
                // block hash
                assert_eq!(log_result.block_hash.clone(), log.block_hash.clone());
                // topics
                for i in 0..log_result.topics.len() {
                    assert_eq!(log_result.topics[i], topics[i]);
                }
                // log index
                assert_eq!(
                    hex_string_to_i32(log_result.log_index.clone()),
                    log.log_index
                );
                // transaction hash
                assert_eq!(log_result.transaction_hash, log.transaction_hash);
                // transaction data
                assert_eq!(log_result.data, log.data);
                // transaction index
                assert_eq!(
                    hex_string_to_i32(log_result.transaction_index.clone()),
                    log.transaction_index
                );
            }
            info!("onchain log:{:?} ", res);
        }
    }
    Ok(())
}
