#[derive(Debug)]
struct Account {
    id: u32,
    holder: String,
    balance: i32,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] } // implicitly returned
    }
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

fn print_account(account: Account) {
    println!("{:#?}", account)
}

fn main() {
    // let bank = Bank::new();
    let account = Account::new(1,String::from("Onyinye"));

    let list_accounts = vec![account];

    // println!("{:#?}", bank);
    // print_account(account);
}
