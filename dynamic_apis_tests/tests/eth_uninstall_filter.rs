use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use serde::{Deserialize, Serialize};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[path = "configs.rs"]
mod configs;
use configs::Configs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct FilterParams {
    pub from_block: String,
    pub to_block: String,
}

impl FilterParams {
    pub fn new(from_block: String, to_block: String) -> FilterParams {
        FilterParams {
            from_block,
            to_block,
        }
    }
}

// #[tokio::test]
// async fn test_eth_uninstall_filter() -> anyhow::Result<()> {
//     let subscriber = FmtSubscriber::builder()
//         .with_max_level(Level::INFO)
//         .finish();
//     let _t = tracing::subscriber::set_global_default(subscriber);
//     let configs = Configs::load().unwrap();
//     let client = http_client::HttpClientBuilder::default().build(configs.rpc_url.clone())?;
//     let filter_id: String;
//     if configs.rpc_url.contains("aurora") {
//         let params = rpc_params![];
//         let tx_pending_filter_id: Result<String, _> = client
//             .request("eth_newPendingTransactionFilter", params)
//             .await;
//         filter_id = tx_pending_filter_id.unwrap();
//     } else {
//         let params = rpc_params![FilterParams::new("0x0".to_string(), "0x1".to_string())];
//         let new_filter_id: Result<String, _> = client.request("eth_newFilter", params).await;
//         filter_id = new_filter_id.unwrap();
//     }
//     let params = rpc_params![filter_id];
//     let response: Result<bool, _> = client.request("eth_uninstallFilter", params).await;
//     let status = response.unwrap();
//     assert_eq!(status, true);
//     info!("eth_uninstallFilter status: {}", status);
//     Ok(())
// }
