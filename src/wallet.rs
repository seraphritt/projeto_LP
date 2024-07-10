pub struct Wallet {
    pub wallet_address: String,
    pub balance: i64,
}

impl Wallet {
    pub fn new(address: String) -> Self {
        Self {
            wallet_address: address,
            balance: 0,
        }
    }
}
