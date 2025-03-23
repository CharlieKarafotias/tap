mod cli;
mod db;

use cli::run;
use tokio::runtime::Builder;

fn main() {
    Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(10 * 1024 * 1024) // 10MiB
        .build()
        .unwrap()
        .block_on(async {
            run();
        })
}
