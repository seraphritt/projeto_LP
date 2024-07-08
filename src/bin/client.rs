use std::collections::HashMap;

use blockchaincrypto::transaction::Transaction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:8333")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}
