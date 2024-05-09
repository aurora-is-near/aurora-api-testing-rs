use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;

use crate::configs::Configs;

#[tokio::test]
async fn test_web3_client_version() -> anyhow::Result<()> {
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![];
    let response: Result<String, _> = client.request("web3_clientVersion", params).await;
    let result = response.unwrap();
    assert_eq!(result, configs.client_version);
    Ok(())
}
