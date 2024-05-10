use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::configs::Configs;

use crate::utils::hex_string_to_i32;

#[tokio::test]
async fn test_eth_get_uncle_count_by_block_hash() -> anyhow::Result<()> {
    let configs = Configs::load().unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let data_contents: Vec<String> = task
        .get_test_data_content_array("receipt".to_string())
        .unwrap();
    let receipts = TransactionReceipt::load(data_contents).unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    for receipt in receipts {
        info!("block hash: {}", receipt.block_hash.to_string());
        let params = rpc_params![receipt.block_hash.to_string()];
        let response: Result<String, _> =
            client.request("eth_getUncleCountByBlockHash", params).await;
        let uncle_count = response.unwrap();
        assert_eq!(hex_string_to_i32(uncle_count.clone()), 0);
        info!("eth_getUncleCountByBlockHash: {:?}", uncle_count.clone());
    }
    Ok(())
}
