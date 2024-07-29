use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Config {
    #[structopt(long = "address", env = "API_ADDRESS")]
    pub address: String,
}
