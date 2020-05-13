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
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = Simple::from_uuid(self.id);
        let mut id = id.to_string();
        let (id_string, _extra) = id.split_at_mut(6);
        write!(f, "{},\t{}\t\t{}", id_string, self.name, self.value)
    }
}
