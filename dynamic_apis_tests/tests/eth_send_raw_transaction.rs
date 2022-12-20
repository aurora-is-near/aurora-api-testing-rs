use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{TestRun, TestTask};
use ethers_contract::Contract;
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
use contract_utils::{SignerWallet, SmartContract};

#[tokio::test]
async fn test_eth_send_raw_transaction() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let private_key = task
        .get_test_data_content_by_group_index(0, "destination_private_key".to_string())
        .unwrap();
    let abi = utils::read_abi_from_json_file("tests/abis/incrementer.json").unwrap();
    let bytecode = utils::read_bytes_from_file("tests/abis/incrementer.bytecode").unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let signer_wallet = SignerWallet::new(
        &configs.rpc_url,
        &private_key,
        configs.chain_id.parse::<u64>().unwrap(),
    );
    let signer = signer_wallet.create().unwrap();
    let contract_address = contract.deploy(Some(()), signer).await.unwrap();
    info!("Address: {:?}", contract_address);
    let signer = signer_wallet.create().unwrap();
    let value = contract
        .call::<_, i32>(contract_address, "value", Some(()), signer)
        .await
        .unwrap();
    info!("Value: {:?}", value.to_string());
    assert_eq!(value, 0);
    let signer = signer_wallet.create().unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let receipt = contract
        .submit(contract_address, "increment", Some(()), signer)
        .await
        .unwrap();
    info!("Receipt: {:?}", receipt);
    let signer = signer_wallet.create().unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let value = contract
        .call::<_, i32>(contract_address, "value", Some(()), signer)
        .await
        .unwrap();
    info!("Value: {:?}", value.to_string());
    assert_eq!(value, 1);
    Ok(())
}
