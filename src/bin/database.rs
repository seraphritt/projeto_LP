use blockchaincrypto::database::KeyValueStore;
// use blockchaincrypto::transaction::Transaction;

fn main() {
    let kv_store = KeyValueStore {
        db: KeyValueStore::open("./data/my_db".to_string()),
    };

    kv_store.put(&"block:Teste1".to_string(), &"99".to_string());
    kv_store.put(&"block:Teste2".to_string(), &"100".to_string());
    kv_store.put(&"block:Teste3".to_string(), &"101".to_string());
    kv_store.put(&"block:Teste4".to_string(), &"102".to_string());

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

    println!(
        "{:?}",
        kv_store.range_scan(&"txn:".to_string(), &"block:Teste4".to_string())
    );

    // println!("{:?}", kv_store.prefix_scan(&"block:".to_string()));

    // *******
    // let blockchain_db = BlockchainDB::new("./data/blockchain_db".to_string());
    // blockchain_db.save_transaction(&Transaction {
    //     id: "aaaa-bbb-ccc".to_string(),
    //     data: "This is a txn!".to_string(),
    // });
    // blockchain_db.save_transaction(&Transaction {
    //     id: "xxx-bbb-ccc".to_string(),
    //     data: "This is a txn!".to_string(),
    // });
    // blockchain_db.save_transaction(&Transaction {
    //     id: "000-bbb-ccc".to_string(),
    //     data: "This is a txn!".to_string(),
    // });
    // let txn: Transaction = blockchain_db
    //     .get_transaction("aaaa-bbb-ccc".to_string())
    //     .unwrap();
    // println!("{} {}", &txn.id, &txn.data);

    // for txn in blockchain_db.list_transactions() {
    //     println!("{} {}", &txn.id, &txn.data);
    // }
}
