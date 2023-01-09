use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[path = "configs.rs"]
mod configs;
use configs::Configs;

#[tokio::test]
async fn test_eth_coinbase() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let params = rpc_params![];
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url.clone())?;
    let response: Result<String, _> = client.request("eth_coinbase", params).await;
    let coinbase = response.unwrap();
    if configs.rpc_url.clone().contains("goerli") {
        info!("Goerli coinbase value is: {}", coinbase.clone().to_string());
    } else if configs.rpc_url.clone().contains("aurora") {
        info!("Aurora coinbase value is: {}", coinbase.clone().to_string());
        assert_eq!(
            coinbase,
            "0x0000000000000000000000000000000000000000".to_string()
        );
    }
    Ok(())
}
