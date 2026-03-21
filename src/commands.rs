use super::utils::{
    cli_usage_table::{Row, UsageTableBuilder},
    datastore::Datastore,
};
use std::{
    fmt::{Display, Formatter},
    io::{Read, Seek, Write},
};

pub(super) mod add;
pub(super) mod delete;
pub(super) mod export;
pub(super) mod help;
pub(super) mod here;
pub(super) mod import;
pub(super) mod init;
pub(super) mod parent_entity;
pub(super) mod show;
pub(super) mod upsert;
pub(super) mod version;

#[derive(Debug, PartialEq)]
pub(super) enum CommandResult {
    Value(String),
}

impl Display for CommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResult::Value(v) => v.fmt(f),
        }
    }
}

pub(super) trait Command<RW: Read + Write + Seek> {
    fn consumes_arg() -> bool {
        true
    }
    fn run<I: Iterator<Item = String>>(
        &self,
        parsed_args: I,
        ds: Datastore<RW>,
    ) -> Result<CommandResult, String>;
}

pub(super) trait CommandInfo {
    fn error_message(&self) -> String;
    fn help_message(&self) -> String;
}

// Utility Messages used across commands
pub(in super::commands) fn display_version() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

pub(in super::commands) fn display_commands() -> String {
    let res = UsageTableBuilder::new("Usage:")
        .add_section(
            "Commands:",
            vec![
                // Opening Links:
                Row::new(parent_entity::ParentEntity::default()),
                Row::new(here::Here::default()),
                // Adding, Updating, and Deleting Links:
                Row::new(add::Add::default()),
                Row::new(delete::Delete::default()),
                Row::new(show::Show::default()),
                Row::new(upsert::Upsert::default()),
                // Utility Commands:
                Row::new(init::Init::default()),
                Row::new(import::Import::default()),
                Row::new(export::Export::default()),
                // Other Commands:
                Row::new(help::Help::default()),
                Row::new(version::Version::default()),
            ],
        )
        .build();
    res.to_string()
}
