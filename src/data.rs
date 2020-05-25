use account::Account;
use category::Category;
use prettytable::{Cell, Row, Table};
use std::collections::HashMap;
use transaction::Transaction;

mod account;
mod category;
mod transaction;

#[derive(Clone, Debug)]
pub struct Data {
    pub accounts: Vec<Account>,
    pub categories: Vec<Category>,
    pub transactions: Vec<Transaction>,
}

pub enum DataType {
    Account,
    Category,
    Transaction,
}

impl Data {
    /// build empty Data struct
    pub fn new() -> Data {
        Data {
            accounts: Vec::new(),
            categories: Vec::new(),
            transactions: Vec::new(),
        }
    }

    /// build the data.accounts Vec from file contents
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

    /// build the data.categories Vec from file contents
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

    /// build the data.transactions Vec from file contents
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

    /// display the list of DataType
    pub fn display(&self, data: DataType) {
        match data {
            DataType::Account => {
                println!("===== ACCOUNTS =====");
                let mut account_table = Table::new();
                account_table.add_row(Row::new(vec![
                    Cell::new("id"),
                    Cell::new("name"),
                    Cell::new("value"),
                ]));
                for acc in self.accounts.iter() {
                    account_table.add_row(Row::new(vec![
                        Cell::new(&acc.get_simple_id()),
                        Cell::new(&acc.get_name()),
                        Cell::new(&acc.get_value().to_string()),
                    ]));
                }
                account_table.printstd();
            }
            DataType::Category => {
                println!("===== CATEGORIES =====");
                let mut category_table = Table::new();
                category_table.add_row(Row::new(vec![
                    Cell::new("id"),
                    Cell::new("name"),
                    Cell::new("expected"),
                    Cell::new("actual"),
                ]));
                for cat in self.categories.iter() {
                    category_table.add_row(Row::new(vec![
                        Cell::new(&cat.get_simple_id()),
                        Cell::new(&cat.get_name()),
                        Cell::new(&cat.get_expected().to_string()),
                        Cell::new(&cat.get_actual().to_string()),
                    ]));
                }
                category_table.printstd();
            }
            DataType::Transaction => {
                println!("===== TRANSACTIONS =====");
                let mut transaction_table = Table::new();
                transaction_table.add_row(Row::new(vec![
                    Cell::new("id"),
                    Cell::new("date"),
                    Cell::new("amount"),
                    Cell::new("account"),
                    Cell::new("category"),
                    Cell::new("description"),
                ]));
                for tra in self.transactions.iter() {
                    transaction_table.add_row(Row::new(vec![
                        Cell::new(&tra.get_simple_id()),
                        Cell::new(&tra.get_date()),
                        Cell::new(&tra.get_amount().to_string()),
                        Cell::new(&tra.get_account()),
                        Cell::new(&tra.get_category()),
                        Cell::new(&tra.get_description()),
                    ]));
                }
                transaction_table.printstd();
            }
        }
    }

    /// returns an array of String corresponding to the three DataTypes
    const DATA_TYPES: [&'static str; 3] = ["acc", "cat", "tra"];

    pub fn list(&self, arg: &String) {
        // expect args to have a type argument
        if arg.is_empty() {
            return;
        }
        if arg == &Data::DATA_TYPES[0] {
            self.display(DataType::Account);
        } else if arg == &Data::DATA_TYPES[1] {
            self.display(DataType::Category);
        } else if arg == &Data::DATA_TYPES[2] {
            self.display(DataType::Transaction);
        }
    }

    pub fn search(&self, arg: &String) {
        // expect args to have a type argument
        if arg.is_empty() {
            return;
        }
        Transaction::search(&self.transactions, arg);
    }

    /* require mutable Data */

    pub fn add(&mut self, arg: &String) {
        if arg.is_empty() {
            return;
        }
        if arg == &Data::DATA_TYPES[0] {
            self.accounts.push(Account::new());
        } else if arg == &Data::DATA_TYPES[1] {
            self.categories.push(Category::new());
        } else if arg == &Data::DATA_TYPES[2] {
            self.transactions.push(Transaction::new());
        }
    }

    pub fn edit(&mut self, arg: &String) {
        if arg.is_empty() {
            return;
        }
        if arg == &Data::DATA_TYPES[0] {
            let index = Account::find(&self.accounts);
            if index >= 0 {
                if let Some(acc) = self.accounts.get_mut(index as usize) {
                    println!("{}", acc);
                    acc.edit();
                }
            }
        } else if arg == &Data::DATA_TYPES[1] {
            let index = Category::find(&self.categories);
            if index >= 0 {
                if let Some(cat) = self.categories.get_mut(index as usize) {
                    println!("{}", cat);
                    cat.edit();
                }
            }
        } else if arg == &Data::DATA_TYPES[2] {
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
        if arg == &Data::DATA_TYPES[0] {
            let index = Account::find(&self.accounts);
            if index >= 0 {
                self.accounts.remove(index as usize);
            }
        } else if arg == &Data::DATA_TYPES[1] {
            let index = Category::find(&self.categories);
            if index >= 0 {
                self.categories.remove(index as usize);
            }
        } else if arg == &Data::DATA_TYPES[2] {
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
        self.display(DataType::Account);
        self.display(DataType::Category);
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
