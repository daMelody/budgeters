use crate::cli;
use crate::table::Table;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Account {
    id: usize,
    name: String,
    value: f32,
}

impl Account {
    pub fn new(id: usize, possible_name: Option<&str>, possible_value: Option<&str>) -> Account {
        Account {
            id,
            name: match possible_name {
                Some(name) => String::from(name),
                None => String::new(),
            },
            value: match possible_value {
                Some(value) => match str::parse(value) {
                    Ok(parsed) => parsed,
                    Err(_) => 0.0,
                },
                None => 0.0,
            },
        }
    }

    pub fn add(table: &Table) -> Account {
        let id = match table.accounts.is_empty() {
            true => 0,
            false => table.accounts.len(),
        };
        let name = cli::get("Name");
        Account {
            id,
            name,
            value: 0.0,
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},\t{}\t\t{}", self.id, self.name, self.value)
    }
}
