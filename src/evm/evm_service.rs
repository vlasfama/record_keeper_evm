use anyhow::Error;
use ethers_core::abi::Hash;
use ethers_core::types::H256;
use revm::interpreter::Contract;
use revm::primitives::{AccountInfo, Address, Bytecode, Bytes, Env, FixedBytes, U256};
use revm::{Evm, EvmBuilder, InMemoryDB, StateBuilder};
use std::fs::File;
use std::io::Read;
use std::result::Result;
use std::str::FromStr;

use crate::database::provider::DatabaseProvider;

#[derive(Clone)]
pub struct EvmHandler<D>
where
    D: DatabaseProvider,
{
    database: D,
}

impl<D> EvmHandler<D>
where
    D: DatabaseProvider,
{
    pub fn new(database: D) -> Self {
        EvmHandler { database }
    }

    pub fn excute_contract(&mut self) -> Result<(), Error> {
        let bytecode_path = "./build/UserRecord.bin";
        let bytecode = load_file(bytecode_path).expect("Unable to load bytecode");
        let bytecode = hex::decode(bytecode.trim()).expect("Decoding failed");
        let bytes = Bytes::from(bytecode);
        let mut db = InMemoryDB::default();

        let balance = U256::from(9999999);
        let address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".parse()?;

        //create contract instance
        let contract = Contract {
            input: bytes.clone(),
            bytecode: Bytecode::LegacyRaw(bytes),
            hash: Some(FixedBytes::<32>::ZERO),
            target_address: Address::ZERO,
            bytecode_address: None,
            caller: Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e")?,
            call_value: U256::ZERO,
        };

        let info = AccountInfo {
            balance,
            ..Default::default()
        };
        db.insert_account_info(address, info);
        let mut env = Env::default();

        Ok(())
    }

    pub fn create_transaction() -> Result<(), Error> {
        Ok(())
    }

    pub fn call_info() -> Result<(), Error> {
        Ok(())
    }
}
fn load_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
