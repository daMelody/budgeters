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

enum TableType {
    Account,
    Category,
    Transaction,
}

impl Table {
    /// build empty Table struct
    pub fn new() -> Table {
        Table {
            accounts: Vec::new(),
            categories: Vec::new(),
            transactions: Vec::new(),
        }
    }

    /// build the Table.accounts Vec from file contents
    ///     - creates new Vec<Account> if file contents are empty
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

    /// build the Table.categories Vec from file contents
    ///     - creates new Vec<Category> if file contents are empty
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

    /// build the Table.transactions Vec from file contents
    ///     - creates new Vec<Transaction> if file contents are empty
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

    pub fn display(&self, table: TableType) {
        match table {
            TableType::Account => {
                println!("===== ACCOUNTS =====");
                println!();
                for acc in self.accounts.as_slice() {
                    println!("{}", acc);
                }
                println!();
            }
            TableType::Category => {
                println!("===== CATEGORIES =====");
                println!();
                for cat in self.categories.as_slice() {
                    println!("{}", cat);
                }
                println!();
            }
            TableType::Transaction => {
                println!("===== TRANSACTIONS =====");
                println!();
                for tran in self.transactions.as_slice() {
                    println!("{}", tran);
                }
                println!();
            }
            _ => (),
        }

        // for matching CLI input
        const ACCOUNT: String = String::from("acc");
        const CATEGORY: String = String::from("cat");
        let TRANSACTION: String = String::from("tra");

        pub fn list(table: &Table, args: &Vec<String>) {
            // expect args to have have a type argument
            if args.len() != 2 {
                return;
            }
            let table_type = match args.get(1) {
                Some(st) => st,
                None => return,
            };
            let account = String::from("acc");
            let category = String::from("cat");
            let transaction = String::from("tra");
            match table_type {
                &account => table.display(TableType::Account),
                &category => table.display(TableType::Category),
                &transaction => table.display(TableType::Transaction),
            };
        }

        pub fn add(args: &Vec<String>) {}
        pub fn edit(args: &Vec<String>) {}

        pub fn search(args: &Vec<String>) {}
        pub fn delete(args: &Vec<String>) {}
    }
}
