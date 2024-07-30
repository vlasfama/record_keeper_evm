-- Your SQL goes here
CREATE TABLE user_records (
    address VARCHAR PRIMARY KEY,
    users JSONB,
    contract_owner_address VARCHAR,
    contract_bytecode JSONB,
    random_counter INT,
    method VARCHAR
);