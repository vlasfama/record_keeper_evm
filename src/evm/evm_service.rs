use anyhow::Error;
use revm::Evm;
use std::fs::File;
use std::io::Read;
use std::result::Result;

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
        let abi_path = "./build/UserRecord.abi";
        let _abi = load_file(abi_path).expect("Unable to load ABI");
        let bytecode_path = "./build/UserRecord.bin";
        let bytecode = load_file(bytecode_path).expect("Unable to load bytecode");
        let bytecode = hex::decode(bytecode.trim()).expect("Decoding failed");
        let mut evm = Evm::builder().build();

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
