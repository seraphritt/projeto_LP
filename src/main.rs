// // src/main.rs
// mod database;
// mod transaction;
// mod wallet;

// use database::BlockchainDB;
// use transaction::Transaction;
// use wallet::Wallet;

// fn main() {
//     let _txn = Transaction {
//         id: 1,
//         from: "aaa".to_string(),
//         to: "BBB".to_string(),
//         money: 3,
//     };
// }

use env_logger::Env;
use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}
