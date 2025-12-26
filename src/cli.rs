use std::env::Args;

use crate::commands::{Command, CommandResult};
use crate::commands::{
    add::Add, delete::Delete, export::Export, help::Help, here::Here, import::Import, init::Init,
    parent_entity::ParentEntity, show::Show, upsert::Upsert, version::Version,
};

pub fn run(mut args: Args) -> Result<CommandResult, String> {
    match args.next() {
        None => Help::default().run(args),
        Some(command) => match command.as_str() {
            // General:
            "--help" => Help::default().run(args),
            "-v" | "--version" => Version::default().run(args),
            // Utilities:
            "-i" | "--init" => Init::default().run(args),
            "--import" => Import::default().run(args),
            "--export" => Export::default().run(args),
            // Adding, Updating, and Deleting Links:
            "-a" | "--add" => Add::default().run(args),
            "-d" | "--delete" => Delete::default().run(args),
            "-s" | "--show" => Show::default().run(args),
            "-u" | "--upsert" => Upsert::default().run(args),
            // Opening links:
            "here" => Here::default().run(args),
            _parent_entity => ParentEntity::default().run(args),
        },
    }
}
