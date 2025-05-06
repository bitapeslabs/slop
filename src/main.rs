pub mod p2p;
pub mod utils;
use tokio::{self, task};
use utils::{config::get_config, logger::Logger};

#[tokio::main]
async fn main() {
    Logger::info("Hello");

    let config = get_config().unwrap_or_else(|err| Logger::error_panic(err));

    println!("config: {}", config.node.peers[0]);
}
