-- Your SQL goes here
CREATE TABLE user_records (
    id SERIAL PRIMARY KEY,
    users JSONB NOT NULL,
    random_counter INT NOT NULL
);