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
            accounts.push(Account::build(cells.next(), cells.next()));
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
            categories.push(Category::build(cells.next(), cells.next(), cells.next()));
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
            transactions.push(Transaction::build(
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

    pub fn list(table: &Table, arg: &String) {
        // expect args to have a type argument
        if arg.is_empty() {
            return;
        }
        if arg == &Table::TABLE_TYPES[0] {
            Table::display(table, TableType::Account);
        } else if arg == &Table::TABLE_TYPES[1] {
            Table::display(table, TableType::Category);
        } else if arg == &Table::TABLE_TYPES[2] {
            Table::display(table, TableType::Transaction);
        }
    }

    pub fn search(table: &Table, arg: &String) {
        // expect args to have a type argument
        if arg.is_empty() {
            return;
        }
        Transaction::search(&table.transactions, arg);
    }

    /* require mutable Table */

    pub fn add(table: &mut Table, arg: &String) {
        if arg.is_empty() {
            return;
        }
        if arg == &Table::TABLE_TYPES[0] {
            table.accounts.push(Account::new());
        } else if arg == &Table::TABLE_TYPES[1] {
            table.categories.push(Category::new());
        } else if arg == &Table::TABLE_TYPES[2] {
            table.transactions.push(Transaction::new());
        }
    }

    pub fn edit(table: &mut Table, arg: &String) {
        if arg.is_empty() {
            return;
        }
        if arg == &Table::TABLE_TYPES[0] {
            let index = Account::find(&table.accounts);
            if index >= 0 {
                if let Some(acc) = table.accounts.get_mut(index as usize) {
                    println!("{}", acc);
                    //TODO: do the editing
                    Account::edit(acc);
                }
            }
        } else if arg == &Table::TABLE_TYPES[1] {
            let index = Category::find(&table.categories);
            if index >= 0 {
                if let Some(cat) = table.categories.get_mut(index as usize) {
                    println!("{}", cat);
                    //TODO: do the editing
                    Category::edit(cat);
                }
            }
        } else if arg == &Table::TABLE_TYPES[2] {
            let index = Transaction::find(&table.transactions);
            if index >= 0 {
                if let Some(tra) = table.transactions.get_mut(index as usize) {
                    println!("{}", tra);
                    //TODO: do the editing
                    Transaction::edit(tra);
                }
            }
        }
    }

    pub fn delete(table: &mut Table, arg: &String) {
        if arg.is_empty() {
            return;
        }
        if arg == &Table::TABLE_TYPES[0] {
            let index = Account::find(&table.accounts);
            if index >= 0 {
                table.accounts.remove(index as usize);
            }
        } else if arg == &Table::TABLE_TYPES[1] {
            let index = Category::find(&table.categories);
            if index >= 0 {
                table.categories.remove(index as usize);
            }
        } else if arg == &Table::TABLE_TYPES[2] {
            let index = Transaction::find(&table.transactions);
            if index >= 0 {
                table.transactions.remove(index as usize);
            }
        }
    }

    // TODO:
    pub fn roll(table: &mut Table, arg: &String) {}
}
