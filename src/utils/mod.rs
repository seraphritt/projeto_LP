use chrono::Utc;
use sha2::{Digest, Sha256};
use std::hash::{Hash, Hasher};

const DIFFICULTY: usize = 3; // dificuldade do proof of work da blockchain

#[derive(Debug)]
pub struct Block {
    index: u64,
    timestamp: String,
    pub proof: u64,
    previous_hash: String,
    content: Content,
}
#[derive(Clone, Debug)]
pub struct Content {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}
impl Block {
    pub fn new(proof: u64, index: u64, previous_hash: String, content: Content) -> Self {
        Block {
            index,
            timestamp: Utc::now().to_rfc2822(),
            proof,
            previous_hash,
            content,
        }
    }
    pub fn get_sender(&self) -> String {
        return self.content.sender.clone();
    }
    pub fn get_receiver(&self) -> String {
        return self.content.receiver.clone();
    }
    pub fn get_amount(&self) -> f64{
        return self.content.amount.clone();
    }
}
impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.content.sender.hash(state);
        self.content.receiver.hash(state);
        self.content.amount.to_string().hash(state);
        self.index.hash(state);
        self.timestamp.hash(state);
        self.proof.hash(state);
        self.previous_hash.hash(state);
    }
}
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let ct: Content = Content {
            sender: "Bank".to_string(),
            receiver: "Alisson".to_string(),
            amount: 700.00,
        };
        let genesis_block = Block::new(1, 0, String::from("0"), ct);
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    pub fn create_block(&mut self, proof: u64, previous_hash: String, ct: Content) {
        let index = self.chain.len() as u64;
        let new_block = Block::new(proof, index, previous_hash, ct);
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
    pub fn is_valid_proof(&self, last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let guess_hash = Blockchain::hash_string(&guess);
        // println!("Proof{}", guess_hash);
        guess_hash.starts_with(&String::from("0").repeat(DIFFICULTY))
    }
    pub fn is_chain_valid(&self) -> bool {
        let mut previous_block = &self.chain[0];
        let mut index = 1;

        while index < self.chain.len() {
            let current_block = &self.chain[index];
            if current_block.previous_hash != Blockchain::hash_block(previous_block) {
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
    pub fn hash_block(block: &Block) -> String {
        let block_string = format!(
            "{}{}{}{}",
            block.index, block.timestamp, block.proof, block.previous_hash
        );
        Blockchain::hash_string(&block_string)
    }
    pub fn hash_string(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

// fn main() {
//     // o blockchain já vem com um bloco padrão, chamado genesis block
//     let mut blockchain = Blockchain::new();
//     for _ in 0..4 {
//         let last_proof = blockchain.chain.last().unwrap().proof;
//         let proof = blockchain.proof_of_work(last_proof);
//         let previous_hash = Blockchain::hash_block(blockchain.chain.last().unwrap());
//         blockchain.create_block(proof, previous_hash);
//     }
//     // exemplo de criação de um bloco que não passou pelo proof of work, fazendo assim com que a blockchain fique invalidada
//     blockchain.create_block(10, String::from("4f607389fe5630ad233e04a316e12bf864329551f19c180de9805a3e337de57f"));
//     // como tem o derive(Debug), ele consegue imprimir cada bloco da blockchain
//     for block in &blockchain.chain {
//         println!("{:?}", block);
//     }
//     println!("Is blockchain valid? {}", blockchain.is_chain_valid());
//     println!("{:?}", blockchain.chain)
// }