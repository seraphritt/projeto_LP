mod database;
mod transaction;

use chrono::Utc;
use uuid::Uuid;
use sha2::{Digest, Sha256};
use std::{fs::File, hash::{Hash, Hasher}, io::{BufReader, Read}};
use serde::{Serialize, Deserialize};
use std::io::{Write, Error};
use std::fs;
use rand::Rng;
use database::BlockchainDB;
use transaction::Transaction;

type Id = fn() -> String;
const CREATE_ID: Id = || Uuid::new_v4().to_string();

trait JsonSerDe: Serialize {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // error
    // fn from_json(json_str: &str) -> Self {
    //     serde_json::from_str(&json_str).unwrap()
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader{
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: String,
    pub difficulty: u32,
    pub nonce: u32
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub proof: u64,
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub transaction_counter: u32
}

impl JsonSerDe for Block {}

impl Block {
    pub fn new(proof: u64, index: u64, previous_hash: String, timestamp: String) -> Self {
        let block_header : BlockHeader = BlockHeader{
            previous_hash,
            timestamp,
            merkle_root: String::from(""),
            difficulty: 0,
            nonce: 0
        };

        Block {
            index,
            proof,
            header: block_header,
            transactions: vec![],
            transaction_counter: 0,
        }
    }

    pub fn add_transaction(&mut self, txn: Transaction) {
        self.transactions.push(txn);
        self.transaction_counter = self.transactions.len() as u32;
    }
}

// TODO: add all fields, including block header!!!
impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        // self.timestamp.hash(state);          
        self.proof.hash(state);
        // self.previous_hash.hash(state);
    }
}

struct BlockBuilder {
    // Block Header
    previous_hash: String,
    merkle_root: String,
    timestamp: String, //TODO: convert to u64
    difficulty: u32,
    nonce: u32,
    // Block Body
    index: u64,
    proof: u64,
    transactions: Vec<Transaction>,
    transaction_counter: u32,
}

impl BlockBuilder {
    pub fn new() -> BlockBuilder {
        Self{
            index: 0,
            proof: 0,
            previous_hash: "".to_string(),
            merkle_root: "".to_string(),
            timestamp: "".to_string(), //TODO: convert to u64
            difficulty: 0,
            nonce: 0,
            transactions: vec![],
            transaction_counter: 0,           
        }
    }
    pub fn index(mut self, i: u64) -> Self {
        self.index = i;
        self
    }
    pub fn previous_hash(mut self, ph: String) -> Self {
        self.previous_hash = ph.clone();
        self
    }
    pub fn merkle_root(mut self, mr: String) -> Self {
        self.merkle_root = mr;
        self
    }
    pub fn timestamp(mut self, ts: String) -> Self {
        self.timestamp = ts;
        self
    }
    pub fn difficulty(mut self, d: u32) -> Self {
        self.difficulty = d;
        self
    }

    pub fn nonce(mut self, n: u32) -> Self {
        self.nonce = n;
        self
    }
    pub fn proof(mut self, p: u64) -> Self {
        self.proof = p;
        self
    }

    pub fn transactions(mut self, txns: &Vec<Transaction>) -> BlockBuilder {
        self.transactions = txns.clone();
        self.transaction_counter = self.transactions.len() as u32;
        self
    }
    pub fn build(&self) -> Block {
        Block {
            index: self.index,
            proof: self.proof,
            header: BlockHeader{
                previous_hash: self.previous_hash.clone(),
                merkle_root: self.merkle_root.clone(),
                timestamp: self.timestamp.clone(), //TODO: convert to u64
                difficulty: self.difficulty,
                nonce: self.nonce,
            },
            transactions: self.transactions.clone(),
            transaction_counter: self.transaction_counter, 
        }
    }
}

const DIFFICULTY: usize = 3; // dificuldade do proof of work da blockchain

