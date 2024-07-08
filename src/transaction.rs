use serde::{Deserialize, Serialize};
use sled::transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: u64,
    pub from: String,
    pub to: String,
    pub money: i64,
    // pub locktime: f64,
}

impl Transaction {
    pub fn consume_transaction(&self, address: &String) -> i64 {
        if self.from == *address {
            return -self.money;
        } else if self.to == *address {
            return self.money;
        }
        0
    }
}
