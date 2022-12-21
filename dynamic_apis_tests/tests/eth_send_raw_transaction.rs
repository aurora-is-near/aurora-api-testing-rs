use dao::dao::helpers::TransactionReceipt;
use dao::dao::models::{TestRun, TestTask};
use ethers_contract::Contract;
use ethers_core::abi::{Abi, Token, Tokenize};
use ethers_core::types::{Address, Bytes, U256};
use hex;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use serial_test::serial;
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
#[serial]
async fn test_eth_send_raw_transaction_increment() -> anyhow::Result<()> {
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
    // deploy
    let abi = utils::read_abi_from_json_file("tests/abis/incrementer.json").unwrap();
    let bytecode = utils::read_bytes_from_file("tests/abis/incrementer.bytecode").unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let signer_wallet = SignerWallet::new(
        &configs.rpc_url,
        &private_key,
        configs.chain_id.parse::<u64>().unwrap(),
    );
    // call (value)
    let signer = signer_wallet.create().unwrap();
    let contract_address = contract.deploy(Some(()), signer).await.unwrap();
    info!("Increment contract Address: {:?}", contract_address);
    let signer = signer_wallet.create().unwrap();
    let value = contract
        .call::<_, i32>(contract_address, "value", Some(()), signer)
        .await
        .unwrap();
    info!("Current Value: {:?}", value.to_string());
    assert_eq!(value, 0);
    // send tx to increment the value++
    let signer = signer_wallet.create().unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let receipt = contract
        .submit(contract_address, "increment", Some(()), signer)
        .await
        .unwrap();
    info!("increment Receipt: {:?}", receipt);
    // assert the new incremented value
    let signer = signer_wallet.create().unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let value = contract
        .call::<_, i32>(contract_address, "value", Some(()), signer)
        .await
        .unwrap();
    info!("Current Value: {:?}", value.to_string());
    assert_eq!(value, 1);
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_eth_send_raw_transaction_wtm() -> anyhow::Result<()> {
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
    let deployer_private_key = task
        .get_test_data_content_by_group_index(0, "deployer_private_key".to_string())
        .unwrap();
    let destination_address = task
        .get_test_data_content_by_group_index(0, "destination_address".to_string())
        .unwrap();
    let deployer_address = task
        .get_test_data_content_by_group_index(0, "deployer_address".to_string())
        .unwrap();
    // deploy
    let abi = utils::read_abi_from_json_file("tests/abis/watermelonToken.json").unwrap();
    let bytecode = utils::read_bytes_from_file("tests/abis/watermelonToken.bytecode").unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let signer_wallet = SignerWallet::new(
        &configs.rpc_url,
        &private_key,
        configs.chain_id.parse::<u64>().unwrap(),
    );
    // call (total supply)
    let signer = signer_wallet.create().unwrap();
    let initial_supply = Token::Uint(1000.into());
    let contract_address = contract
        .deploy(Some(initial_supply.clone()), signer)
        .await
        .unwrap();
    info!("WTM token contract address: {:?}", contract_address);
    let signer = signer_wallet.create().unwrap();
    let value = contract
        .call::<_, i32>(contract_address, "totalSupply", Some(()), signer)
        .await
        .unwrap();
    info!("total supply: {:?}", value.to_string());
    assert_eq!(Token::Uint(value.into()), initial_supply);
    // transfer
    let signer = signer_wallet.create().unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let receiver: Address = deployer_address.parse().unwrap();
    let receipt = contract
        .submit(
            contract_address,
            "transfer",
            Some((Token::Address(receiver.clone()), Token::Uint(10.into()))),
            signer,
        )
        .await
        .unwrap();
    info!("Transfer Receipt: {:?}", receipt);
    // balanceOf after calling transfer
    let signer = signer_wallet.create().unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let receiver: Address = deployer_address.parse().unwrap();
    let balance = contract
        .call::<_, i32>(
            contract_address,
            "balanceOf",
            Some(Token::Address(receiver.clone())),
            signer.clone(),
        )
        .await
        .unwrap();
    info!(
        "Balance of receiver: {:?}, {:?}",
        receiver.clone(),
        balance.to_string()
    );
    assert_eq!(balance, 10);
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let balance = contract
        .call::<_, i32>(
            contract_address,
            "balanceOf",
            Some(Token::Address(signer.clone().address())),
            signer.clone(),
        )
        .await
        .unwrap();
    info!(
        "Balance of spender: {:?}, {:?}",
        signer.address().clone(),
        balance.to_string()
    );
    // approve
    let signer = signer_wallet.create().unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let to = Token::Address(receiver.clone());
    let from = Token::Address(signer.address().clone());
    let amount = Token::Uint(10.into());
    let receipt = contract
        .submit(
            contract_address,
            "approve",
            Some((to.clone(), amount)),
            signer,
        )
        .await
        .unwrap();
    // allowance
    let signer = signer_wallet.create().unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let receiver: Address = deployer_address.parse().unwrap();
    let allowance = contract
        .call::<_, i32>(
            contract_address,
            "allowance",
            Some((
                Token::Address(signer.address().clone()),
                Token::Address(receiver.clone()),
            )),
            signer.clone(),
        )
        .await
        .unwrap();
    info!(
        "Allowance for: {:?} is {:?}",
        receiver.clone(),
        allowance.to_string()
    );
    info!("from: {:?}, to: {:?}", from.clone(), to.clone());
    info!("Approve Receipt: {:?}", receipt);
    let signer_wallet = SignerWallet::new(
        &configs.rpc_url,
        &deployer_private_key,
        configs.chain_id.parse::<u64>().unwrap(),
    );
    let signer = signer_wallet.create().unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let amount = Token::Uint(10.into());
    let receipt = contract
        .submit(
            contract_address,
            "transferFrom",
            Some((from.clone(), to.clone(), amount)),
            signer,
        )
        .await
        .unwrap();
    info!("TransferFrom Receipt: {:?}", receipt);
    let signer = signer_wallet.create().unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let receiver: Address = deployer_address.parse().unwrap();
    let balance = contract
        .call::<_, i32>(
            contract_address,
            "balanceOf",
            Some(Token::Address(receiver.clone())),
            signer.clone(),
        )
        .await
        .unwrap();
    info!(
        "Balance of receiver after transferFrom: {:?}, {:?}",
        receiver.clone(),
        balance.to_string()
    );
    //TODO: check failed transferFrom tx
    //assert_eq!(balance, 20);
    Ok(())
}
