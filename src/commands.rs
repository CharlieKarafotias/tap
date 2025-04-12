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
    // Opening Links:
    let parent_entity = parent_entity::ParentEntity::default();
    let here = here::Here::default();
    // Adding, Updating, and Deleting Links:
    let add = add::Add::default();
    let delete = delete::Delete::default();
    let show = show::Show::default();
    let upsert = upsert::Upsert::default();
    // Utility Commands:
    let init = init::Init::default();
    let import = import::Import::default();
    let export = export::Export::default();
    let tui = tui::Tui::default();
    let update = update::Update::default();
    // Other Commands:
    let help = help::Help::default();
    let version = version::Version::default();

    let res = UsageTableBuilder::new("Usage:")
        .add_section(
            "Commands:",
            vec![
                Row::new(parent_entity),
                Row::new(here),
                Row::new(add),
                Row::new(delete),
                Row::new(show),
                Row::new(upsert),
                Row::new(init),
                Row::new(import),
                Row::new(export),
                Row::new(tui),
                Row::new(update),
                Row::new(help),
                Row::new(version),
            ],
        )
        .build();
    res.to_string()
}
