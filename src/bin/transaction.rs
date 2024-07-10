use blockchaincrypto::transaction::Transaction;

fn main(){

    let txns = vec![
        Transaction{ id: 1, from: "A".to_string(), to: "B".to_string(), money: 10},
        Transaction{ id: 2, from: "B".to_string(), to: "C".to_string(), money: 3} ];

    let balance = txns.into_iter()
                                                  .map(|x|  x.consume_transaction(&"B".to_string()))
                                                  .sum::<i64>();

    println!("balance = {}", balance);
}