use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[tokio::test]
async fn test_eth_accounts() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let params = rpc_params![];
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let response: Result<Vec<String>, _> = client.request("eth_accounts", params).await;
    assert_eq!(response.as_ref().unwrap().len(), 0);
    info!("response: {:?}", response.unwrap());
    Ok(())
}
