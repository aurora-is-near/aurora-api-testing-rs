use dao::dao::models::{TestRun, TestTask};
use dao::utils::utils::get_env_var;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use std::cmp::Ordering;
use std::i64;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[path = "configs.rs"]
mod configs;
use configs::Configs;

#[tokio::test]
async fn test_eth_gas_price() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let client_aurora_plus = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let aurora_rpc_url = get_env_var(&"RPC_URL".to_string()).unwrap();
    let client_aurora = http_client::HttpClientBuilder::default().build(aurora_rpc_url)?;
    let mut params = rpc_params![];
    let response_aurora_plus: Result<String, _> =
        client_aurora_plus.request("eth_gasPrice", params).await;
    let mut gas_price_aurora_plus = response_aurora_plus.unwrap();
    let mut res = i64::from_str_radix(&gas_price_aurora_plus[2..gas_price_aurora_plus.len()], 16)
        .unwrap()
        .cmp(&0); // result should be zero for aurora plus
    assert_eq!(Ordering::Equal, res);
    info!("Aurora plus response: {}", gas_price_aurora_plus);
    params = rpc_params![];
    let response_aurora: Result<String, _> = client_aurora.request("eth_gasPrice", params).await;
    let gas_price_aurora = response_aurora.unwrap();
    res = i64::from_str_radix(&gas_price_aurora[2..gas_price_aurora.len()], 16)
        .unwrap()
        .cmp(&0); // result should be > zero
    assert_eq!(Ordering::Greater, res);
    info!("Aurora plus response: {}", gas_price_aurora);
    Ok(())
}