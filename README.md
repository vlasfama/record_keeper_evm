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


## API Documentation

This section details the available API routes in our application. Each route is designed to handle specific functionalities such as deploying contracts, creating users, and updating user information.

### API Routes Table

| Endpoint          | Method | Description                                           |
|-------------------|--------|-------------------------------------------------------|
| `/deploy_contract`| POST   | Initiates the deployment of a contract.               |
| `/create_user`    | POST   | Creates a new user in the system with provided details.|
| `/update_user`    | PUT    | Updates existing user information.                     |

