use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{TestRun, TestTask};
use ethers_core::abi::{Abi, Token};
use ethers_core::types::Bytes;
use ethers_core::types::U256;
use hex;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

#[path = "configs.rs"]
mod configs;
use configs::Configs;

#[path = "utils.rs"]
mod utils;
use utils::hex_string_to_i32;

#[path = "aurora_transaction_receipt.rs"]
mod aurora_transaction_receipt;
use aurora_transaction_receipt::AuroraTransactionReceipt;

#[path = "contract_utils.rs"]
mod contract_utils;
use contract_utils::SmartContract;

#[tokio::test]
async fn test_eth_send_raw_transaction_deploy() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url.clone())?;
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let private_key = task
        .get_test_data_content_by_group_index(0, "destination_private_key".to_string())
        .unwrap();
    let from_address = task
        .get_test_data_content_by_group_index(0, "destination_address".to_string())
        .unwrap();
    let params = rpc_params![from_address.clone(), "latest"];
    let response: Result<String, _> = client.request("eth_getTransactionCount", params).await;
    let res = response.unwrap();
    let pending_nonce = hex_string_to_i32(res) + 1;
    let abi: Abi = serde_json::from_str(
        r#"[{"inputs": [], "name": "check", "outputs": [{"internalType": "uint256", "name": "", "type": "uint256"}],"stateMutability": "pure", "type": "function"}]"#,
    )?;
    let mut decoded = [0, 4];
    let decoded_bytecode = hex::decode_to_slice(
        "0x608060405234801561001057600080fd5b5060b68061001f6000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c8063919840ad14602d575b600080fd5b60336047565b604051603e9190605d565b60405180910390f35b60006005905090565b6057816076565b82525050565b6000602082019050607060008301846050565b92915050565b600081905091905056fea26469706673582212201b8a5fbc5e869b9317072c716b16bb6c4189c2476bd913fe1f28559a2cca2cba64736f6c63430008070033",
        &mut decoded
    );
    let bytecode: Bytes = Bytes::from(decoded);
    let contract = SmartContract::new(abi, bytecode);
    let contract_address = contract
        .deploy(
            &configs.rpc_url,
            &private_key,
            configs.chain_id.parse::<u64>().unwrap(),
            Some(()),
        )
        .await
        .unwrap();
    info!("contract address: {:?}", contract_address);
    Ok(())
}
