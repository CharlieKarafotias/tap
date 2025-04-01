use crate::commands::update::Update;
use crate::commands::{Command, CommandResult};
use crate::commands::{
    add::Add, delete::Delete, export::Export, help::Help, here::Here, import::Import, init::Init,
    parent_entity::ParentEntity, show::Show, tui::Tui, upsert::Upsert, version::Version,
};
use std::env;

/// Collects command-line arguments, skipping the first argument (the program name).
///
/// # Returns
///
/// A vector of strings containing the command-line arguments provided after the program name.
pub fn collect_args() -> Vec<String> {
    env::args().skip(1).collect()
}

// TODO: add tests for these entry see CLI book: https://rust-cli.github.io/book/tutorial/testing.html
pub fn run(args: Vec<String>) -> Result<CommandResult, String> {
    match args.len() {
        0 => Help::default().run(args),
        _ => match args[0].as_str() {
            // General:
            "--help" => Help::default().run(Vec::from(&args[1..])),
            "-v" | "--version" => Version::default().run(Vec::from(&args[1..])),
            // // Utilities:
            "--update" => Update::default().run(Vec::from(&args[1..])),
            "--tui" => Tui::default().run(Vec::from(&args[1..])),
            "-i" | "--init" => Init::default().run(Vec::from(&args[1..])),
            "--import" => Import::default().run(Vec::from(&args[1..])),
            "--export" => Export::default().run(Vec::from(&args[1..])),
            // Adding, Updating, and Deleting Links:
            "-a" | "--add" => Add::default().run(Vec::from(&args[1..])),
            "-d" | "--delete" => Delete::default().run(Vec::from(&args[1..])),
            "-s" | "--show" => Show::default().run(Vec::from(&args[1..])),
            "-u" | "--upsert" => Upsert::default().run(Vec::from(&args[1..])),
            // // Opening links:
            "here" => Here::default().run(Vec::from(&args[1..])),
            _parent_entity => ParentEntity::default().run(Vec::from(&args[..])),
        },
    }
}
