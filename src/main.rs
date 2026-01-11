mod cli;
mod commands;
mod utils;

use cli::run_with_stdio;
use std::process::exit;

fn main() {
    let exit_code = run_with_stdio();
    exit(exit_code)
}
