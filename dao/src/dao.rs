pub mod models;
#[macro_use]

pub mod helpers {
    pub use ethereum_types::{Address, H256};
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{Error as SerdeError, Value};
    pub use std::error::Error;
    extern crate serde;
    extern crate serde_derive;
    extern crate serde_json;

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
    pub struct GasUsed {
        #[serde(rename = "type")]
        pub gas_type: String,
        pub hex: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
    pub struct Log {
        pub transaction_index: i32,
        pub block_number: i32,
        pub transaction_hash: String,
        pub address: Address,
        pub topics: Vec<String>,
        pub data: String,
        pub log_index: i32,
        pub block_hash: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
    pub struct Event {
        pub transaction_index: i32,
        pub block_number: i32,
        pub transaction_hash: String,
        pub address: Address,
        pub topics: Vec<String>,
        pub data: String,
        pub log_index: i32,
        pub block_hash: String,
        pub args: Vec<Value>,
        pub event: String,
        pub event_signature: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
    pub struct TransactionReceipt {
        pub from: Address,
        pub to: Address,
        pub contract_address: Option<Address>,
        pub transaction_index: i32,
        pub gas_used: GasUsed,
        pub logs_bloom: String,
        pub block_hash: String,
        pub transaction_hash: String,
        #[serde(default, rename = "logs")]
        pub logs: Vec<Log>,
        pub block_number: i32,
        pub confirmations: i32,
        pub cumulative_gas_used: GasUsed,
        pub status: i32,
        #[serde(rename = "type")]
        pub transaction_type: i32,
        pub byzantium: bool,
        #[serde(default, rename = "events")]
        pub events: Vec<Event>,
    }

    impl TransactionReceipt {
        pub fn load(
            raw_transactions_data: Vec<String>,
        ) -> Result<Vec<TransactionReceipt>, SerdeError> {
            let receipts = raw_transactions_data
                .into_iter()
                .map(|r: String| -> TransactionReceipt {
                    serde_json::from_str(&r.clone()).unwrap()
                })
                .collect();
            Ok(receipts)
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
    pub struct BlockWithTransactionReceipts {
        pub difficulty: String,
        pub extra_data: String,
        pub gas_limit: String,
        pub gas_used: String,
        pub hash: String,
        pub logs_bloom: String,
        pub miner: Address,
        pub mix_hash: H256,
        pub nonce: String,
        pub number: String,
        pub parent_hash: H256,
        pub receipts_root: H256,
        pub sha3_uncles: H256,
        pub size: String,
        pub state_root: H256,
        pub timestamp: String,
        pub total_difficulty: String,
        pub transactions: Vec<Transaction>,
        pub transactions_root: H256,
        pub uncles: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
    pub struct Transaction {
        pub block_hash: H256,
        pub block_number: String,
        pub from: Address,
        pub gas: String,
        pub gas_price: String,
        pub hash: String,
        pub input: String,
        pub nonce: String,
        pub r: String,
        pub s: String,
        pub to: Option<Address>,
        pub transaction_index: String,
        pub v: String,
        pub value: String,
    }
}
