use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;

mod configs;
use configs::Configs;

#[tokio::test]
async fn test_net_listening() -> anyhow::Result<()> {
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![];
    let response: Result<bool, _> = client.request("net_listening", params).await;
    let result = response.unwrap();
    assert_eq!(result, true);
    Ok(())
}
