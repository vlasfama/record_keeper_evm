mod commands;
use commands::Command;
pub mod api;
pub mod database;
pub mod evm;
pub mod model;

#[tokio::main]
async fn main() {
    match Command::from_args() {
        Command::Start(cmd) => cmd.execute().await,
    }
}
