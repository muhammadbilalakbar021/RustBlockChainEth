fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
    pub privatekey: String,
    pub to_account_key: String,
    pub amount: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            privatekey: get_nth_arg(1),
            to_account_key: get_nth_arg(2),
            amount: get_nth_arg(3),
        }
    }
}
