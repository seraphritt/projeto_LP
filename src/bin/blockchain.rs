use chrono :: Utc;
use sha2::{Sha256, Digest};
use std::hash::{Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut hasher = Sha256::new();
    let mut state = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut state);
    let hash_value = state.finish().to_ne_bytes();
    hasher.update(&hash_value);
    let result = hasher.finalize();
    hex::encode(result)
}
struct Block{
    index: u64,
    timestamp: String,
    proof: u64,
    previous_hash: String
}

impl Block{
    pub fn new(proof: u64, index: u64, previous_hash: String)-> Self{
        Block {
            proof,
            index,
            timestamp: Utc::now().to_rfc2822(),
            previous_hash
        }
    }
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.timestamp.hash(state);
        self.proof.hash(state);
        self.previous_hash.hash(state);
    }
}

struct Blockchain{
    chain: Vec<Block>
}
impl Blockchain{
    pub fn new() -> Self{
        Blockchain {
            chain: vec![Block::new(1, 0, String::from("0"))]
        }
    }

    fn create_block(&mut self, proof: u64, previous_hash: String){
        let block = Block::new(proof, self.chain.len() as u64, previous_hash);
        self.chain.push(block);
    }

    fn print_previous_block(&self){
        match self.chain.last(){
            Some(block) => println!("Block: {}\nTime: {}\nProof: {}\nLast block: {}",
                                    block.index,
                                    block.timestamp,
                                    block.proof,
                                    block.previous_hash),
            None => println!("There is no block in the chain")
        }
    }

    fn proof_of_work(&self, last_proof: u64){
        let mut new_proof : u64 = 1;
        let mut check_proof : bool = false;
        let (mut hash_num, mut hash_string): (u64, String);
        let five_zeros = String::from("00000");

        while !check_proof {
            hash_num = new_proof.pow(2)-last_proof.pow(2);
            hash_string = calculate_hash(&hash_num);
            if hash_string[0..5] == five_zeros {
                check_proof = true;
            }
            else{
                new_proof += 1;
            }
        }
    }

    fn is_chain_valid(&self, chain: Vec<Block>) -> bool{
        let mut last_block = &chain[0];
        let mut block;
        let mut block_index : usize = 1;
        let mut last_proof;
        let mut proof;
        let mut hash_string;
        let five_zeros = String::from("00000");
        let mut hash_num: u64;

        while block_index < chain.len(){
            block = &chain[block_index];
            if block.previous_hash != calculate_hash(&last_block) {
                return false;
            } 
            last_proof = last_block.proof;
            proof = block.proof;
            hash_num = proof.pow(2)-(&last_proof).pow(2);

            hash_string = calculate_hash(&hash_num);

            if hash_string[0..5] != five_zeros {
                return false;
            }

            last_block = block;
            block_index += 1;
        }
        return true;
    }
}

fn main(){
    let mut bc = Blockchain::new();
    bc.create_block(1, String::from("abc"));
    bc.print_previous_block();
}