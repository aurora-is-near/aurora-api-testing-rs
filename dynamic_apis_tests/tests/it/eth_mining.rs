use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;

use crate::common::init;
use crate::configs::Configs;

#[tokio::test]
async fn test_eth_mining() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![];
    let response: Result<bool, _> = client.request("eth_mining", params).await;
    let result = response.unwrap();
    assert!(!result);
    Ok(())
}
