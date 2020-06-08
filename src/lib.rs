use home; // for the home_dir function
use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};

mod cli;
mod data;
use cli::Command;
use data::Data;

fn setup() -> Data {
    println!("Setting up...");
    let mut new_data = Data::new();
    match fs::read_dir(Path::new(&get_dir_path())) {
        Ok(files) => parse_dir(&mut new_data, files),
        Err(e) => {
            eprintln!("Error setting up: {}", e);
            new_data
        }
    }
}

fn save(data: &Data) {
    println!("Saving...");
    let root = get_dir_path();
    // write out Accounts
    let mut accounts = root.clone();
    accounts.push_str("/Account.cls");
    fs::write(Path::new(&accounts), data.to_cls(&accounts)).expect("Failed to save accounts.");
    let mut categories = root.clone();
    categories.push_str("/Category.cls");
    fs::write(Path::new(&categories), data.to_cls(&categories))
        .expect("Failed to save categories.");
    let mut transactions = root.clone();
    transactions.push_str("/Transaction.cls");
    fs::write(Path::new(&transactions), data.to_cls(&transactions))
        .expect("Failed to save transactions.");
}

pub fn run() {
    let mut data = Data::new();
    loop {
        match cli::prompt() {
            Command::Help => cli::print_help(),
            Command::Open => {
                data = if data.accounts.is_empty()
                    && data.categories.is_empty()
                    && data.transactions.is_empty()
                {
                    setup()
                } else {
                    save(&mut data);
                    setup()
                }
            }
            Command::Save => save(&mut data),
            Command::RollOver => roll(&mut data), // TODO: data.roll(args)
            Command::Cancel => break,
            Command::Quit => {
                save(&mut data);
                break;
            }
            Command::Update => data.update(),
            Command::Empty => continue,
            Command::Add(ref args) => data.add(args),
            Command::Edit(ref args) => data.edit(args),
            Command::Delete(ref args) => data.delete(args),
            Command::Search(ref args) => data.search(args),
            Command::List(ref args) => data.list(args),
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
    root.push_str(&cli::get_input("Year"));
    root.push('/');
    root.push_str(&cli::get_input("Month"));
    println!("{}", root); // * INFO
    root
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

fn roll(_data: &mut Data) {
    /*data.update(); // make sure that Data is updated
    save(data);
    data.transactions.clear();
    let dir = get_dir_path();
    for acc in data.accounts.iter() {
        data.transactions.push(Transaction {
            date: cli::first_of_month(month, year),
        })
    } */
}
