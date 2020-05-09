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
            let mut cells = line.split(",");
            accounts.push(Account::new(cells.next(), cells.next()));
        }
        accounts.remove(accounts.len() - 1);
        self.accounts = accounts;
    }

    pub fn build_categories(&mut self, contents: String) {
        let mut categories: Vec<Category> = Vec::new();
        for line in contents.split("\n") {
            let mut cells = line.split(",");
            categories.push(Category::new(cells.next(), cells.next(), cells.next()));
        }
        categories.remove(categories.len() - 1);
        self.categories = categories;
    }

    pub fn build_transactions(&mut self, contents: String) {
        let mut transactions: Vec<Transaction> = Vec::new();
        for line in contents.split("\n") {
            let mut cells = line.split(",");
            transactions.push(Transaction::new(
                cells.next(),
                cells.next(),
                cells.next(),
                cells.next(),
                cells.next(),
            ));
        }
        transactions.remove(transactions.len() - 1);
        self.transactions = transactions;
    }

    pub fn display(&self, table: &str) {
        match table {
            "accounts" => {
                for acc in self.accounts.as_slice() {
                    println!("{}", acc);
                }
                println!();
            }
            "categories" => {
                for cat in self.categories.as_slice() {
                    println!("{}", cat);
                }
                println!();
            }
            "transactions" => {
                for tran in self.transactions.as_slice() {
                    println!("{}", tran);
                }
                println!();
            }
            _ => (),
        }
    }
}
