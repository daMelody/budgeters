use home; // for the home_dir function
use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};

use crate::cli;
use crate::data::Data;

pub fn setup() -> Data {
    println!("Setting up...");
    let mut new_data = Data::new();
    match fs::read_dir(Path::new(&get_dir_path().full)) {
        Ok(files) => parse_dir(&mut new_data, files),
        Err(e) => {
            eprintln!("Error setting up: {}", e);
            new_data
        }
    }
}

pub fn save(data: &Data) {
    let root = get_dir_path();
    // write out Accounts
    let mut accounts = root.full.to_string();
    accounts.push_str("/Account.cls");
    fs::write(Path::new(&accounts), data.to_cls(&accounts)).expect("Failed to save accounts.");
    let mut categories = root.full.to_string();
    categories.push_str("/Category.cls");
    fs::write(Path::new(&categories), data.to_cls(&categories))
        .expect("Failed to save categories.");
    let mut transactions = root.full.to_string();
    transactions.push_str("/Transaction.cls");
    fs::write(Path::new(&transactions), data.to_cls(&transactions))
        .expect("Failed to save transactions.");
    println!("Saved...");
}

pub struct DirPath {
    pub month: String,
    pub year: String,
    pub full: String,
}

pub fn get_dir_path() -> DirPath {
    let home = match home::home_dir() {
        Some(path) => path,
        None => panic!("Could not get home directory path"),
    };
    let home_str = match home.to_str() {
        Some(s) => s,
        None => panic!("Could not convert home path to string"),
    };
    let mut root = String::from(home_str);
    let year = cli::get_input("Year");
    let month = cli::get_input("Month");
    root.push_str("/budget_tracker/");
    root.push_str(&year);
    root.push('/');
    root.push_str(&month);
    println!("{}", root); // * INFO
    DirPath {
        month,
        year,
        full: root,
    }
}

fn parse_dir(new_data: &mut Data, files: ReadDir) -> Data {
    for f in files {
        match f {
            Ok(f) => {
                if let Ok(contents) = fs::read_to_string(f.path()) {
                    parse_file(new_data, contents, f.path());
                } else {
                    eprintln!("Error converting file to String, making new list");
                }
            }
            Err(_) => {
                eprintln!("Error reaching file, making new list");
            }
        };
    }
    new_data.clone()
}

fn parse_file(new_data: &mut Data, contents: String, filename: PathBuf) {
    if filename.ends_with("Account.cls") {
        new_data.build_accounts(contents);
    } else if filename.ends_with("Category.cls") {
        new_data.build_categories(contents);
    } else if filename.ends_with("Transaction.cls") {
        new_data.build_transactions(contents);
    } else {
        eprintln!("Unexpected filename while parsing file");
    }
}
