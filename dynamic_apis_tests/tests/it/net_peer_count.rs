use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;

use crate::common::init;
use crate::configs::Configs;

use crate::utils::hex_string_to_i32;

#[tokio::test]
async fn test_net_peer_count() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![];
    let response: Result<String, _> = client.request("net_peerCount", params).await;
    assert_eq!(hex_string_to_i32(response.unwrap()), 0);
    Ok(())
}
