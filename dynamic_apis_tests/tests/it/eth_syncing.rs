use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::configs::Configs;

#[tokio::test]
async fn test_eth_syncing() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _t = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![];
    let response: Result<bool, _> = client.request("eth_syncing", params).await;
    assert!(!response.unwrap());
    Ok(())
}
