/**
 * Bank
 *  - name
 *  - accounts
 *
 * => createAccount()
 * => getAllAccounts()
 * => getBankSummary()
 *
 * Account
 * - id
 * - balance
 * - holder
 *
 * => deposit()
 * => withdraw()
 * => summary()
 */

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
    active: bool,
}

#[derive(Debug)]
struct Bank {
    name: String,
    accounts: Vec<Account>,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
            active: false,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} account balance: {}", self.holder, self.balance)
    }

    fn toggle_activation(&mut self) {
        self.active = !self.active;
    }
}

impl Bank {
    fn new(name: String) -> Self {
        Bank {
            name,
            accounts: vec![],
        }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn main() {
    let mut bank = Bank::new(String::from("first_bank"));
    let mut account = Account::new(1, String::from("Henry"));

    account.toggle_activation();

    account.deposit(500);
    account.withdraw(200);
    account.withdraw(100);

    println!("{:#?}", account.summary());

    bank.add_account(account);

    println!("{:#?}", bank);
    // println!("{:#?}", account);
}
