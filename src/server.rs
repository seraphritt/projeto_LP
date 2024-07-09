use jsonrpc_core::*;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::jsonrpc_core::Value;
use jsonrpc_http_server::ServerBuilder;
use log::info;
use uuid::Uuid;

use crate::blockchain::BlockchainManager;

// https://github.com/paritytech/jsonrpc

#[rpc]
pub trait Rpc {
    /// Adds two numbers and returns a result
    #[rpc(name = "add")]
    fn add(&self, a: u64, b: u64) -> Result<u64>;

    #[rpc(name = "ping")]
    fn ping(&self) -> Result<Value>;

    // #[rpc(name = "add_transaction")]
    // fn add_transaction(&self, data: String) -> Result<Value>;

    #[rpc(name = "list_transactions")]
    fn list_transactions(&self) -> Result<Value>;

    #[rpc(name = "get_transaction")]
    fn get_transaction(&self, txn_id: String) -> Result<Value>;

    #[rpc(name = "get_block")]
    fn get_block(&self, block_id: u64) -> Result<Value>;
}

// API
pub struct RpcImpl {
    blockchain_manager: BlockchainManager,
}
impl Rpc for RpcImpl {
    fn add(&self, a: u64, b: u64) -> Result<u64> {
        Ok(a + b)
    }

    fn ping(&self) -> Result<Value> {
        Ok(Value::String("pong".into()))
    }

    // fn add_transaction(&self, data: String) -> Result<Value> {
    //     let txn_id = Uuid::new_v4().to_string();
    //     self.db.save_transaction(&Transaction {
    //         id: txn_id.clone(),
    //         data: data,
    //     });
    //     Ok(Value::String(txn_id.into()))
    // }

    fn list_transactions(&self) -> Result<Value> {
        Ok(Value::Array(
            self.blockchain_manager
                .list_transactions()
                .iter()
                .map(|txn| Value::String(serde_json::to_string(&txn).unwrap()))
                .collect(),
        ))
    }

    fn get_transaction(&self, txn_id: String) -> Result<Value> {
        let result = match self.blockchain_manager.get_transaction(txn_id) {
            Some(txn) => serde_json::to_string(&txn).unwrap(),
            None => "not found".to_string(),
        };
        Ok(Value::String(result))
    }

    fn get_block(&self, block_idx: u64) -> Result<Value> {
        let result = match self.blockchain_manager.get_block(block_idx) {
            Some(txn) => serde_json::to_string(&txn).unwrap(),
            None => "not found".to_string(),
        };
        Ok(Value::String(result))
    }
}

pub struct BlockchainServer {
    blockchain_manager: BlockchainManager,
}

impl BlockchainServer {
    pub fn start() {
        let blockchain_manager: BlockchainManager = BlockchainManager::new();
        // start miner thread
        blockchain_manager.start_miner_thread();

        // register the RPC API
        let mut io = jsonrpc_core::IoHandler::new();
        io.extend_with(
            RpcImpl {
                blockchain_manager: blockchain_manager,
            }
            .to_delegate(),
        );

        // Start the server
        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&"127.0.0.1:8333".parse().unwrap())
            .unwrap();
        info!("server started");
        server.wait();
    }
}
