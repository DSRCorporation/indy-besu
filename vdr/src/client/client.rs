use std::collections::HashMap;

use crate::{
    client::{
        implementation::web3::{client::Web3Client, contract::Web3Contract},
        Client, Contract, ContractConfig, ContractSpec, Status, StatusResult,
        Transaction, TransactionType,
    },
    error::{VdrError, VdrResult},
    signer::Signer,
};

pub struct LedgerClient {
    chain_id: u64,
    client: Box<dyn Client>,
    contracts: HashMap<String, Box<dyn Contract>>,
}

impl LedgerClient {
    pub fn new(
        chain_id: u64,
        node_address: &str,
        contract_configs: &Vec<ContractConfig>,
        signer: Option<Box<dyn Signer + 'static + Send + Sync>>,
    ) -> VdrResult<LedgerClient> {
        let client = Web3Client::new(node_address, signer)?;
        let contracts = Self::init_contracts(&client, &contract_configs)?;
        Ok(LedgerClient {
            chain_id,
            client: Box::new(client),
            contracts,
        })
    }

    pub async fn ping(&self) -> VdrResult<StatusResult> {
        Ok(StatusResult { status: Status::Ok })
    }

    pub async fn sign_transaction(&self, transaction: &Transaction) -> VdrResult<Transaction> {
        return self.client.sign_transaction(&transaction).await;
    }

    pub async fn submit_transaction(&self, transaction: &Transaction) -> VdrResult<Vec<u8>> {
        match transaction.type_ {
            TransactionType::Read => self.client.call_transaction(&transaction).await,
            TransactionType::Write => self.client.submit_transaction(&transaction).await,
        }
    }

    pub fn contract(&self, name: &str) -> VdrResult<&Box<dyn Contract>> {
        self.contracts.get(name).ok_or(VdrError::Unexpected)
    }

    pub async fn get_transaction_receipt(&self, hash: &[u8]) -> VdrResult<String> {
        self.client.get_transaction_receipt(hash).await
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn init_contracts(
        client: &Web3Client,
        contract_configs: &[ContractConfig],
    ) -> VdrResult<HashMap<String, Box<dyn Contract>>> {
        let mut contracts: HashMap<String, Box<dyn Contract>> = HashMap::new();
        for contract_config in contract_configs {
            let contract_spec = Self::read_contract_spec(&contract_config.spec_path)?;
            let contract = Web3Contract::new(client, &contract_config.address, &contract_spec)?;
            contracts.insert(contract_spec.name.clone(), Box::new(contract));
        }
        Ok(contracts)
    }

    fn read_contract_spec(spec_path: &str) -> VdrResult<ContractSpec> {
        let contract_spec = std::fs::read_to_string(spec_path).map_err(|err| {
            VdrError::Common(format!("Unable to read contract spec file. Err: {:?}", err))
        })?;
        let contract_spec: ContractSpec = serde_json::from_str(&contract_spec).map_err(|err| {
            VdrError::Common(format!(
                "Unable to parse contract specification. Err: {:?}",
                err.to_string()
            ))
        })?;
        Ok(contract_spec)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::BasicSigner;

    pub const CHAIN_ID: u64 = 1337;
    pub const NODE_ADDRESS: &'static str = "http://127.0.0.1:8545";
    pub const ACCOUNT: &'static str = "0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5";
    pub const PRIVATE_KEY: &'static str =
        "8bbbb1b345af56b560a5b20bd4b0ed1cd8cc9958a16262bc75118453cb546df7";
    pub const DID_REGISTRY_ADDRESS: &'static str = "0x0000000000000000000000000000000000003333";
    pub const DID_REGISTRY_SPEC_PATH: &'static str = "/Users/artem/indy-ledger/smart_contracts/artifacts/contracts/did/DidRegistry.sol/DidRegistry.json";
    pub const SCHEMA_REGISTRY_ADDRESS: &'static str = "0x0000000000000000000000000000000000005555";
    pub const SCHEMA_REGISTRY_SPEC_PATH: &'static str = "/Users/artem/indy-ledger/smart_contracts/artifacts/contracts/cl/SchemaRegistry.sol/SchemaRegistry.json";

    fn contracts() -> Vec<ContractConfig> {
        vec![
            ContractConfig {
                address: DID_REGISTRY_ADDRESS.to_string(),
                spec_path: DID_REGISTRY_SPEC_PATH.to_string(),
            },
            ContractConfig {
                address: SCHEMA_REGISTRY_ADDRESS.to_string(),
                spec_path: SCHEMA_REGISTRY_SPEC_PATH.to_string(),
            },
        ]
    }

    fn signer() -> BasicSigner {
        let mut signer = BasicSigner::new().unwrap();
        signer.add_key(ACCOUNT, PRIVATE_KEY).unwrap();
        signer
    }

    pub fn client() -> LedgerClient {
        let contracts = contracts();
        let signer = signer();
        LedgerClient::new(CHAIN_ID, NODE_ADDRESS, &contracts, Some(Box::new(signer))).unwrap()
    }

    mod create {
        use super::*;

        #[test]
        fn create_client_test() {
            client();
        }
    }
}
