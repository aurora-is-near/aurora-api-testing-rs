use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[path = "configs.rs"]
mod configs;
use configs::Configs;

// #[tokio::test]
// async fn test_eth_submit_hash_rate() -> anyhow::Result<()> {
//     let subscriber = FmtSubscriber::builder()
//         .with_max_level(Level::INFO)
//         .finish();
//     let _ = tracing::subscriber::set_global_default(subscriber);
//     let configs = Configs::load().unwrap();
//     let params = rpc_params![
//         "0x0000000000000000000000000000000000000000000000000000000000500000",
//         "0x59daa26581d0acd1fce254fb7e85952f4c09d0915afd33d3886cd914bc7d283c"
//     ];
//     let client = http_client::HttpClientBuilder::default().build(configs.rpc_url.clone())?;
//     let response: Result<String, jsonrpsee_core::Error> =
//         client.request("eth_SubmitHashRate", params).await;
//     if configs.rpc_url.clone().contains("aurora") {
//         let result = match response {
//             Ok(value) => value,
//             Err(error) => error.to_string(),
//         };
//         // info!("{}", result);
//         assert_eq!(
//             result.contains("Unsupported method: eth_SubmitHashRate"),
//             true
//         );
//     } else if configs.rpc_url.clone().contains("goerli") {
//         let goerli_responses = [
//             "the method eth_SubmitHashRate does not exist",
//             "method not supported",
//             "the method eth_SubmitHashRate does not exist",
//         ];
//         let result = match response {
//             Ok(value) => value,
//             Err(error) => error.to_string(),
//         };
//         // info!("{}", result);
//         let does_exist: Vec<bool> = goerli_responses
//             .iter()
//             .map(|v| result.contains(&v.to_string()))
//             .collect();
//         info!("Unsupported method ? {:?}", does_exist.contains(&true));
//     }
//
//     Ok(())
// }
