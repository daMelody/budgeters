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
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_actual(&mut self, new_actual: f32) {
        self.actual = new_actual;
    }

    pub fn from_cls(
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

    pub fn find(categories: &Vec<Category>) -> i32 {
        let arg = cli::get_input("ID");
        let mut index = 0;
        for cat in categories {
            if cat.simplify_id().contains(&arg) {
                return index;
            }
            index += 1;
        }
        -1
    }

    pub fn edit(&mut self) {
        let field = cli::get_input("Field to edit");
        if field == "name" {
            self.name = cli::get_input("Name");
        } else if field == "expected" {
            self.expected = cli::try_into_money(&mut cli::get_input("Expected"));
        } else if field == "actual" {
            self.actual = cli::try_into_money(&mut cli::get_input("Actual"));
        } else {
            return;
        }
    }

    fn simplify_id(&self) -> String {
        let id = Simple::from_uuid(self.id);
        let mut id = id.to_string();
        let (id_string, _extra) = id.split_at_mut(6);
        id_string.to_string()
    }

    pub fn to_cls(&self) -> String {
        let mut st = String::new();
        st.push_str(&self.name);
        st.push(',');
        st.push_str(&self.expected.to_string());
        st.push(',');
        st.push_str(&self.actual.to_string());
        st.push('\n');
        st
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},\t{}\t\t{}\t\t{}",
            self.simplify_id(),
            self.name,
            self.expected,
            self.actual
        )
    }
}
