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
    }

    /// returns an array of String corresponding to the three TableTypes
    const table_types: [String; 3] = [
        String::from("acc"),
        String::from("cat"),
        String::from("tra"),
    ];

    pub fn list(table: &Table, arg: &String) {
        // expect args to have a type argument
        if arg.is_empty() {
            return;
        }
        if arg == &Table::table_types[0] {
            Table::display(table, TableType::Account);
        } else if arg == &Table::table_types[1] {
            Table::display(table, TableType::Category);
        } else {
            Table::display(table, TableType::Transaction);
        }
    }

    pub fn search(table: &Table, arg: &String) {
        // expect args to have a type argument
        if arg.is_empty() {
            return;
        }
    }

    /* require mutable Table */

    pub fn add(table: &mut Table, arg: &String) {
        if arg.is_empty() {
            return;
        }
        if arg == &Table::table_types[0] {
            table.accounts.push(Account::add(&table.accounts));
        } else if arg == &Table::table_types[1] {
            table.categories.push(Category::add(&table.categories));
        } else {
            table.transactions.push(Transaction::add());
        }
    }

    pub fn edit(table: &mut Table, arg: &String) {}
    pub fn delete(table: &mut Table, arg: &String) {}

    // * this is a future thing
    pub fn roll(table: &mut Table, arg: &String) {}
}
