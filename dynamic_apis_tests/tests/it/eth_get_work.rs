use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[ignore]
#[tokio::test]
async fn test_eth_get_work() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let params = rpc_params![];
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url.clone())?;
    let response = client.request("eth_getWork", params).await;
    if configs.rpc_url.clone().contains("aurora") {
        let result = match response {
            Ok(value) => value,
            Err(error) => error.to_string(),
        };
        // info!("{}", result);
        assert!(result.contains("method not supported"));
    } else if configs.rpc_url.clone().contains("goerli") {
        let goerli_responses = [
            "the method eth_getWork does not exist",
            "method not supported",
            "the method eth_getWork does not exist",
        ];
        let result = match response {
            Ok(value) => value,
            Err(error) => error.to_string(),
        };
        // info!("{}", result);
        let does_exist: Vec<bool> = goerli_responses
            .iter()
            .map(|v| result.contains(&(*v).to_string()))
            .collect();
        info!("Unsupported method ? {:?}", does_exist.contains(&true));
    }
    Ok(())
}
