mod account;
use account::Account;
mod category;
use category::Category;
mod transaction;
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
        self.accounts = accounts;
    }

    pub fn build_categories(&mut self, contents: String) {
        let mut categories: Vec<Category> = Vec::new();
        for line in contents.split("\n") {
            let mut cells = line.split(",");
            categories.push(Category::new(cells.next(), cells.next(), cells.next()));
        }
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
        self.transactions = transactions;
    }
}
