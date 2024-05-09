use dao::dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::i64;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::configs::Configs;

#[derive(Serialize, Deserialize, Debug)]
struct MessageCallParams {
    from: String,
    to: String,
    value: String,
}

impl MessageCallParams {
    pub fn new(from: String, to: String) -> MessageCallParams {
        MessageCallParams {
            from,
            to,
            value: "0".to_string(),
        }
    }
}

#[tokio::test]
async fn test_eth_estimate_gas() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let from_address_content: String = task
        .get_test_data_content_by_group_index(0, "deployer_address".to_string())
        .unwrap();

    let to_address_content: String = task
        .get_test_data_content_by_group_index(0, "destination_address".to_string())
        .unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let msg = MessageCallParams::new(from_address_content, to_address_content);
    let params = rpc_params![msg];
    // info!("msg: {:?}",params);
    let response: Result<String, _> = client.request("eth_estimateGas", params).await;
    let result = response.unwrap();
    let estimate_gas = i64::from_str_radix(&result[2..result.len()], 16).unwrap();
    info!("response: {:?}", estimate_gas);
    let res = estimate_gas.cmp(&21000); // result should be >= 21000
    assert_eq!(Ordering::Greater, res);
    Ok(())
}
