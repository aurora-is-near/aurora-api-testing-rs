use ethers_core::abi::{Abi, AbiError};
use ethers_core::types::Bytes;
use std::env;
use std::path::{Path, PathBuf};

extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use dao::dao::helpers::SerdeError;
use std::str::FromStr;

pub fn hex_string_to_i32(s: String) -> i32 {
    i32::from_str_radix(&s[2..s.len()], 16).unwrap()
}

pub fn hex_string_to_i64(s: String) -> i64 {
    i64::from_str_radix(&s[2..s.len()], 16).unwrap()
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
