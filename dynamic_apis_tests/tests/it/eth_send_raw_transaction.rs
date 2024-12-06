use dao::helpers::SerdeError;
use dao::models::{TestRun, TestTask};
use ethers_core::abi::{Abi, AbiError, Token};
use ethers_core::types::{Address, Bytes};
use serial_test::serial;
use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::info;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use crate::common::init;
use crate::configs::Configs;
use crate::contract_utils::{SignerWallet, SmartContract};

#[tokio::test]
#[serial]
async fn test_eth_send_raw_transaction_increment() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let private_key = task
        .get_test_data_content_by_group_index(0, "destination_private_key".to_string())
        .unwrap();
    // deploy
    let abi = read_abi_from_json_file("tests/abis/incrementer.json").unwrap();
    let bytecode = read_bytes_from_file("tests/abis/incrementer.bytecode").unwrap();
    let contract = SmartContract::new(abi.clone(), bytecode.clone());
    let signer_wallet = SignerWallet::new(
        &configs.rpc_url,
        &private_key,
        configs.chain_id.parse::<u64>().unwrap(),
    );
    // call (value)
    let signer = signer_wallet.create().unwrap();
    let _signer_address = signer.address().to_string();
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
    let _guard = init();
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let private_key = task
        .get_test_data_content_by_group_index(0, "destination_private_key".to_string())
        .unwrap();
    let deployer_private_key = task
        .get_test_data_content_by_group_index(1, "destination_private_key".to_string())
        .unwrap();
    let _destination_address = task
        .get_test_data_content_by_group_index(0, "destination_address".to_string())
        .unwrap();
    let deployer_address = task
        .get_test_data_content_by_group_index(1, "destination_address".to_string())
        .unwrap();
    // deploy
    let abi = read_abi_from_json_file("tests/abis/watermelonToken.json").unwrap();
    let bytecode = read_bytes_from_file("tests/abis/watermelonToken.bytecode").unwrap();
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
            Some((Token::Address(receiver), Token::Uint(10.into()))),
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
            Some(Token::Address(receiver)),
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
    let to = Token::Address(receiver);
    let from = Token::Address(signer.address());
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
            Some((Token::Address(signer.address()), Token::Address(receiver))),
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
        .submit_no_gas(
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
            Some(to.clone()),
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

pub fn get_absolute_path(relative_path: &str) -> Option<PathBuf> {
    Some(Path::join(
        env::current_dir().unwrap().as_path(),
        Path::new(relative_path).to_str().unwrap(),
    ))
}

pub fn read_bytes_from_file(file: &str) -> Result<Bytes, AbiError> {
    let bytecode_path = get_absolute_path(file).unwrap();
    let bytecode_text = std::fs::read_to_string(&bytecode_path).unwrap();
    Ok(Bytes::from_str(&bytecode_text).unwrap())
}

pub fn read_abi_from_json_file(file: &str) -> Result<Abi, SerdeError> {
    let abi_path = get_absolute_path(file).unwrap();
    let abi_text = std::fs::read_to_string(&abi_path).unwrap();
    serde_json::from_str(&abi_text)
}
