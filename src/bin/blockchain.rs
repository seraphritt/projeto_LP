use log::{debug, info};
use std::thread;
use std::time::Duration;

use blockchaincrypto::blockchain::BlockchainManager;
use blockchaincrypto::transaction::Transaction;

fn main() {
    // // o blockchain já vem com um bloco padrão, chamado genesis block
    // let mut blockchain = Blockchain::new();
    // for _ in 0..4 {
    //     let last_proof = blockchain.chain.last().unwrap().proof;
    //     let proof = Blockchain::proof_of_work(last_proof);
    //     let previous_hash = Blockchain::hash_block(blockchain.chain.last().unwrap());
    //     blockchain.create_block(proof, previous_hash);
    // }
    // // exemplo de criação de um bloco que não passou pelo proof of work, fazendo assim com que a blockchain fique invalidada
    // blockchain.create_block(10, String::from("4f607389fe5630ad233e04a316e12bf864329551f19c180de9805a3e337de57f"));
    // // como tem o derive(Debug), ele consegue imprimir cada bloco da blockchain
    // for block in &blockchain.chain {
    //     println!("{:?}", block);
    // }
    // println!("Is blockchain valid? {}", blockchain.is_chain_valid());
    // println!("{:?}", blockchain.chain)

    // Initialize the logger with default settings
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
