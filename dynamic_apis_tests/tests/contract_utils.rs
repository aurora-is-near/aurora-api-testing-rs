use ethers_contract::{Contract, ContractFactory};
use ethers_core::{
    abi::{Abi, Tokenize},
    types::{Address, Bytes, TransactionReceipt, H256},
};
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Http, Provider};
use ethers_signers::{LocalWallet, Signer};
use jsonrpsee_core::client::TransportSenderT;
use std::convert::TryFrom;
use std::error::Error;

pub struct SignerWallet<'a> {
    pub rpc_url: &'a str,
    pub private_key: &'a str,
    pub chain_id: u64,
}

impl SignerWallet<'_> {
    pub fn new<'a>(rpc_url: &'a str, private_key: &'a str, chain_id: u64) -> SignerWallet<'a> {
        SignerWallet {
            rpc_url,
            private_key,
            chain_id,
        }
    }

    pub fn create(&self) -> Result<SignerMiddleware<Provider<Http>, LocalWallet>, Box<dyn Error>> {
        let wallet: LocalWallet = self.private_key[2..self.private_key.len()]
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(self.chain_id);
        let provider = Provider::<Http>::try_from(self.rpc_url.clone())?;
        Ok(SignerMiddleware::new(provider.clone(), wallet))
    }
}

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
        args: Option<T>,
        signer: SignerMiddleware<Provider<Http>, LocalWallet>,
    ) -> Result<Address, Box<dyn Error>> {
        let client = std::sync::Arc::new(signer);
        let factory = ContractFactory::new(self.abi.clone(), self.bytecode.clone(), client);
        let deployer = factory.deploy(args.unwrap())?;
        let deployed_contract = deployer.clone().legacy().send().await?;
        Ok(deployed_contract.address())
    }

    pub async fn call<T: Tokenize, D: Tokenize + ethers_core::abi::Tokenizable>(
        self,
        address: Address,
        method: &str,
        args: Option<T>,
        signer: SignerMiddleware<Provider<Http>, LocalWallet>,
    ) -> Result<D, Box<dyn Error>> {
        let contract = Contract::new(address, self.abi.clone(), signer);
        let value: D = contract
            .method::<_, D>(method, args.unwrap())?
            .call()
            .await?;
        Ok(value)
    }

    pub async fn submit<T: Tokenize>(
        self,
        address: Address,
        method: &str,
        args: Option<T>,
        signer: SignerMiddleware<Provider<Http>, LocalWallet>,
    ) -> Result<Option<TransactionReceipt>, Box<dyn Error>> {
        let contract = Contract::new(address, self.abi.clone(), signer);
        let call = contract.method::<_, H256>(method, args.unwrap())?;
        let pending_tx = call.gas_price(21000);
        let pending_tx = pending_tx.send().await?;
        let receipt = pending_tx.confirmations(1).await?;
        Ok(receipt)
    }
}
