use crate::cli;
use std::fmt;
use uuid::{adapter::Simple, Uuid};

#[derive(Debug, Clone)]
pub struct Category {
    id: Uuid,
    name: String,
    expected: f32,
    actual: f32,
}

impl Category {
    pub fn build(
        possible_name: Option<&str>,
        possible_expected: Option<&str>,
        possible_actual: Option<&str>,
    ) -> Category {
        Category {
            id: Uuid::new_v4(),
            name: match possible_name {
                Some(name) => String::from(name),
                None => String::new(),
            },
            expected: match possible_expected {
                Some(expected) => match str::parse(expected) {
                    Ok(parsed) => parsed,
                    Err(_) => 0.0,
                },
                None => 0.0,
            },
            actual: match possible_actual {
                Some(actual) => match str::parse(actual) {
                    Ok(parsed) => parsed,
                    Err(_) => 0.0,
                },
                None => 0.0,
            },
        }
    }

    pub fn new() -> Category {
        let name = cli::get_input("Name");
        let mut possible_expected = cli::get_input("Expected");
        let expected = cli::try_into_money(&mut possible_expected);
        Category {
            id: Uuid::new_v4(),
            name,
            expected,
            actual: 0.0,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = Simple::from_uuid(self.id);
        let mut id = id.to_string();
        let (id_string, _extra) = id.split_at_mut(6);
        write!(
            f,
            "{},\t{}\t\t{}\t\t{}",
            id_string, self.name, self.expected, self.actual
        )
    }
}
