-- Your SQL goes here
CREATE TABLE users (
    user_id SMALLINT PRIMARY KEY,
    user_age SMALLINT NOT NULL,
    user_credit_balance SMALLINT NOT NULL,
    user_is_member BOOLEAN NOT NULL
);