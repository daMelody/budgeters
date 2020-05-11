use crate::cli;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Category {
    id: usize,
    name: String,
    expected: f32,
    actual: f32,
}

impl Category {
    pub fn new(
        id: usize,
        possible_name: Option<&str>,
        possible_expected: Option<&str>,
        possible_actual: Option<&str>,
    ) -> Category {
        Category {
            id,
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

    pub fn add(categories: &Vec<Category>) -> Category {
        let id = match categories.is_empty() {
            true => 0,
            false => categories.len(),
        };
        let name = cli::get("Name");
        let expected: f32 = match cli::get("Expected").parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Error converting Expected value: {}", e);
                eprintln!("Substituting 0.0, delete and try again if not satisfactory");
                0.0
            }
        };
        Category {
            id,
            name: String::from(name),
            expected,
            actual: 0.0,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},\t{}\t\t{}\t\t{}",
            self.id, self.name, self.expected, self.actual
        )
    }
}
