use std::fmt;

#[derive(Clone, Debug)]
pub struct Transaction {
    date: String,
    amount: f32,
    account: String,
    category: String,
    description: String,
}

impl Transaction {
    pub fn new(
        possible_date: Option<&str>,
        possible_amount: Option<&str>,
        possible_account: Option<&str>,
        possible_category: Option<&str>,
        possible_description: Option<&str>,
    ) -> Transaction {
        Transaction {
            date: match possible_date {
                Some(datetime) => String::from(datetime),
                None => String::new(),
            },
            amount: match possible_amount {
                Some(amount) => match str::parse(amount) {
                    Ok(parsed) => parsed,
                    Err(_) => 0.0,
                },
                None => 0.0,
            },
            account: match possible_account {
                Some(account) => String::from(account),
                None => String::new(),
            },
            category: match possible_category {
                Some(category) => String::from(category),
                None => String::new(),
            },
            description: match possible_description {
                Some(description) => String::from(description),
                None => String::new(),
            },
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}",
            self.date, self.amount, self.account, self.category, self.description
        )
    }
}
