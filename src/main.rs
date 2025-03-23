mod db;

use tokio::runtime::Builder;

fn main() {
    Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(10 * 1024 * 1024) // 10MiB
        .build()
        .unwrap()
        .block_on(async {
            // TODO: Make run function for parse args with clap
            // TODO: Make run function for consuming args
            // TODO: Make run function for returning results
            todo!("Implement run functions")
        })
}
