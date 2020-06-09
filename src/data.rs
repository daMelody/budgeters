use crate::cli::{self, Content};
use crate::filesystem;
use account::Account;
use category::Category;
use std::collections::HashMap;
use transaction::Transaction;

pub mod account;
pub mod category;
pub mod transaction;

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

    /// returns an array of String corresponding to the three DataTypes
    const DATA_TYPES: [&'static str; 4] = ["acc", "cat", "tra", "trf"];

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
        } else {
            eprintln!("Transfer is not a valid data type for LIST function");
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
        } else {
            let (from, to) = Transaction::new_transfer();
            self.transactions.push(from);
            self.transactions.push(to);
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
                    match acc.edit() {
                        account::AccountField::Name(old, new) => {
                            for tr in self.transactions.iter_mut() {
                                if tr.get_account() == old {
                                    tr.set_account(new.clone());
                                }
                            }
                        }
                        account::AccountField::Value => (),
                        account::AccountField::None => (),
                    }
                }
            }
        } else if arg == &Data::DATA_TYPES[1] {
            let index = Category::find(&self.categories);
            if index >= 0 {
                if let Some(cat) = self.categories.get_mut(index as usize) {
                    println!("{}", cat);
                    match cat.edit() {
                        category::CategoryField::Name(old, new) => {
                            for tr in self.transactions.iter_mut() {
                                if tr.get_category() == old {
                                    tr.set_category(new.clone());
                                }
                            }
                        }
                        category::CategoryField::Expected => (),
                        category::CategoryField::None => (),
                    }
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
        } else {
            eprintln!("Transfer is not a valid data type for EDIT functionality");
        }
    }

    pub fn delete(&mut self, arg: &String) {
        if arg.is_empty() {
            return;
        }
        if arg == &Data::DATA_TYPES[0] {
            let index = Account::find(&self.accounts);
            if index >= 0 {
                let deleted = self.accounts.get(index as usize).unwrap();
                for tr in self.transactions.iter_mut() {
                    if tr.get_account() == deleted.get_name() {
                        tr.set_account(String::from("<empty>"));
                    }
                }
                self.accounts.remove(index as usize);
            }
        } else if arg == &Data::DATA_TYPES[1] {
            let index = Category::find(&self.categories);
            if index >= 0 {
                let deleted = self.categories.get(index as usize).unwrap();
                for tr in self.transactions.iter_mut() {
                    if tr.get_category() == deleted.get_name() {
                        tr.set_category(String::from("<empty>"));
                    }
                }
                self.categories.remove(index as usize);
            }
        } else if arg == &Data::DATA_TYPES[2] {
            let index = Transaction::find(&self.transactions);
            if index >= 0 {
                self.transactions.remove(index as usize);
            }
        } else {
            eprintln!("Transfer is not a valid data type for DELETE functionality");
        }
    }

    pub fn update(&mut self) {
        println!("Updating...");
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
            if cat_name != String::from("Transfer") {
                let cat_value = match category_map.get(cat_name) {
                    Some(last) => last + tra_amount,
                    None => tra_amount,
                };
                category_map.insert(cat_name, cat_value);
            }
        }
        // iterate through Accounts and update Value fields
        for acc in self.accounts.iter_mut() {
            match account_map.get(acc.get_name()) {
                Some(num) => acc.set_value(cli::money_round(*num)),
                None => acc.set_value(0.0),
            }
        }
        // iterate through Category and update Actual fields
        for cat in self.categories.iter_mut() {
            match category_map.get(cat.get_name()) {
                Some(num) => cat.set_actual(cli::money_round(*num)),
                None => cat.set_actual(0.0),
            }
        }
        println!("Done");
    }

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

    pub fn roll(&mut self) {
        self.update(); // make sure that Data is updated
        filesystem::save(self);
        self.transactions.clear();
        let dir = filesystem::get_dir_path();
        for acc in self.accounts.iter() {
            let mut description = String::from("rolling into ");
            description.push_str(&dir.month);
            self.transactions.push(Transaction::build(
                cli::first_of_month(&dir.month, &dir.year),
                *acc.get_value(),
                acc.get_name().to_string(),
                String::from("Rollover"),
                description,
            ));
        }
    }

    /// display the list of DataType
    pub fn display(&self, data: DataType) {
        match data {
            DataType::Account => {
                let mut contents = Vec::new();
                let mut total_value = 0.0;
                for acc in self.accounts.iter() {
                    total_value += acc.get_value();
                    contents.push({
                        let mut tmp = Vec::new();
                        tmp.push(Content::St(acc.get_simple_id()));
                        tmp.push(Content::St(acc.get_name().to_string()));
                        tmp.push(Content::Num(acc.get_value().to_string()));
                        tmp
                    });
                }
                println!("===== ACCOUNTS =====");
                println!("You are worth ${}", cli::money_round(total_value));
                cli::make_table(vec!["id", "name", "value"], &contents);
            }
            DataType::Category => {
                let mut contents = Vec::new();
                let mut total_expected = 0.0;
                let mut total_actual = 0.0;
                for cat in self.categories.iter() {
                    if cat.get_name() != "Rollover" {
                        total_expected += cat.get_expected();
                        total_actual += cat.get_actual();
                    }
                    contents.push({
                        let mut tmp = Vec::new();
                        tmp.push(Content::St(cat.get_simple_id()));
                        tmp.push(Content::St(cat.get_name().to_string()));
                        tmp.push(Content::Num(cat.get_expected().to_string()));
                        tmp.push(Content::Num(cat.get_actual().to_string()));
                        tmp
                    });
                }
                println!("===== CATEGORIES =====");
                println!(
                    "You had planned to save ${}, you are actually saving ${}",
                    cli::money_round(total_expected),
                    cli::money_round(total_actual)
                );
                cli::make_table(vec!["id", "name", "expected", "actual"], &contents);
            }
            DataType::Transaction => {
                let mut contents = Vec::new();
                for tra in self.transactions.iter() {
                    contents.push({
                        let mut tmp = Vec::new();
                        tmp.push(Content::St(tra.get_simple_id()));
                        tmp.push(Content::St(tra.get_date()));
                        tmp.push(Content::Num(tra.get_amount().to_string()));
                        tmp.push(Content::St(tra.get_account().to_string()));
                        tmp.push(Content::St(tra.get_category().to_string()));
                        tmp.push(Content::St(tra.get_description().to_string()));
                        tmp
                    });
                }
                println!("===== TRANSACTIONS =====");
                cli::make_table(
                    vec!["id", "date", "amount", "account", "category", "description"],
                    &contents,
                );
            }
        }
    }
}
