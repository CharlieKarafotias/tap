use super::utils::cli_usage_table::{Row, UsageTableBuilder};
use std::fmt::{Display, Formatter};

pub(crate) mod add;
pub(crate) mod delete;
pub(crate) mod export;
pub(crate) mod help;
pub(crate) mod here;
pub(crate) mod import;
pub(crate) mod init;
pub(crate) mod parent_entity;
pub(crate) mod show;
pub(crate) mod tui;
pub(crate) mod update;
pub(crate) mod upsert;
pub(crate) mod version;

#[derive(Debug, PartialEq)]
pub enum CommandResult {
    Value(String),
}

impl Display for CommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResult::Value(v) => v.fmt(f),
        }
    }
}

pub trait Command {
    fn error_message(&self) -> String;
    fn help_message(&self) -> String;
    fn run(&self, parsed_args: Vec<String>) -> Result<CommandResult, String>;
}

// Utility Messages used across commands
pub(in crate::commands) fn display_version() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

pub(in crate::commands) fn display_commands() -> String {
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
                Row::new(tui::Tui::default()),
                Row::new(update::Update::default()),
                // Other Commands:
                Row::new(help::Help::default()),
                Row::new(version::Version::default()),
            ],
        )
        .build();
    res.to_string()
}
