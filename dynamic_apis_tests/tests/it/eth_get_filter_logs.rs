use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[tokio::test]
async fn test_eth_get_filter_logs() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params!["0x0"];
    let response: Result<Vec<String>, jsonrpsee_core::Error> =
        client.request("eth_getFilterLogs", params).await;
    let res = response;
    info!("filter changes: {:?}", res); // no filters
    Ok(())
}
