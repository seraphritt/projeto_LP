// use actix_web::web::block;
// use serde::{Deserialize, Serialize};
use serde_json;
// use rand::Rng;
// use sled::Db;
// use std::fmt;

use crate::blockchain::{Block, Blockchain};
use crate::transaction::Transaction;
// use crate::wallet::Wallet;

pub struct KeyValueStore {
    pub db: sled::Db,
}

impl KeyValueStore {
    pub fn open(db_path: String) -> sled::Db {
        sled::open(db_path).unwrap()
    }
    // insert an key-value pair into the database
    pub fn put(&self, key: &String, value: &String) {
        let _ = self.db.insert(key, value.as_str());
    }

    // return an element by key or None if absent
    pub fn get(&self, key: &String) -> Option<String> {
        let value = &self.db.get(&key).unwrap();
        let result = match value {
            Some(ivec) => Some(String::from(std::str::from_utf8(&ivec).unwrap())),
            None => None,
        };
        result
    }

    // queries the database by range of keys: [from_key, to_key]
    pub fn range_scan(&self, from_key: &String, to_key: &String) -> Vec<(String, String)> {
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
    pub fn prefix_scan(&self, from_key: &String) -> Vec<(String, String)> {
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
    pub fn delete(&self, key: &String) {
        let _ = self.db.remove(key);
    }

    // clear the database
    pub fn delete_all(&self) {
        let _ = self.db.clear();
    }

    // returns number of entries in database
    pub fn size(&self) -> usize {
        self.db.len()
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
const BLOCKCHAIN_KEY: &str = "chain:";

impl BlockchainDB {
    pub fn new(db_path: String) -> Self {
        let kv_store = KeyValueStore {
            db: KeyValueStore::open(db_path),
        };
        Self { kv_store: kv_store }
    }

    // transactions
    pub fn save_transaction(&self, txn: &Transaction) {
        let json_string = serde_json::to_string(txn).unwrap();
        let prefix = TXN_PREFIX.to_string();
        let key = prefix + &txn.id.to_string();
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

    // blocks
    pub fn save_block(&self, block: Block) {
        let json_string = serde_json::to_string(&block).unwrap();
        let prefix = BLOCK_PREFIX.to_string();
        let key = prefix + &block.index.to_string();
        self.kv_store.put(&key, &json_string)
    }
    pub fn get_block(&self, block_idx: u64) -> Option<Block> {
        let prefix = BLOCK_PREFIX.to_string();
        let key = prefix + &block_idx.to_string();
        let value = self.kv_store.get(&key);
        let result = match value {
            Some(json_str) => Some(serde_json::from_str(&json_str)).expect("cannot deserialize"),
            None => Ok(None),
        };
        result.unwrap()
    }
    pub fn list_blocks(&self) -> Vec<Block> {
        self.kv_store
            .prefix_scan(&BLOCK_PREFIX.to_string())
            .iter()
            .map(|(_, value)| serde_json::from_str(value.as_str()).unwrap())
            .collect()
    }
    // blockchain
    pub fn save_blockchain(&self, blockchain: Blockchain) {
        let json_string = serde_json::to_string(&blockchain).unwrap();
        // doesn't need a prefix as it is singleton
        self.kv_store.put(&BLOCKCHAIN_KEY.to_string(), &json_string)
    }

    pub fn get_blockchain(&self) -> Option<Blockchain> {
        let key = BLOCKCHAIN_KEY.to_string();
        let value = self.kv_store.get(&key);
        let result = match value {
            Some(json_str) => Some(serde_json::from_str(&json_str)).expect("cannot deserialize"),
            None => Ok(None),
        };
        result.unwrap()
    }
}
