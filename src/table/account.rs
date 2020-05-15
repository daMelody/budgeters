use crate::cli;
use std::fmt;
use uuid::{adapter::Simple, Uuid};

#[derive(Clone, Debug)]
pub struct Account {
    id: Uuid,
    name: String,
    value: f32,
}

impl Account {
    pub fn build(possible_name: Option<&str>, possible_value: Option<&str>) -> Account {
        Account {
            id: Uuid::new_v4(),
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

    pub fn new() -> Account {
        let name = cli::get_input("Name");
        Account {
            id: Uuid::new_v4(),
            name,
            value: 0.0,
        }
    }

    pub fn find(accounts: &Vec<Account>) -> i32 {
        let arg = cli::get_input("ID");
        let mut index = 0;
        for acc in accounts {
            if acc.simplify_id().contains(&arg) {
                return index;
            }
            index += 1;
        }
        -1
    }

    pub fn edit(&mut self) {
        let field = cli::get_input("Field to edit");
        let name_field = String::from("name");
        let value_field = String::from("value");
        match field {
            name_field => self.name = cli::get_input("Name"),
            value_field => self.value = cli::try_into_money(&mut cli::get_input("Value")),
            _ => (),
        }
    }

    fn simplify_id(&self) -> String {
        let id = Simple::from_uuid(self.id);
        let mut id = id.to_string();
        let (id_string, _extra) = id.split_at_mut(6);
        id_string.to_string()
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},\t{}\t\t{}",
            self.simplify_id(),
            self.name,
            self.value
        )
    }
}
