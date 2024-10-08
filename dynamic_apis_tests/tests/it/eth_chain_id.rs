use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use std::cmp::Ordering;
use std::i64;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[tokio::test]
async fn test_eth_chain_id() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![];
    let response: Result<String, _> = client.request("eth_chainId", params).await;
    let result = response.unwrap();
    let chain_id = i64::from_str_radix(&result[2..result.len()], 16).unwrap();
    let res = chain_id.cmp(&0);
    info!("chain_id: {:?}", chain_id);
    assert_eq!(chain_id.to_string(), configs.chain_id);
    assert_eq!(Ordering::Greater, res);
    Ok(())
}
