pub struct Wallet {
    pub ladscoin_address: String,
    // TODO: private and public key
    pub balance: i64,
}

impl Wallet {
    pub fn new(address: &String) -> Self {
        Self {
            ladscoin_address: "abc".to_string(),
            balance: 0,
        }
    }
}
