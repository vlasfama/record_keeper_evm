use std::{collections::HashMap, sync::Arc};

use async_std::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct UserInfo {
    pub user_id: u8,
    pub user_age: u8,
    pub user_credit_balance: u16,
    pub user_is_member: bool,
}

#[derive(Debug)]
pub struct UserRecords {
    pub address: String,
    pub users: HashMap<String, UserInfo>,
    pub contract_owner_address: String,
    pub contract_bytecode: Value,
    pub random_counter: u32,
    pub method: String,
}
