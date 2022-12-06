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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
struct LogFilter {
    topics: Vec<String>,
    address: String,
    from_block: String,
    to_block: String,
}

#[tokio::test]
async fn test_eth_new_filter() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
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
            let response: Result<String, _> = client.request("eth_newFilter", params).await;
            let res = response.unwrap();
            info!("eth_newFilter: {:?}", res);
        }
    }
    Ok(())
}
