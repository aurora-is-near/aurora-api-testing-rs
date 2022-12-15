use ethers_contract::ContractFactory;
use ethers_core::{
    abi::{Abi, Token, TokenizableItem, Tokenize},
    types::{Address, Bytes},
};
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Http, Provider};
use ethers_signers::{LocalWallet, Signer};
use std::convert::TryFrom;
use std::error::Error;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub struct SmartContract {
    pub abi: Abi,
    pub bytecode: Bytes,
}

impl SmartContract {
    pub fn new(abi: Abi, bytecode: Bytes) -> SmartContract {
        SmartContract { abi, bytecode }
    }

    pub async fn deploy<T: Tokenize>(
        &self,
        rpc_url: &str,
        private_key: &str,
        chain_id: u64,
        args: Option<T>,
    ) -> Result<Address, Box<dyn Error>> {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();
        let _ = tracing::subscriber::set_global_default(subscriber);
        let wallet: LocalWallet = private_key[2..private_key.len()]
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id);
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let client = SignerMiddleware::new(provider.clone(), wallet);
        let client = std::sync::Arc::new(client);
        let factory = ContractFactory::new(self.abi.clone(), self.bytecode.clone(), client);
        let deployer = factory.deploy(args.unwrap())?;
        let deployed_contract = deployer.clone().legacy().send().await?;
        Ok(deployed_contract.address())
    }
}
