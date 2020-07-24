use clap::Clap;

#[derive(Clap)]
pub struct Args {
    /// a REPL command
    #[clap(long, short, arg_enum)]
    pub command: Command,
}

#[derive(Copy, Clone, Debug)]
pub enum Command {
    Load,
    Save,
    Quit,
    Roll,
    Add,
    Edit,
    Delete,
    Search,
    Sort,
}
