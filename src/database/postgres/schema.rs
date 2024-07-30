// @generated automatically by Diesel CLI.

diesel::table! {
    user_records (address) {
        address -> Varchar,
        users -> Nullable<Jsonb>,
        contract_owner_address -> Nullable<Varchar>,
        contract_bytecode -> Nullable<Jsonb>,
        random_counter -> Nullable<Int4>,
        method -> Nullable<Varchar>,
    }
}
