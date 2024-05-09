use dao::dao::helpers::{Transaction, TransactionReceipt};
use dao::dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod configs;
use configs::Configs;

#[tokio::test]
async fn test_eth_get_transaction_by_hash() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    for group_id in 0..task.data_groups.len() {
        let receipt: String = task
            .get_test_data_content_by_group_index(group_id, "receipt".to_string())
            .unwrap();
        let transactions = TransactionReceipt::load(vec![receipt]).unwrap();
        let params = rpc_params![transactions[0].transaction_hash.clone()];
        let response: Result<Transaction, _> =
            client.request("eth_getTransactionByHash", params).await;
        let res = response.unwrap();
        info!("result: {:?}", res);
        assert_eq!(res.hash, transactions[0].transaction_hash);
        assert_eq!(res.block_hash, transactions[0].block_hash);
    }
    Ok(())
}
