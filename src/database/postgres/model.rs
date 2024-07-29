use crate::database::postgres::schema::*;
use diesel::prelude::*;
use diesel::{prelude::*, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
#[derive(Serialize, Deserialize, Queryable, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub user_age: i16,
    pub user_credit_balance: i32,
    pub user_is_member: bool,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
#[diesel(table_name = crate::schema::user_records)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRecord {
    pub id: i32,
    pub users: Value,
    pub random_counter: i32,
}
