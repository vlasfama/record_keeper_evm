pub mod start;

use crate::commands::start::StartCmd;
use async_trait::async_trait;
use structopt::StructOpt;

#[async_trait]
pub trait CoinhavenScannerCommand {
    /// Returns the result of the command execution.
    async fn execute(self);
}

#[derive(Debug, StructOpt)]
pub enum Command {
    ///Start the record keeper service
    #[structopt(name = "start")]
    Start(StartCmd),
}

impl Command {
    /// Wrapper around `StructOpt::from_args` method.
    pub fn from_args() -> Self {
        <Self as StructOpt>::from_args()
    }
}

#[async_trait]
impl CoinhavenScannerCommand for Command {
    async fn execute(self) {
        match self {
            Self::Start(command) => command.execute().await,
        }
    }
}
