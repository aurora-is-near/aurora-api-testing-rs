use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::info;

use crate::common::init;
use crate::configs::Configs;

#[ignore]
#[tokio::test]
async fn test_eth_submit_hash_rate() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let params = rpc_params![
        "0x0000000000000000000000000000000000000000000000000000000000500000",
        "0x59daa26581d0acd1fce254fb7e85952f4c09d0915afd33d3886cd914bc7d283c"
    ];
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url.clone())?;
    let response: Result<String, jsonrpsee_core::Error> =
        client.request("eth_SubmitHashRate", params).await;
    if configs.rpc_url.clone().contains("aurora") {
        let result = match response {
            Ok(value) => value,
            Err(error) => error.to_string(),
        };
        // info!("{}", result);
        assert_eq!(result.contains("method not supported"), true);
    } else if configs.rpc_url.clone().contains("goerli") {
        let goerli_responses = [
            "the method eth_SubmitHashRate does not exist",
            "method not supported",
            "the method eth_SubmitHashRate does not exist",
        ];
        let result = match response {
            Ok(value) => value,
            Err(error) => error.to_string(),
        };
        // info!("{}", result);
        let does_exist: Vec<bool> = goerli_responses
            .iter()
            .map(|v| result.contains(&v.to_string()))
            .collect();
        info!("Unsupported method ? {:?}", does_exist.contains(&true));
    }

    Ok(())
}
