use blockchaincrypto::server::BlockchainServer;
use env_logger::Env;
use log::{debug, error, info, trace, warn};

fn main() {
    // Initialize the logger with default settings
    env_logger::init();
    BlockchainServer::start()
}
