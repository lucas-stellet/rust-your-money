#[derive(Debug)]

pub enum Tx {
    // Add variants for storing withdraw/deposit transactions
    Deposit { account: String, amount: u64 },
    Withdraw { account: String, amount: u64 },
}
