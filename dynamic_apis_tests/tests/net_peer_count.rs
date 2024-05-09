use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod configs;
use configs::Configs;

#[path = "utils.rs"]
mod utils;
use utils::hex_string_to_i32;

#[tokio::test]
async fn test_net_peer_count() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _t = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![];
    let response: Result<String, _> = client.request("net_peerCount", params).await;
    assert_eq!(hex_string_to_i32(response.unwrap()), 0);
    Ok(())
}
