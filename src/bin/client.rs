use jsonrpsee::proc_macros::rpc;
use jsonrpsee_http_client::HttpClientBuilder;

#[rpc(client)]
pub trait Rpc {
    // async methods throw error when returning String
    #[method(name = "add")]
    fn add(&self, a: u64, b: u64) -> Result<u64, Error>;

    #[method(name = "ping")]
    fn ping(&self) -> Result<String, Error>;

    #[method(name = "list_transactions")]
    fn list_transactions(&self) -> Result<Vec<String>, Error>;

    #[method(name = "get_transaction")]
    fn get_transaction(&self, txn_id: String) -> Result<String, Error>;

    #[method(name = "get_block")]
    fn get_block(&self, block_id: u64) -> Result<String, Error>;

    #[method(name = "get_balance")]
    fn get_balance(&self, wallet_id: String)->Result<i64, Error>;

    #[method(name = "create_transaction")]
    fn create_transaction(&self, from: String, to: String, money: i64)->Result<String, Error>;
}

#[tokio::main]
async fn main() {
    // Build RPC client
    let client = HttpClientBuilder::default()
        // .set_headers(headers)
        .build("http://127.0.0.1:8333")
        .unwrap();

    // Examples of RPC client invocation
    let result: String = client.ping().await.unwrap();
    println!("ping: {}\n", result);

    let result = client.list_transactions().await.unwrap();
    println!("list_transactions: {:?}\n", result);

    let txn_id = "5".to_string();
    let result = client.get_transaction(txn_id).await.unwrap();
    println!("transaction: {}\n", result);

    let balance = client.get_balance("A".to_string()).await.unwrap();
    println!("balance: {}\n", balance);

}