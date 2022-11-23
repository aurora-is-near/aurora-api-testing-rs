use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[path = "configs.rs"]
mod configs;
use configs::Configs;

#[path = "aurora_transaction_receipt.rs"]
mod aurora_transaction_receipt;
use aurora_transaction_receipt::AuroraTransactionReceipt;

#[tokio::test]
async fn test_eth_get_transaction_by_hash() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network, configs.runs_table).unwrap();
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
        info!(
            "Transaction receipt: {:?}",
            transactions[0].transaction_hash.to_string()
        );
        let response: Result<AuroraTransactionReceipt, _> =
            client.request("eth_getTransactionReceipt", params).await;
        let res = response.unwrap();
        info!("result: {:?}", res);
        assert_eq!(res.transaction_hash, transactions[0].transaction_hash);
        assert_eq!(res.block_hash, transactions[0].block_hash);
        let on_chain_block_number =
            i32::from_str_radix(&res.block_number[2..res.block_number.len()], 16).unwrap();
        assert_eq!(on_chain_block_number, transactions[0].block_number);
        // assert_eq!(res.logs_bloom, transactions[0].logs_bloom); //TODO: this assertion is not working !
    }
    Ok(())
}
