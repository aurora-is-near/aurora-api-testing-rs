use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::{common::init, configs::Configs};

#[tokio::test]
async fn test_eth_new_pending_transaction_filter() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![];
    let response: Result<String, _> = client
        .request("eth_newPendingTransactionFilter", params)
        .await;
    let result = response.unwrap();
    info!("New filter Id: {}", result);
    Ok(())
}
