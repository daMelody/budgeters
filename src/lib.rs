use home; // for the home_dir function
use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};

mod table;
use table::Table;
mod cli;
use cli::Command;

fn setup() -> Table {
    println!("Setting up...");
    let mut new_table = Table::new();
    match fs::read_dir(Path::new(&get_dir_path())) {
        Ok(files) => parse_dir(&mut new_table, files),
        Err(e) => {
            eprintln!("Error setting up: {}", e);
            new_table
        }
    }
}

fn shutdown(table: &Table) {
    println!("Shutting down...");
    let root = get_dir_path();
    // write out Accounts
    let mut accounts = root.clone();
    accounts.push_str("/Account.cls");
    fs::write(Path::new(&accounts), table.to_cls(&accounts)).expect("Failed to save accounts.");
    let mut categories = root.clone();
    categories.push_str("/Category.cls");
    fs::write(Path::new(&categories), table.to_cls(&categories))
        .expect("Failed to save categories.");
    let mut transactions = root.clone();
    transactions.push_str("/Transaction.cls");
    fs::write(Path::new(&transactions), table.to_cls(&transactions))
        .expect("Failed to save transactions.");
}

pub fn run() {
    let mut table = setup();
    loop {
        match cli::prompt() {
            Command::Cancel => break,
            Command::Quit => {
                shutdown(&mut table);
                break;
            }
            Command::Empty => continue,
            Command::Add(ref args) => table.add(args),
            Command::Edit(ref args) => table.edit(args),
            Command::Delete(ref args) => table.delete(args),
            Command::Search(ref args) => table.search(args),
            Command::List(ref args) => table.list(args),
            Command::RollOver(ref args) => table.roll(args), // TODO: table.rollover(args)
        }
    }
}

fn get_dir_path() -> String {
    let home = match home::home_dir() {
        Some(path) => path,
        None => panic!("Could not get home directory path"),
    };
    let home_str = match home.to_str() {
        Some(s) => s,
        None => panic!("Could not convert home path to string"),
    };
    let mut root = String::from(home_str);
    root.push_str("/budget_tracker/");
    root.push_str(&cli::get_input("year"));
    root.push('/');
    root.push_str(&cli::get_input("month"));
    println!("{}", root); // ! DEBUG
    root
}

fn parse_dir(new_table: &mut Table, files: ReadDir) -> Table {
    for f in files {
        match f {
            Ok(f) => {
                if let Ok(contents) = fs::read_to_string(f.path()) {
                    parse_file(new_table, contents, f.path());
                } else {
                    eprintln!("Error converting file to String, making new list");
                }
            }
            Err(_) => {
                eprintln!("Error reaching file, making new list");
            }
        };
    }
    new_table.clone()
}

fn parse_file(new_table: &mut Table, contents: String, filename: PathBuf) {
    if filename.ends_with("Account.cls") {
        new_table.build_accounts(contents);
    } else if filename.ends_with("Category.cls") {
        new_table.build_categories(contents);
    } else if filename.ends_with("Transaction.cls") {
        new_table.build_transactions(contents);
    } else {
        eprintln!("Unexpected filename while parsing file");
    }
}
