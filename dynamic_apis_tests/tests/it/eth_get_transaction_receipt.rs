use dao::helpers::{Address, TransactionReceipt};
use dao::models::{TestRun, TestTask};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client as http_client;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::common::init;
use crate::configs::Configs;
use crate::utils::hex_string_to_i32;

#[tokio::test]
async fn test_eth_get_transaction_receipt() -> anyhow::Result<()> {
    let _guard = init();
    let configs = Configs::load().unwrap();
    let test_run = TestRun::new(&configs.conn, configs.network).unwrap();
    let task: TestTask = test_run
        .filter_tasks_with_limit_one("transferNtimes".to_string())
        .unwrap();
    let client = http_client::HttpClientBuilder::default().build(configs.rpc_url)?;
    for group_id in 0..task.data_groups.len() {
        let receipt: String = task
            .get_test_data_content_by_group_index(group_id, "receipt".to_string())
            .unwrap();
        let transactions = TransactionReceipt::load(vec![receipt]).unwrap();
        let params = rpc_params![transactions[0].transaction_hash.clone()];
        let response: Result<AuroraTransactionReceipt, _> =
            client.request("eth_getTransactionReceipt", params).await;
        let res = response.unwrap();
        // transaction hash
        assert_eq!(res.transaction_hash, transactions[0].transaction_hash);
        info!("transaction hash: {}", res.transaction_hash.clone());
        // block hash
        assert_eq!(res.block_hash, transactions[0].block_hash);
        // block number
        assert_eq!(
            hex_string_to_i32(res.block_number),
            transactions[0].block_number
        );
        // logs bloom
        assert_eq!(res.logs_bloom, transactions[0].logs_bloom);
        // contract address
        assert_eq!(res.contract_address, transactions[0].contract_address);
        // gas used
        assert_eq!(res.gas_used, transactions[0].gas_used.hex);
        // cumulative gas used
        assert_eq!(
            hex_string_to_i32(res.cumulative_gas_used),
            hex_string_to_i32(transactions[0].cumulative_gas_used.hex.clone())
        );

        // Status
        assert_eq!(hex_string_to_i32(res.status), transactions[0].status);
        // transaction index
        let on_chain_tx_index = hex_string_to_i32(res.transaction_index);
        let off_chain_tx_index = transactions[0].transaction_index;
        assert_eq!(on_chain_tx_index, off_chain_tx_index);
        // transaction logs
        for i in 0..res.logs.len() {
            // address
            assert_eq!(
                res.logs[i].address,
                transactions[0].logs[i].address.to_lowercase()
            );
            // block hash
            assert_eq!(
                res.logs[i].block_hash.clone(),
                transactions[0].logs[i].block_hash
            );
            // block number
            assert_eq!(
                hex_string_to_i32(res.logs[i].block_number.clone()),
                transactions[0].logs[i].block_number
            );
            // data
            assert_eq!(res.logs[i].data, transactions[0].logs[i].data);
            // log index
            assert_eq!(
                hex_string_to_i32(res.logs[i].log_index.clone()),
                transactions[0].logs[i].log_index
            );
            // topics length
            assert_eq!(
                res.logs[i].topics.len(),
                transactions[0].logs[i].topics.len()
            );
            // topics
            assert_eq!(res.logs[i].topics, transactions[0].logs[i].topics);
            assert_eq!(
                res.logs[i].transaction_hash,
                transactions[0].logs[i].transaction_hash
            );
            assert_eq!(
                hex_string_to_i32(res.logs[i].transaction_index.clone()),
                transactions[0].logs[i].transaction_index
            );
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Log {
    pub transaction_index: String,
    pub block_number: String,
    pub transaction_hash: String,
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub log_index: String,
    pub block_hash: String,
    pub removed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct AuroraTransactionReceipt {
    pub from: Address,
    pub to: Address,
    pub contract_address: Option<Address>,
    pub transaction_index: String,
    pub gas_used: String,
    pub logs_bloom: String,
    pub block_hash: String,
    pub transaction_hash: String,
    pub logs: Vec<Log>,
    pub block_number: String,
    pub cumulative_gas_used: String,
    pub status: String,
    pub near_receipt_hash: String,
    pub near_transaction_hash: String,
}
