
struct Wallet{
    ladscoin_address: String,
    // TODO: private and public key
    balance: f64,

}

impl Wallet{
    fn new(address: &String) -> Self{
        Self {
            ladscoin_address: address,
            balance: 0.0,
        }
    }

    fn transfer_money(from_address: &String, to_address: &String, ammount: f64) -> String {
        
    }
}