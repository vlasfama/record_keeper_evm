use diesel::{prelude::*, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Serialize, Deserialize, Queryable, Debug)]
#[diesel(table_name = crate::schema::user_records)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRecord {
    pub address: String,
    pub users: Value,
    pub contract_owner_address: String,
    pub contract_bytecode: Value,
    pub random_counter: i32,
    pub method: String,
}
