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

    pub fn add() -> Category {}
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
