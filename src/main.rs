mod cli;
mod commands;
mod utils;

use cli::run;
use std::process::exit;

fn main() {
    let mut args = std::env::args();
    // NOTE: consume the executable path
    args.next();
    match run(args) {
        Ok(res) => {
            println!("{}", res);
            exit(0);
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            exit(1);
        }
    }
}
