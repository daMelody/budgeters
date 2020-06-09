mod cli;
mod data;
mod filesystem;
use cli::Command;
use data::Data;

pub fn run() {
    let mut data = Data::new();
    loop {
        match cli::prompt() {
            Command::Help => cli::print_help(),
            Command::Open => {
                data = if data.accounts.is_empty()
                    && data.categories.is_empty()
                    && data.transactions.is_empty()
                {
                    filesystem::setup()
                } else {
                    filesystem::save(&mut data);
                    filesystem::setup()
                }
            }
            Command::Save => filesystem::save(&mut data),
            Command::RollOver => data.roll(), // TODO: data.roll(args)
            Command::Cancel => break,
            Command::Quit => {
                filesystem::save(&mut data);
                break;
            }
            Command::Update => data.update(),
            Command::Empty => continue,
            Command::Add(ref args) => data.add(args),
            Command::Edit(ref args) => data.edit(args),
            Command::Delete(ref args) => data.delete(args),
            Command::Search(ref args) => data.search(args),
            Command::List(ref args) => data.list(args),
        }
    }
}
