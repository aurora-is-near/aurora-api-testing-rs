use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod configs;
use configs::Configs;

#[tokio::test]
async fn test_web3_sha3() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _t = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params!["0x68656c6c6f20776f726c64"];
    let response: Result<String, _> = client.request("web3_sha3", params).await;
    let result = response.unwrap();
    // web3_sha3
    assert_eq!(
        result.clone(),
        "0x47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad"
    );
    info!("web3_sha3 output is: {:?}", result);
    Ok(())
}
