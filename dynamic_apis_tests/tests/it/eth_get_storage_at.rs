use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{TestRun, TestTask};
use hex;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::configs::Configs;

#[tokio::test]
async fn test_eth_get_storage_at() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let group_id = 0;
    let contract_address: String = task
        .get_test_data_content_by_group_index(group_id, "contract_address".to_string())
        .unwrap();
    let receipt: String = task
        .get_test_data_content_by_group_index(group_id, "receipt".to_string())
        .unwrap();
    let transactions = TransactionReceipt::load(vec![receipt]).unwrap();
    let storage_location = String::from("0x02");
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    let params = rpc_params![
        contract_address.clone(),
        storage_location,
        transactions[0].block_number
    ];
    let response: Result<String, _> = client.request("eth_getStorageAt", params).await;
    let res = response.unwrap();
    let total_supply = i32::from_str_radix(&res[2..res.len()], 16).unwrap();
    let expected_total_supply = 1000000;
    info!("Asserting total_supply is {}", total_supply);
    assert_eq!(total_supply, expected_total_supply);
    let expected_token_name = String::from("Watermelon");
    let storage_location = String::from("0x03");
    let params = rpc_params![
        contract_address.clone(),
        storage_location,
        transactions[0].block_number
    ];
    let response: Result<String, _> = client.request("eth_getStorageAt", params).await;
    let token_name = response.unwrap();
    let decoded_token_name =
        hex::decode(&token_name[2..token_name.len()]).expect("Decoding failed");
    let token_name_str = match std::str::from_utf8(&decoded_token_name) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    info!("Asserting token name is: {:?}", token_name_str);
    assert_eq!(token_name_str.contains(&expected_token_name), true);
    let expected_token_symbol = String::from("WTM");
    let storage_location = String::from("0x04");
    let params = rpc_params![
        contract_address.clone(),
        storage_location,
        transactions[0].block_number
    ];
    let response: Result<String, _> = client.request("eth_getStorageAt", params).await;
    let token_symbol = response.unwrap();
    let decoded_token_symbol =
        hex::decode(&token_symbol[2..token_symbol.len()]).expect("Decoding failed");
    let token_symbol_str = match std::str::from_utf8(&decoded_token_symbol) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    info!("Asserting token symbol is: {:?}", token_symbol_str);
    assert_eq!(token_symbol_str.contains(&expected_token_symbol), true);
    Ok(())
}
