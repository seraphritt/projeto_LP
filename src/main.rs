use blockchaincrypto::utils::{Blockchain, Content};

fn main() {
    // o blockchain já vem com um bloco padrão, chamado genesis block
    let content: Content = Content {
    amount: 30.0,
    receiver : "Rafael".to_string(),
    sender : "Nicolau".to_string(),
    };
    let mut blockchain = Blockchain::new();
    for _ in 0..5 {
        let last_proof = blockchain.chain.last().unwrap().proof;
        let proof = blockchain.proof_of_work(last_proof);
        let previous_hash = Blockchain::hash_block(blockchain.chain.last().unwrap());
        blockchain.create_block(proof, previous_hash, content.clone());
    }
    // exemplo de criação de um bloco que não passou pelo proof of work, fazendo assim com que a blockchain fique invalidada
    // blockchain.create_block(10, String::from("4f607389fe5630ad233e04a316e12bf864329551f19c180de9805a3e337de57f"), content.clone());
    // como tem o derive(Debug), ele consegue imprimir cada bloco da blockchain
    for block in &blockchain.chain {
        // println!("{:?}", block);
        println!("Sender: {:?}", block.get_sender());
        println!("Receiver: {:?}", block.get_receiver());
        println!("Amount: {:?}", block.get_amount());
    }
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());
    // println!("{:?}", blockchain.chain)
}