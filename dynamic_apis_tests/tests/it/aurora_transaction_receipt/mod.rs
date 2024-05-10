use dao::dao::helpers::Address;
use serde::{Deserialize, Serialize};

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