#[derive(Serialize, Deserialize, Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        // let genesis_block = Block::new(1, 0, String::from("0"), Utc::now().to_rfc2822());
        let genesis_block = BlockBuilder::new()
                                                .proof(1)
                                                .index(0)
                                                .previous_hash(String::from("0"))
                                                .timestamp(Utc::now().to_rfc2822())
                                                .build();
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    pub fn create_block(&mut self, proof: u64, previous_hash: String) {
        let index = self.chain.len() as u64;
        // let new_block = Block::new(proof, index, previous_hash, Utc::now().to_rfc2822());
        // TODO: add other fields
        let new_block = BlockBuilder::new()
                                            .proof(proof)
                                            .index(index)
                                            .previous_hash(previous_hash)
                                            .timestamp(Utc::now().to_rfc2822())
                                            .build();
        self.chain.push(new_block);
    }
    pub fn print_previous_block(&self) {
        if let Some(block) = self.chain.last() {
            println!("Previous Block: {:?}", block);
        } else {
            println!("Blockchain is empty.");
        }
    }
    pub fn proof_of_work(&self, last_proof: u64) -> u64 {
        let mut proof = 0;
        while !self.is_valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }
    fn is_valid_proof(&self, last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let guess_hash = Blockchain::hash_string(&guess);
        println!("Proof{}", guess_hash);
        guess_hash.starts_with(&String::from("0").repeat(DIFFICULTY))
    }
    pub fn is_chain_valid(&self) -> bool {
        let mut previous_block = &self.chain[0];
        let mut index = 1;

        for current_block in &self.chain {
            if current_block.header.previous_hash != Blockchain::hash_block(previous_block) {
                return false;
            }
            if !self.is_valid_proof(previous_block.proof, current_block.proof) {
                return false;
            }

            previous_block = current_block;
            index += 1;
        }
        true
    }
    fn hash_block(block: &Block) -> String {
        let block_string = format!(
            "{}{}{}{}",
            block.index, block.header.timestamp, block.proof, block.header.previous_hash
        );
        Blockchain::hash_string(&block_string)
    }
    fn hash_string(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}


struct BlockchainManager{
    database: BlockchainDB,
    prospective_block: Block,
    blockchain: Blockchain,    
}

fn generate_random_u64()-> u64{
    let mut rng = rand::thread_rng();
    rng.gen()
}

impl BlockchainManager{
    fn new() -> Self{
        Self{
            database: BlockchainDB::new("blockchain_db".to_string()),
            prospective_block: BlockBuilder::new().build(), //TODO: fill in parameters
            blockchain: Blockchain::new(),
        }
    }

    fn create_new_transaction(&self, inputs: Vec<String>, outputs: Vec<String>) -> Transaction {
        Transaction{
            id: generate_random_u64(),
            input_counter: inputs.len() as i32,
            inputs,
            output_counter: outputs.len() as i32,
            outputs,
            locktime: 33.0,
        }
    }

    fn is_transaction_valid(transaction: Transaction) -> bool{
        true // TODO: implement it!
    }
}

//TODO: move to struct? 
pub fn to_json(block: &Block) -> String {
    serde_json::to_string(block).unwrap()
}
//TODO: move to struct? 
pub fn from_json(json_str: &str) -> Block {
    serde_json::from_str(&json_str).unwrap()
}
//TODO: move to struct? 
pub fn write_to_file(file_path: &str, json_str: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(json_str.as_bytes())?;
    Ok(())
}
//TODO: move to struct? 
pub fn read_from_file(file_path: &str) -> Result<String, Error> {
    // Open the file in read-only mode
    let file = File::open(file_path)?;

    // Wrap the file in a BufReader to read it line by line
    let mut reader = BufReader::new(file);

    // Read the entire contents into a String
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(contents)
}
fn main() {
    // o blockchain já vem com um bloco padrão, chamado genesis block
    let mut blockchain = Blockchain::new();
    for _ in 0..4 {
        let last_proof = blockchain.chain.last().unwrap().proof;
        let proof = blockchain.proof_of_work(last_proof);
        let previous_hash = Blockchain::hash_block(blockchain.chain.last().unwrap());
        blockchain.create_block(proof, previous_hash);
    }
    // exemplo de criação de um bloco que não passou pelo proof of work, fazendo assim com que a blockchain fique invalidada
    blockchain.create_block(10, String::from("4f607389fe5630ad233e04a316e12bf864329551f19c180de9805a3e337de57f"));
    // como tem o derive(Debug), ele consegue imprimir cada bloco da blockchain
    for block in &blockchain.chain {
        println!("{:?}", block);
    }
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());
    println!("{:?}", blockchain.chain)
}


