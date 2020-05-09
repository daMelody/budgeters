use home; // for the home_dir function
use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};
use std::str;

mod table; // bringing table module into scope
use table::Table; // import Table struct for use

pub fn setup(sub_dir: String) -> Table {
    let mut new_table = Table::new();
    let mut dir_name = String::from("/budget_tracker/");
    dir_name.push_str(&sub_dir);
    match fs::read_dir(Path::new(&get_dir_path(dir_name))) {
        Ok(files) => parse_dir(&mut new_table, files),
        Err(e) => {
            eprintln!("Error setting up: {}", e);
            new_table
        }
    }
}

pub fn run(data: &mut Table) {
    data.display("accounts");
    println!();
    data.display("categories");
    println!();
    data.display("transactions");
}

pub fn get_dir_name(mut args: std::env::Args) -> Result<String, &'static str> {
    args.next(); // advance iterator the the first item
    let month = match args.next() {
        Some(m) => m,
        None => return Err("Didn't get a month"),
    };
    let mut year = match args.next() {
        Some(y) => y,
        None => return Err("Didn't get a year"),
    };
    year.push('/');
    year.push_str(&month);
    Ok(year)
}

fn get_dir_path(dir_name: String) -> String {
    let home = match home::home_dir() {
        Some(path) => path,
        None => panic!("Could not get home directory path"),
    };
    let home_str = match home.to_str() {
        Some(s) => s,
        None => panic!("Could not convert home path to string"),
    };
    let mut root = String::from(home_str);
    root.push_str(&dir_name);
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
        eprintln!("Unexpected filename");
    }
}
