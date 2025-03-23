mod cli;
mod db;

use cli::{parse_args, run};
use tokio::runtime::Builder;

fn main() {
    let args = parse_args();
    Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(10 * 1024 * 1024) // 10MiB
        .build()
        .unwrap()
        .block_on(async {
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
        })
}
