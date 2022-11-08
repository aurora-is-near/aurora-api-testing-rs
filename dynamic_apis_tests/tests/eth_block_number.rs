use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{get_db_connection, TestRun, TestTask};
use dao::utils::utils::{get_env_var, get_full_db_path};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use std::cmp::Ordering;
use std::i64;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::test]
async fn test_eth_block_number() -> anyhow::Result<()> {
    let network_name =
        get_env_var(&"NETWORK_NAME".to_string()).unwrap_or("mainnet_aurora_plus".to_string());
    let full_db_path = get_full_db_path().unwrap();
    let conn = get_db_connection(&full_db_path).unwrap();
    let runs_table =
        get_env_var(&"RUNS_TABLE".to_string()).unwrap_or("aurora_relayer_test_runs".to_string());
    let test_run = TestRun::new(&conn, network_name, runs_table).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let data_contents: Vec<String> = task
        .get_test_data_content_array("receipt".to_string())
        .unwrap();
    let receipts = TransactionReceipt::load(data_contents).unwrap();
    let rpc_url = get_env_var(&"RPC_URL".to_string()).unwrap();
    let api_key = get_env_var(&"API_KEY".to_string()).unwrap();
    let url = format!("{}{}", rpc_url, api_key);
    let client = http_client::HttpClientBuilder::default().build(url)?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber);
    for i in 0..receipts.len() {
        let params = rpc_params![];
        let response: Result<String, _> = client.request("eth_blockNumber", params).await;
        let block_number = response.unwrap();
        let len = block_number.len();
        let live_block_number = i64::from_str_radix(&block_number[2..len], 16).unwrap();
        let receipt_block_number = receipts[i].block_number as i64;
        let result = live_block_number.cmp(&receipt_block_number);
        assert_eq!(Ordering::Greater, result);
        info!(
            "Live block number: {}, receipt block number: {}",
            live_block_number, receipt_block_number
        );
    }
    Ok(())
}
