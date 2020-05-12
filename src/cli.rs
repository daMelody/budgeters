use std::io::{self, prelude::*};

// getting the commands
pub enum Command {
    Empty,
    Cancel,
    Quit,
    List(String),
    Add(String),
    Edit(String),
    Delete(String),
    Search(String),
    RollOver(String),
}

pub fn prompt() -> Command {
    let mut inputs: Vec<String> = Vec::new();
    for word in get("$").split_whitespace() {
        inputs.push(String::from(word));
    }
    let command = match inputs.get(0) {
        Some(st) => st,
        None => "",
    };
    let types = match inputs.get(1) {
        Some(st) => String::from(st),
        None => String::new(),
    };
    match command {
        "--cancel" => Command::Cancel,
        "q" => Command::Quit,
        "l" => Command::List(types),
        "a" => Command::Add(types),
        "e" => Command::Edit(types),
        "d" => Command::Delete(types),
        "s" => Command::Search(types),
        "r" => Command::RollOver(types),
        _ => Command::Empty,
    }
}

pub fn get(arg: &str) -> String {
    print!("{}: ", arg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Error getting input: {}", e);
            return String::new();
        }
    };
    String::from(buffer)
}
