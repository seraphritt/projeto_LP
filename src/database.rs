use serde::{Deserialize, Serialize};
use serde_json;
use sled::Db;
use std::fmt;

struct KeyValueStore {
    db: sled::Db,
}

impl KeyValueStore {
    fn open(db_path: String) -> sled::Db {
        sled::open(db_path).unwrap()
    }
    // insert an key-value pair into the database
    fn put(&self, key: &String, value: &String) {
        let _ = self.db.insert(key, value.as_str());
    }

    // return an element by key or None if absent
    fn get(&self, key: &String) -> Option<String> {
        let value = &self.db.get(&key).unwrap();
        let result = match value {
            Some(ivec) => Some(String::from(std::str::from_utf8(&ivec).unwrap())),
            None => None,
        };
        result
    }

    // queries the database by range of keys: [from_key, to_key]
    fn range_scan(&self, from_key: &String, to_key: &String) -> Vec<(String, String)> {
        let mut scanner_result: Vec<(String, String)> = vec![];
        for kv_result in self.db.range(from_key.as_bytes()..=to_key.as_bytes()) {
            let (key, value) = &kv_result.unwrap();
            let key = String::from(std::str::from_utf8(&key).unwrap());
            let value = String::from(std::str::from_utf8(&value).unwrap());
            scanner_result.push((key, value));
        }
        scanner_result
    }

    // queries the database by key prefix, e.g., &"txn:".to_string()
    fn prefix_scan(&self, from_key: &String) -> Vec<(String, String)> {
        let mut scanner_result: Vec<(String, String)> = vec![];
        for kv_result in self.db.scan_prefix(from_key.as_bytes()) {
            let (key, value) = &kv_result.unwrap();
            let key = String::from(std::str::from_utf8(&key).unwrap());
            let value = String::from(std::str::from_utf8(&value).unwrap());
            scanner_result.push((key, value));
        }
        scanner_result
    }

    // delete an element from the database
    fn delete(&self, key: &String) {
        let _ = self.db.remove(key);
    }

    // clear the database
    fn delete_all(&self) {
        let _ = self.db.clear();
    }

    // returns number of entries in database
    fn size(&self) -> usize {
        self.db.len()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: String,
    pub data: String,
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.data)
    }
}

pub struct BlockchainDB {
    kv_store: KeyValueStore,
}

// The key value store is used to store all the Blocks, Transactions, and the Blockchain
// so we define prefixes to make it easy to scan by blocks only, for example
// this is done by using prefix scan of the key value store
const TXN_PREFIX: &str = "txn:";
const BLOCK_PREFIX: &str = "block:";
const CHAIN_PREFIX: &str = "chain:";

impl BlockchainDB {
    pub fn new(db_path: String) -> Self {
        let kv_store = KeyValueStore {
            db: KeyValueStore::open(db_path),
        };
        Self { kv_store: kv_store }
    }

    pub fn save_transaction(&self, txn: &Transaction) {
        let json_string = serde_json::to_string(txn).unwrap();
        let prefix = TXN_PREFIX.to_string();
        let key = prefix + &txn.id;
        self.kv_store.put(&key, &json_string)
    }

    pub fn get_transaction(&self, id: String) -> Option<Transaction> {
        let prefix = TXN_PREFIX.to_string();
        let key = prefix + &id;
        let value = self.kv_store.get(&key);
        let result = match value {
            Some(json_str) => Some(serde_json::from_str(&json_str)).expect("cannot deserialize"),
            None => Ok(None),
        };
        result.unwrap()
    }

    pub fn list_transactions(&self) -> Vec<Transaction> {
        self.kv_store
            .prefix_scan(&TXN_PREFIX.to_string())
            .iter()
            .map(|(_, value)| serde_json::from_str(value.as_str()).unwrap())
            .collect()
    }
}

fn main() {
    let kv_store = KeyValueStore {
        db: KeyValueStore::open("my_db".to_string()),
    };

    // kv_store.put(&"block:Teste1".to_string(), &"99".to_string());
    // kv_store.put(&"block:Teste2".to_string(), &"100".to_string());
    // kv_store.put(&"block:Teste3".to_string(), &"101".to_string());
    // kv_store.put(&"block:Teste4".to_string(), &"102".to_string());

    // println!("{}", kv_store.size());

    // kv_store.delete_all();

    // println!("{}", kv_store.size());

    // let value = match kv_store.get(&"Teste1".to_string()) {
    //     Some(v) => v,
    //     None => "not found".to_string(),
    // };
    // println!("{}", value);

    // kv_store.delete(&"Teste1".to_string());

    // let value = match kv_store.get(&"Teste1".to_string()) {
    //     Some(v) => v,
    //     None => "not found".to_string(),
    // };
    // println!("{}", value);

    // println!(
    //     "{:?}",
    //     kv_store.range_scan(&"txn:".to_string(), &"block:Teste4".to_string())
    // );

    // println!("{:?}", kv_store.prefix_scan(&"block:".to_string()));

    let blockchain_db = BlockchainDB::new("blockchain_db".to_string());
    blockchain_db.save_transaction(&Transaction {
        id: "aaaa-bbb-ccc".to_string(),
        data: "This is a txn!".to_string(),
    });
    blockchain_db.save_transaction(&Transaction {
        id: "xxx-bbb-ccc".to_string(),
        data: "This is a txn!".to_string(),
    });
    blockchain_db.save_transaction(&Transaction {
        id: "000-bbb-ccc".to_string(),
        data: "This is a txn!".to_string(),
    });
    let txn: Transaction = blockchain_db
        .get_transaction("aaaa-bbb-ccc".to_string())
        .unwrap();
    println!("{} {}", &txn.id, &txn.data);

    for txn in blockchain_db.list_transactions() {
        println!("{} {}", &txn.id, &txn.data);
    }
}
