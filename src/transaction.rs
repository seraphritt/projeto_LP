use serde::{Deserialize, Serialize};
// use sled::transaction;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: u64,
    pub from: String,
    pub to: String,
    pub money: i64,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction {}: {} -> {} (${})",
            self.id, self.from, self.to, self.money
        )
    }
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
