use clap::{Parser, Subcommand};
use clap_complete::engine::{ArgValueCompleter, CompletionCandidate};

#[derive(Debug, Parser)]
#[command(name = "tap")]
pub(super) struct Cli {
    #[command(subcommand)]
    command: Mode,
}

#[derive(Debug, Subcommand)]
enum Mode {
    // TODO: will need to add something like the following for dynamic completion of the paths
    // #[arg(add = clap_complete::ArgValueCompleter::new(completer))]
    #[command(visible_alias = "o")]
    /// Open stored link(s)
    Open {
        /// Link path, e.g. search-engines/google
        path: String,
    },

    #[command(visible_alias = "a")]
    /// Add a stored link
    Add {
        /// Link path, e.g. search-engines/google
        path: String,
        /// URL to store
        url: String,
    },

    #[command(visible_alias = "rm")]
    /// Remove stored link(s)
    Remove {
        /// Link path, e.g. search-engines/google
        path: String,
    },

    #[command(visible_alias = "ls")]
    /// List stored paths / link(s)
    List {
        /// Link path, e.g. search-engines/google
        path: Option<String>,
    },

    #[command(visible_alias = "e")]
    /// Edit an existing link
    Edit {
        /// Link path, e.g. search-engines/google
        path: String,
    },

    #[command(visible_alias = "mv")]
    /// Rename an existing path or link
    Rename {
        /// Link path, e.g. search-engines/google
        path: String,
        new_name: String,
    },
}

// TODO: need to write a custom completer for the path arg
// https://docs.rs/clap_complete/latest/clap_complete/engine/struct.ArgValueCompleter.html
fn path_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    todo!("Implement custom completer for path arg utilizing db");
}
