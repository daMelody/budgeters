use chrono::{DateTime, NaiveDate, SecondsFormat, Utc};
use prettytable::{Cell, Row, Table};
use std::io::{self, prelude::*};

// getting the commands
pub enum Command {
    Empty,
    Cancel,
    Quit,
    Update,
    List(String),
    Add(String),
    Edit(String),
    Delete(String),
    Search(String),
    RollOver(String),
}

pub fn prompt() -> Command {
    let mut inputs: Vec<String> = Vec::new();
    for word in get_input("$").split_whitespace() {
        inputs.push(String::from(word));
    }
    let command = match inputs.get(0) {
        Some(st) => st,
        None => "",
    };
    let types = match inputs.get(1..) {
        Some(st) => {
            let mut tmp = String::new();
            for s in st {
                tmp.push_str(s);
                tmp.push(' ');
            }
            tmp.trim_end().to_string()
        }
        None => String::new(),
    };
    match command {
        "--cancel" => Command::Cancel,
        "q" => Command::Quit,
        "--update" => Command::Update,
        "l" => Command::List(types),
        "a" => Command::Add(types),
        "e" => Command::Edit(types),
        "d" => Command::Delete(types),
        "/" => Command::Search(types),
        "--roll" => Command::RollOver(types),
        _ => Command::Empty,
    }
}

pub fn get_input(arg: &str) -> String {
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
    String::from(buffer.trim())
}

pub fn try_into_money(possible_num: &mut String) -> f32 {
    match possible_num.parse() {
        Ok(num) => num,
        Err(_) => {
            possible_num.push_str(".0");
            println!("{}", possible_num);
            match possible_num.parse() {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("Error converting Expected value: {}", e);
                    eprintln!("Substituting 0.0, edit if not satisfactory");
                    0.0
                }
            }
        }
    }
}

pub fn try_into_date(possible_date: &String) -> DateTime<Utc> {
    let dt = NaiveDate::parse_from_str(possible_date, "%m/%d/%Y").expect("Couldn't parse date");
    DateTime::<Utc>::from_utc(dt.and_hms(0, 0, 0), Utc)
}

pub fn try_date_to_string(date_time: DateTime<Utc>) -> String {
    date_time.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn make_table(headers: Vec<&str>, contents: &Vec<Vec<String>>) {
    let mut table = Table::new();
    // add headers to the table
    table.add_row(Row::new({
        let mut cells = Vec::new();
        for h in headers {
            cells.push(
                // style: center & bold text, Yellow color
                Cell::new(h).style_spec("cbFy"),
            );
        }
        cells
    }));
    for row in contents {
        table.add_row(Row::new({
            let mut cells = Vec::new();
            for c in row {
                cells.push(Cell::new(c))
            }
            cells
        }));
    }
    table.printstd();
}