// Unit Tests
#[test]
fn test_new_block() {
    let index = 1;
    let proof = 42;
    let previous_hash = String::from("aff80323039339");
    let timestamp = String::from("2024-07-03");
    let new_block = Block::new(proof, index, previous_hash, timestamp);
    // header
    assert_eq!(new_block.header.previous_hash, String::from("aff80323039339"));
    assert_eq!(new_block.header.difficulty, 0);
    assert_eq!(new_block.header.merkle_root, String::from(""));
    assert_eq!(new_block.header.nonce, 0);
    // block
    assert_eq!(new_block.index, 1);
    assert_eq!(new_block.proof, 42);
    assert_eq!(new_block.transaction_counter, 0);
    assert_eq!(new_block.transactions.len(), 0);
}
#[test]
fn test_block_serialization_as_json() {
    let index = 1;
    let proof = 42;
    let previous_hash = String::from("aff80323039339");
    let timestamp = String::from("2024-07-03");
    let block = Block::new(proof, index, previous_hash, timestamp);

    let serialized = to_json(&block);
    let expected = String::from("{\"index\":1,\"proof\":42,\"header\":{\"previous_hash\":\"aff80323039339\",\"merkle_root\":\"\",\"timestamp\":\"2024-07-03\",\"difficulty\":0,\"nonce\":0},\"transactions\":[],\"transaction_counter\":0}");
    assert_eq!(expected, serialized);
}
#[test]
fn test_block_deserialization_as_json() {
    let serialized_string = "{\"index\":1,\"proof\":42,\"header\":{\"previous_hash\":\"aff80323039339\",\"merkle_root\":\"\",\"timestamp\":\"2024-07-03\",\"difficulty\":0,\"nonce\":0},\"transactions\":[],\"transaction_counter\":0}";
    let block: Block = from_json(serialized_string);

    // header
    assert_eq!(block.header.previous_hash, String::from("aff80323039339"));
    assert_eq!(block.header.difficulty, 0);
    assert_eq!(block.header.merkle_root, String::from(""));
    assert_eq!(block.header.nonce, 0);
    // block
    assert_eq!(block.index, 1);
    assert_eq!(block.proof, 42);
    assert_eq!(block.transaction_counter, 0);
    assert_eq!(block.transactions.len(), 0);
}
#[test]
fn test_write_and_read_block_to_disk() {
    let index = 1;
    let proof = 42;
    let previous_hash = String::from("aff80323039339");
    let timestamp = String::from("2024-07-03");
    let block = Block::new(proof, index, previous_hash, timestamp);

    let json_str = to_json(&block);
    let filename = "test_block.json";
    write_to_file(&filename, &json_str);

    let serialized_json = read_from_file(&filename).unwrap();
    let serialized_block = from_json(&serialized_json);

    // header
    assert_eq!(serialized_block.header.previous_hash, String::from("aff80323039339"));
    assert_eq!(serialized_block.header.difficulty, 0);
    assert_eq!(serialized_block.header.merkle_root, String::from(""));
    assert_eq!(serialized_block.header.nonce, 0);
    // block
    assert_eq!(serialized_block.index, 1);
    assert_eq!(serialized_block.proof, 42);
    assert_eq!(serialized_block.transaction_counter, 0);
    assert_eq!(serialized_block.transactions.len(), 0);

    // match fs::remove_file(filename) {
    //     Ok(()) => println!("File '{}' successfully deleted.", filename),
    //     Err(err) => eprintln!("Error deleting file '{}': {}", filename, err),
    // }
}

#[test]
fn test_block_builder() {
    let txn = Transaction{
        id: generate_random_u64(),
        input_counter: 1,
        inputs: vec!["input".to_string()],
        output_counter: 1,
        outputs: vec!["out".to_string()],
        locktime: 3.3,
    };
    let block = BlockBuilder::new()
                        .index(1)
                        .proof(2)
                        .previous_hash("aaf05261616".to_string())
                        .merkle_root("ccff88213".to_string()) // build after inserting txns
                        .timestamp("Saturday".to_string())
                        .difficulty(9)
                        .nonce(3)
                        .transactions(&vec![txn.clone()])
                        .build();
    //TODO: multi-format lines
    let expected = "{\"index\":1,\"proof\":2,\"header\":{\"previous_hash\":\"aaf05261616\",\"merkle_root\":\"ccff88213\",\"timestamp\":\"Saturday\",\"difficulty\":9,\"nonce\":3},\"transactions\":[{\"input_counter\":1,\"inputs\":[\"input\"],\"output_counter\":1,\"outputs\":[\"out\"],\"locktime\":3}],\"transaction_counter\":1}";
    // assert_eq!(expected, to_json(&block));
    assert_eq!(expected, block.to_json())
}



