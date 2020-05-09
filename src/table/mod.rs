mod account;
mod category;
mod transaction;
use account::Account;
use category::Category;
use transaction::Transaction;

#[derive(Clone, Debug)]
pub struct Table {
    pub accounts: Vec<Account>,
    pub categories: Vec<Category>,
    pub transactions: Vec<Transaction>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            accounts: Vec::new(),
            categories: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub fn build_accounts(&mut self, contents: String) {
        let mut accounts: Vec<Account> = Vec::new();
        for line in contents.split("\n") {
            if line.is_empty() {
                break;
            }
            let mut cells = line.split(",");
            let id = match accounts.is_empty() {
                true => 0,
                false => accounts.len(),
            };
            accounts.push(Account::new(id, cells.next(), cells.next()));
        }
        self.accounts = accounts;
    }

    pub fn build_categories(&mut self, contents: String) {
        let mut categories: Vec<Category> = Vec::new();
        for line in contents.split("\n") {
            if line.is_empty() {
                break;
            }
            let mut cells = line.split(",");
            let id = match categories.is_empty() {
                true => 0,
                false => categories.len(),
            };
            categories.push(Category::new(id, cells.next(), cells.next(), cells.next()));
        }
        self.categories = categories;
    }

    pub fn build_transactions(&mut self, contents: String) {
        let mut transactions: Vec<Transaction> = Vec::new();
        for line in contents.split("\n") {
            if line.is_empty() {
                break;
            }
            let mut cells = line.split(",");
            let id = match transactions.is_empty() {
                true => 0,
                false => transactions.len(),
            };
            transactions.push(Transaction::new(
                id,
                cells.next(),
                cells.next(),
                cells.next(),
                cells.next(),
                cells.next(),
            ));
        }
        self.transactions = transactions;
    }

    pub fn display(&self, table: &str) {
        match table {
            "accounts" => {
                println!("===== ACCOUNTS =====");
                println!();
                for acc in self.accounts.as_slice() {
                    println!("{}", acc);
                }
                println!();
            }
            "categories" => {
                println!("===== CATEGORIES =====");
                println!();
                for cat in self.categories.as_slice() {
                    println!("{}", cat);
                }
                println!();
            }
            "transactions" => {
                println!("===== TRANSACTIONS =====");
                println!();
                for tran in self.transactions.as_slice() {
                    println!("{}", tran);
                }
                println!();
            }
            _ => (),
        }
    }
}
