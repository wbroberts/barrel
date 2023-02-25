use std::process;

use barrel::{args::Args, Barrel, FilterOptions};
use colored::Colorize;

fn main() {
    let args = Args::parse();
    let options = FilterOptions::new();

    match Barrel::create(&args.path, &options) {
        Err(e) => {
            eprint!("{}: {}", "Error".bold().red(), e);
            process::exit(1)
        }
        Ok(is_new) => {
            let message = if is_new {
                format!("{} barrel file", "Created".bold().green())
            } else {
                format!("{} barrel file", "Updated".bold().yellow())
            };

            println!("✔️ {}", message);
        }
    };
}
