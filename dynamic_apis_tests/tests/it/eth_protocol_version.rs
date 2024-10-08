use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

use crate::utils::hex_string_to_i32;

#[tokio::test]
async fn test_eth_protocol_version() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![];
    let response: Result<String, _> = client.request("eth_protocolVersion", params).await;
    let protocol_version = hex_string_to_i32(response.unwrap());
    assert_eq!(protocol_version.to_string(), configs.protocol_version);
    info!("eth_protocolVersion is: {}", protocol_version);
    Ok(())
}
