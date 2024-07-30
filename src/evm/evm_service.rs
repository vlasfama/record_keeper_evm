use anyhow::Error;
use ethers_core::abi::{Function, Hash, Param, ParamType, Token};
use ethers_core::types::{Call, H256};
use revm::inspectors::NoOpInspector;
use revm::interpreter::{CallInputs, CallScheme, CallValue, Contract};
use revm::primitives::{
    AccountInfo, Address, Bytecode, Bytes, Env, ExecutionResult, FixedBytes, U256,
};
use revm::{inspector_handle_register, Evm, EvmBuilder, InMemoryDB, StateBuilder};
use std::fs::File;
use std::io::Read;
use std::result::Result;
use std::str::FromStr;
use std::sync::Arc;

use crate::database::provider::DatabaseProvider;
use crate::model::UserInfo;

#[derive(Clone)]
pub struct EvmHandler<D>
where
    D: DatabaseProvider,
{
    database: Arc<D>,
}

impl<D> EvmHandler<D>
where
    D: DatabaseProvider,
{
    pub fn new(database: Arc<D>) -> Self {
        EvmHandler { database }
    }

    pub fn excute_contract(&mut self) -> Result<(), Error> {
        let bytecode_path = "./build/UserRecords.bin";
        let bytecode = load_file(bytecode_path).expect("Unable to load bytecode");
        let bytecode = hex::decode(bytecode.trim()).expect("Decoding failed");
        println!("the byte code:{:?}", bytecode);
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
        //Account info and pass balance to it.
        let info = AccountInfo {
            balance,
            ..Default::default()
        };

        db.insert_account_info(address, info);

        let mut evm = Evm::builder()
            .with_ref_db(db)
            .with_external_context(NoOpInspector)
            .append_handler_register(inspector_handle_register)
            .build();

        let commit = evm.transact_commit()?;
        match commit {
            ExecutionResult::Success { output, .. } => {
                println!("Transaction executed successfully: {:?}", output);
            }
            ExecutionResult::Revert { output, .. } => {
                println!("Transaction reverted: {:?}", output);
            }
            ExecutionResult::Halt { reason, .. } => {
                println!("Transaction halted: {:?}", reason);
            }
        }

        Ok(())
    }

    pub fn call_add_user_entry(&mut self, user: UserInfo) -> Result<(), Error> {
        let data = self.encode_add_user_entry(user);
        let mut db = InMemoryDB::default();
        let balance = U256::from(9999999);
        let address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".parse()?;

        let info = AccountInfo {
            balance,
            ..Default::default()
        };

        db.insert_account_info(address, info);

        let mut evm = Evm::builder()
            .with_ref_db(db)
            .with_external_context(NoOpInspector)
            .append_handler_register(inspector_handle_register)
            .build();

        let call_inputs = CallInputs {
            input: data.clone(),
            return_memory_offset: 0..32, // Adjust as necessary
            gas_limit: 3000000,          // Example gas limit
            bytecode_address: Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e")?, // Contract address
            target_address: Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e")?, // Contract address
            caller: Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e")?,
            value: CallValue::default(), // No value being transferred
            scheme: CallScheme::Call,
            is_static: false,
            is_eof: false,
        };

        let commit = evm.transact_commit()?;
        match commit {
            ExecutionResult::Success { output, .. } => {
                println!("Transaction executed successfully: {:?}", output);
            }
            ExecutionResult::Revert { output, .. } => {
                println!("Transaction reverted: {:?}", output);
            }
            ExecutionResult::Halt { reason, .. } => {
                println!("Transaction halted: {:?}", reason);
            }
        }

        Ok(())
    }

    pub fn encode_add_user_entry(&self, user: UserInfo) -> Bytes {
        let function = Function {
            name: "addUserEntry".into(),
            inputs: vec![
                Param {
                    name: "_userId".into(),
                    kind: ParamType::Uint(8),
                    internal_type: None,
                },
                Param {
                    name: "_userAge".into(),
                    kind: ParamType::Uint(8),
                    internal_type: None,
                },
                Param {
                    name: "_userCreditBalance".into(),
                    kind: ParamType::Uint(16),
                    internal_type: None,
                },
                Param {
                    name: "_userIsMember".into(),
                    kind: ParamType::Bool,
                    internal_type: None,
                },
            ],
            outputs: vec![Param {
                name: "".into(),
                kind: ParamType::Bool,
                internal_type: None,
            }],
            constant: None,
            state_mutability: Default::default(),
        };

        let vec = vec![
            Token::Uint(user.user_id.into()),
            Token::Uint(user.user_age.into()),
            Token::Uint(user.user_credit_balance.into()),
            Token::Bool(user.user_is_member),
        ];
        let tokens = vec;

        let data = function.encode_input(&tokens).expect("Encoding failed");
        Bytes::from(data)
    }
}
fn load_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
