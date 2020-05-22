mod account;
mod category;
mod transaction;
use account::Account;
use category::Category;
use std::collections::HashMap;
use transaction::Transaction;

#[derive(Clone, Debug)]
pub struct Table {
    pub accounts: Vec<Account>,
    pub categories: Vec<Category>,
    pub transactions: Vec<Transaction>,
}

pub enum TableType {
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
            accounts.push(Account::from_cls(cells.next(), cells.next()));
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
            categories.push(Category::from_cls(cells.next(), cells.next(), cells.next()));
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
            transactions.push(Transaction::from_cls(
                cells.next(),
                cells.next(),
                cells.next(),
                cells.next(),
                cells.next(),
            ));
        }
        self.transactions = transactions;
    }

    /// display the list of TableType
    pub fn display(&self, table: TableType) {
        match table {
            TableType::Account => {
                println!("===== ACCOUNTS =====");
                println!();
                for acc in self.accounts.iter() {
                    println!("{}", acc);
                }
                println!();
            }
            TableType::Category => {
                println!("===== CATEGORIES =====");
                println!();
                for cat in self.categories.iter() {
                    println!("{}", cat);
                }
                println!();
            }
            TableType::Transaction => {
                println!("===== TRANSACTIONS =====");
                println!();
                for tran in self.transactions.iter() {
                    println!("{}", tran);
                }
                println!();
            }
        }
    }

    /// returns an array of String corresponding to the three TableTypes
    const TABLE_TYPES: [&'static str; 3] = ["acc", "cat", "tra"];

    pub fn list(&self, arg: &String) {
        // expect args to have a type argument
        if arg.is_empty() {
            return;
        }
        if arg == &Table::TABLE_TYPES[0] {
            self.display(TableType::Account);
        } else if arg == &Table::TABLE_TYPES[1] {
            self.display(TableType::Category);
        } else if arg == &Table::TABLE_TYPES[2] {
            self.display(TableType::Transaction);
        }
    }

    pub fn search(&self, arg: &String) {
        // expect args to have a type argument
        if arg.is_empty() {
            return;
        }
        Transaction::search(&self.transactions, arg);
    }

    /* require mutable Table */

    pub fn add(&mut self, arg: &String) {
        if arg.is_empty() {
            return;
        }
        if arg == &Table::TABLE_TYPES[0] {
            self.accounts.push(Account::new());
        } else if arg == &Table::TABLE_TYPES[1] {
            self.categories.push(Category::new());
        } else if arg == &Table::TABLE_TYPES[2] {
            self.transactions.push(Transaction::new());
        }
    }

    pub fn edit(&mut self, arg: &String) {
        if arg.is_empty() {
            return;
        }
        if arg == &Table::TABLE_TYPES[0] {
            let index = Account::find(&self.accounts);
            if index >= 0 {
                if let Some(acc) = self.accounts.get_mut(index as usize) {
                    println!("{}", acc);
                    acc.edit();
                }
            }
        } else if arg == &Table::TABLE_TYPES[1] {
            let index = Category::find(&self.categories);
            if index >= 0 {
                if let Some(cat) = self.categories.get_mut(index as usize) {
                    println!("{}", cat);
                    cat.edit();
                }
            }
        } else if arg == &Table::TABLE_TYPES[2] {
            let index = Transaction::find(&self.transactions);
            if index >= 0 {
                if let Some(tra) = self.transactions.get_mut(index as usize) {
                    println!("{}", tra);
                    tra.edit();
                }
            }
        }
    }

    pub fn delete(&mut self, arg: &String) {
        if arg.is_empty() {
            return;
        }
        if arg == &Table::TABLE_TYPES[0] {
            let index = Account::find(&self.accounts);
            if index >= 0 {
                self.accounts.remove(index as usize);
            }
        } else if arg == &Table::TABLE_TYPES[1] {
            let index = Category::find(&self.categories);
            if index >= 0 {
                self.categories.remove(index as usize);
            }
        } else if arg == &Table::TABLE_TYPES[2] {
            let index = Transaction::find(&self.transactions);
            if index >= 0 {
                self.transactions.remove(index as usize);
            }
        }
    }

    pub fn update(&mut self) {
        let mut account_map: HashMap<&str, f32> = HashMap::new();
        let mut category_map: HashMap<&str, f32> = HashMap::new();
        // recalculate totals of Accounts and Categories
        for tra in &self.transactions {
            let tra_amount = tra.get_amount();
            let acc_name = tra.get_account();
            let acc_value = match account_map.get(acc_name) {
                Some(last) => last + tra_amount,
                None => tra_amount,
            };
            account_map.insert(acc_name, acc_value);
            let cat_name = tra.get_category();
            let cat_value = match category_map.get(cat_name) {
                Some(last) => last + tra_amount,
                None => tra_amount,
            };
            category_map.insert(cat_name, cat_value);
        }
        // iterate through Accounts and update Value fields
        for acc in self.accounts.iter_mut() {
            let rounded = (*account_map.get(acc.get_name()).unwrap() * 100.0).round() / 100.0;
            acc.set_value(rounded);
        }
        // iterate through Category and update Actual fields
        for cat in self.categories.iter_mut() {
            let rounded = (*category_map.get(cat.get_name()).unwrap() * 100.0).round() / 100.0;
            cat.set_actual(rounded);
        }
        self.display(TableType::Account);
        self.display(TableType::Category);
    }

    pub fn roll(&mut self, _arg: &String) {} // TODO:

    pub fn to_cls(&self, path: &String) -> String {
        if path.ends_with("Account.cls") {
            let mut accounts = String::new();
            for acc in &self.accounts {
                accounts.push_str(&Account::to_cls(acc));
            }
            accounts
        } else if path.ends_with("Category.cls") {
            let mut categories = String::new();
            for cat in &self.categories {
                categories.push_str(&Category::to_cls(cat));
            }
            categories
        } else if path.ends_with("Transaction.cls") {
            let mut transactions = String::new();
            for tra in &self.transactions {
                transactions.push_str(&Transaction::to_cls(tra));
            }
            transactions
        } else {
            eprintln!("Unexpected filename while writing to cls");
            String::new()
        }
    }
}
