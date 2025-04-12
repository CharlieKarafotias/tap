mod cli;
mod commands;
mod utils;

use cli::{collect_args, run};

fn main() {
    let args = collect_args();
    match run(args) {
        Ok(res) => {
            println!("{}", res);
            std::process::exit(0);
        }
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}
