# Record_keeper

This project, built using Rust, PostgreSQL, and Solidity, utilizes REVM (Rust Ethereum Virtual Machine) for maintaining user records.

## Getting Started

We use PostgreSQL for our database. Ensure you have the PostgreSQL driver installed before compiling the project and that PostgreSQL is running.

```
git clone git@github.com:vlasfama/record_keeper.git
cd record_keeper
cargo build
```
## Running project
```
./target/debug/record_keeper start --address 127.0.0.1:5400 --database-url postgres://postgres:postgres@localhost:5432 --pool-size 3
```
