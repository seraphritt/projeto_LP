use actix_web::web::block;
use blockchaincrypto::blockchain::BlockchainManager;
use jsonrpsee::core::{async_trait, RpcResult};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::ServerBuilder;
use std::net::SocketAddr;

#[rpc(server)]
pub trait Rpc {
    // async methods throw error when returning String
    #[method(name = "add")]
    fn add(&self, a: u64, b: u64) -> RpcResult<u64>;

    #[method(name = "ping")]
    fn ping(&self) -> RpcResult<String>;

    #[method(name = "list_transactions")]
    fn list_transactions(&self) -> RpcResult<Vec<String>>;

    #[method(name = "get_transaction")]
    fn get_transaction(&self, txn_id: String) -> RpcResult<String>;

    #[method(name = "get_block")]
    fn get_block(&self, block_id: u64) -> RpcResult<String>;

    #[method(name = "get_balance")]
    fn get_balance(&self, wallet_id: String)->RpcResult<i64>;

    #[method(name = "create_transaction")]
    fn create_transaction(&self, from: String, to: String, money: i64)->RpcResult<String>;
}

pub struct RpcServerImpl {
    blockchain_manager: BlockchainManager,
}

#[async_trait]
impl RpcServer for RpcServerImpl {
    fn add(&self, a: u64, b: u64) -> RpcResult<u64> {
        Ok(a + b)
    }

    fn ping(&self) -> RpcResult<String> {
        Ok("pong".to_string().into())
    }

    fn list_transactions(&self) -> RpcResult<Vec<String>> {
        Ok(self
            .blockchain_manager
            .list_transactions()
            .iter()
            .map(|txn| serde_json::to_string(&txn).unwrap())
            .collect())
    }

    fn get_transaction(&self, txn_id: String) -> RpcResult<String> {
        let result = match self.blockchain_manager.get_transaction(txn_id) {
            Some(txn) => serde_json::to_string(&txn).unwrap(),
            None => "".to_string(),
        };
        Ok(result)
    }

    fn get_block(&self, block_idx: u64) -> RpcResult<String> {
        let result = match self.blockchain_manager.get_block(block_idx) {
            Some(txn) => serde_json::to_string(&txn).unwrap(),
            None => "".to_string(),
        };
        Ok(result)
    }

    fn create_transaction(&self, from: String, to: String, money: i64)->RpcResult<String>{
        let result = self.blockchain_manager.create_transaction(from, to, money);
        Ok(result)
    }

    fn get_balance(&self, wallet_id: String)->RpcResult<i64>{
        let result = self.blockchain_manager.get_balance(wallet_id);
        Ok(result)
    }
}

pub async fn server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
    // create blockchain_manager and starts miner thread
    let blockchain_manager = BlockchainManager::new();
    blockchain_manager.start_miner_thread();

    let server = ServerBuilder::default()
        .build("127.0.0.1:8333")
        .await
        .unwrap();
    let addr = server.local_addr().unwrap();
    let server_handle = server.start(
        RpcServerImpl {
            blockchain_manager: blockchain_manager, // pass ownership to blockchain_manager
        }
        .into_rpc(),
    );

    let join_handle = tokio::spawn(server_handle.stopped());
    (addr, join_handle)
}

#[tokio::main]
async fn main() {
    let (_server_addr, server_handle) = server().await;
    // Wait for the server to stop
    server_handle.await.unwrap();
}