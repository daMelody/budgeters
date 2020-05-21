use crate::cli;
use std::fmt;
use uuid::{adapter::Simple, Uuid};

#[derive(Clone, Debug)]
pub struct Transaction {
    id: Uuid,
    date: String,
    amount: f32,
    account: String,
    category: String,
    description: String,
}

impl Transaction {
    pub fn get_account(&self) -> &str {
        &self.account
    }

    pub fn get_category(&self) -> &str {
        &self.category
    }

    pub fn get_amount(&self) -> f32 {
        self.amount
    }

    pub fn build(
        possible_date: Option<&str>,
        possible_amount: Option<&str>,
        possible_account: Option<&str>,
        possible_category: Option<&str>,
        possible_description: Option<&str>,
    ) -> Transaction {
        Transaction {
            id: Uuid::new_v4(),
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

    pub fn new() -> Transaction {
        let date = cli::get_input("Date"); // TODO: use some Date object
        let mut possible_amount = cli::get_input("Amount: ");
        let amount = cli::try_into_money(&mut possible_amount);
        let account = cli::get_input("Account"); //TODO: compare with Account names
        let category = cli::get_input("Category"); //TODO: compare with Category names
        let description = cli::get_input("Description");
        Transaction {
            id: Uuid::new_v4(),
            date,
            amount,
            account,
            category,
            description,
        }
    }

    pub fn search(transactions: &Vec<Transaction>, arg: &String) {
        println!("===== Search Results =====");
        for tra in transactions.iter() {
            if tra.date.contains(arg)
                || tra.account.contains(arg)
                || tra.category.contains(arg)
                || tra.description.contains(arg)
            {
                println!("{}", tra);
            }
        }
    }

    pub fn find(transactions: &Vec<Transaction>) -> i32 {
        let arg = cli::get_input("ID");
        let mut index = 0;
        for tra in transactions {
            if tra.simplify_id().contains(&arg) {
                return index;
            }
            index += 1;
        }
        -1
    }

    pub fn edit(&mut self) {
        let field = cli::get_input("Field to edit");
        if field == "date" {
            self.date = cli::get_input("Date");
        } else if field == "amount" {
            self.amount = cli::try_into_money(&mut cli::get_input("Amount"));
        } else if field == "account" {
            self.account = cli::get_input("Account");
        } else if field == "category" {
            self.category = cli::get_input("Category");
        } else if field == "description" {
            self.description = cli::get_input("Description");
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
        st.push_str(&self.date);
        st.push(',');
        st.push_str(&self.amount.to_string());
        st.push(',');
        st.push_str(&self.account);
        st.push(',');
        st.push_str(&self.category);
        st.push(',');
        st.push_str(&self.description);
        st.push('\n');
        st
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},\t{}\t{}\t\t{}\t\t{}\t\t{}",
            self.simplify_id(),
            self.date,
            self.amount,
            self.account,
            self.category,
            self.description
        )
    }
}
