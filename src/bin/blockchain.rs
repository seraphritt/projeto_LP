use log::info;
use std::thread;
use std::time::Duration;

use blockchaincrypto::blockchain::BlockchainManager;
use blockchaincrypto::transaction::Transaction;

fn main() {
    env_logger::init();

    info!("main thread");

    let manager: BlockchainManager = BlockchainManager::new();
    manager.start_miner_thread();

    let mut i = 1;
    loop {
        thread::sleep(Duration::from_secs(1));

        // inserts a new transaction into prospective block
        manager.add_txn_to_prospective_block(Transaction {
            id: i,
            from: "A".to_string(),
            to: "B".to_string(),
            money: 1,
        });

        // stops the miner, just an example
        if i == 100 {
            manager.stop_miner();
            break;
        }
        info!("blockchain: {:?}", manager.get_blockchain());
        // info!("TRANSACTIONS: {:?}", manager.list_transactions());
        // info!("BLOCKS: {:?}", manager.list_blocks());
        // info!("BLOCK: {:?}", manager.get_block(12));
        i += 1;
    }
}
