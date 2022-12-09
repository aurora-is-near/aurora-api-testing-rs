use ethers_core::{
    abi::{Abi, Tokenize, Token, TokenizableItem},
    types::{Address, Bytes},
};
use ethers_signers::LocalWallet;
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Provider, Http};
use ethers_contract::ContractFactory;
use std::convert::TryFrom;
use std::error::Error;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;


pub struct SmartContract {
    pub abi: Abi,
    pub bytecode: Bytes
}

impl SmartContract {
    pub fn new(abi: Abi, bytecode: Bytes) -> SmartContract {
        SmartContract {
            abi,
            bytecode
        }
    }

    pub async fn deploy<T: Tokenize + TokenizableItem  + std::fmt::Debug>(
        &self,
        rpc_url: &str,
        private_key: &str,
        args: Vec<T>,
    ) -> Result<String, Box<dyn Error>>  {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();
        let _ = tracing::subscriber::set_global_default(subscriber);
        let wallet = private_key[2..private_key.len()].parse::<LocalWallet>()?;
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let client = SignerMiddleware::new(provider.clone(), wallet.clone());
        let client = std::sync::Arc::new(client);
        let factory = ContractFactory::new(self.abi.clone(), self.bytecode.clone(), client);
        info!("args: {:?}", args);
        let deployer = factory.deploy(args)?;
        let deployed_contract = deployer.clone().legacy().send().await?;
        Ok(deployed_contract.address().to_string())
    }
}

