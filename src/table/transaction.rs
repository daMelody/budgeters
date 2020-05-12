use crate::cli;
use crate::table::Table;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Transaction {
    id: usize,
    date: String,
    amount: f32,
    account: String,
    category: String,
    description: String,
}

impl Transaction {
    pub fn new(
        id: usize,
        possible_date: Option<&str>,
        possible_amount: Option<&str>,
        possible_account: Option<&str>,
        possible_category: Option<&str>,
        possible_description: Option<&str>,
    ) -> Transaction {
        Transaction {
            id,
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

    pub fn add(table: &Table) -> Transaction {
        let id = match table.transactions.is_empty() {
            true => 0,
            false => table.transactions.len(),
        };
        let date = cli::get_input("Date"); // TODO: use some Date object
        let mut possible_amount = cli::get_input("Amount: ");
        let amount = cli::try_into_money(&mut possible_amount);
        let account = cli::get_input("Account"); //TODO: compare with Account names
        let category = cli::get_input("Category"); //TODO: compare with Category names
        let description = cli::get_input("Description");
        Transaction {
            id,
            date,
            amount,
            account,
            category,
            description,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},\t{}\t{}\t\t{}\t\t{}\t\t{}",
            self.id, self.date, self.amount, self.account, self.category, self.description
        )
    }
}
