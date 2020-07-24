use anyhow::Result;
use clap::Clap;

//mod cli;

/// Just an entry point into the REPL environment
fn main() -> Result<()> {
    loop {
        println!("budgeter >>> ");
        let args = cli::Args::parse();
    }
}
