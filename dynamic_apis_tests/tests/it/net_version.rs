use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;

use crate::common::init;
use crate::configs::Configs;

use crate::utils::hex_string_to_i32;

#[tokio::test]
async fn test_net_version() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let mut params = rpc_params![];
    let net_version_response: Result<String, _> = client.request("net_version", params).await;
    params = rpc_params![];
    let chain_id_response: Result<String, _> = client.request("eth_chainId", params).await;
    assert_eq!(
        hex_string_to_i32(chain_id_response.unwrap()),
        net_version_response.unwrap().parse::<i32>().unwrap()
    );
    Ok(())
}
