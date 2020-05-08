use std::env;

fn main() {
    let config = budgeters::get_dir_name(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing month and year: {}", err);
        std::process::exit(1);
    });
    let mut data = budgeters::setup(config);
    budgeters::run(&mut data);
}
