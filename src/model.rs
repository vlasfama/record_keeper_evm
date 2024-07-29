use std::{collections::HashMap, sync::Arc};

use async_std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct User {
    user_id: u8,
    user_age: u8,
    user_credit_balance: u16,
    user_is_member: bool,
}

#[derive(Debug)]
pub struct UserRecords {
    users: Arc<Mutex<HashMap<String, User>>>,
    random_counter: Arc<Mutex<u32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EthereumAccount {
    address: String,
    balance: u64,
    nonce: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EthereumTransaction {
    from: String,
    to: String,
    value: u64,
    gas_price: u64,
    gas_limit: u64,
    nonce: u64,
    data: Option<String>,
}
