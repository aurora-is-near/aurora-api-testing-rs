pub mod models;
use crate::dao::helpers::{Address, Event, GasUsed, Log, TransactionReceipt};
use crate::dao::models::{get_db_connection, TestRun, TestTask};

mod helpers {
    pub use ethereum_types::Address;

    pub struct GasUsed {
        gas_type: String,
        hex: String,
    }

    pub struct Log {
        transaction_index: i32,
        block_number: i32,
        transaction_hash: String,
        address: Address,
        topics: Vec<String>,
        data: String,
        log_index: i32,
        block_hash: String,
    }

    pub struct Event {
        transaction_index: i32,
        block_number: i32,
        transaction_hash: String,
        address: Address,
        topics: Vec<String>,
        data: String,
        log_index: i32,
        block_hash: String,
        args: Vec<String>,
        event: String,
        event_signature: String,
    }

    pub struct TransactionReceipt {
        pub from: Address,
        pub to: Address,
        pub contract_address: Option<Address>,
        pub transaction_index: i32,
        pub gas_used: GasUsed,
        pub logs_bloom: String,
        pub block_hash: String,
        pub transaction_hash: String,
        pub logs: Vec<Log>,
        pub block_number: i32,
        pub confirmations: i32,
        pub cumulative_gas_used: GasUsed,
        pub status: i32,
        pub transaction_type: i32,
        pub byzantium: bool,
        pub events: Vec<Event>,
    }

    impl TransactionReceipt {
        pub fn load() {}
    }
}
