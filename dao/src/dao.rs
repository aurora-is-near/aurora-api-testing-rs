pub mod models;
#[macro_use]

pub mod helpers {
    pub use ethereum_types::Address;
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
}
