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

#[path = "utils.rs"]
mod utils;
use utils::hex_string_to_i32;

#[path = "aurora_transaction_receipt.rs"]
mod aurora_transaction_receipt;
use aurora_transaction_receipt::AuroraTransactionReceipt;

#[tokio::test]
async fn test_eth_get_uncle_by_block_hash_and_index() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _t = tracing::subscriber::set_global_default(subscriber);
    info!("eth_getUncleBlockByHashAndIndex is not supported in Aurora");
    Ok(())
}
