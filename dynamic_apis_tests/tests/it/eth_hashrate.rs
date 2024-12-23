use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

use crate::utils::hex_string_to_i32;

#[tokio::test]
async fn test_eth_hashrate() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![];
    let eth_hash_rate: Result<String, _> = client.request("eth_hashrate", params).await;
    let hash_rate = eth_hash_rate.unwrap();
    info!("eth_hashrate: {:?}", hash_rate.clone());
    assert_eq!(hex_string_to_i32(hash_rate.clone()), 0);
    Ok(())
}
