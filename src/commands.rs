use std::fmt::{Display, Formatter};

pub(crate) mod add;
pub(crate) mod delete;
pub(crate) mod export;
pub(crate) mod help;
pub(crate) mod import;
pub(crate) mod init;
pub(crate) mod tui;
pub(crate) mod update;
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

// TODO: use the name and descriptions of each command to generate this instead of maintaining manually
pub(in crate::commands) fn display_commands() -> String {
    let mut s = "".to_string();
    s.push_str("Usage:\n");
    s.push_str("  Opening Links:\n");
    s.push_str("   - tap <Parent Entity> [Link Name]                                   Open all Parent Entity's Link. When Link Name is supplied, opens the specific Link\n");
    s.push_str("   - tap here            [Link Name]                                   Uses the current directory as the Parent Entity. Opens all Parent Entity's links. When Link Name is supplied, opens the specific Link\n");
    s.push_str("  Adding, Updating, and Deleting Links:\n");
    s.push_str("   - tap (-a, --add)    <Parent Entity | here> <Link Name> <Value>     Add a new Link to the Parent Entity. It will create the Parent Entity if it doesn't exist\n");
    s.push_str("   - tap (-d, --delete) <Parent Entity | here> [Link Name]             Deletes an existing Link from the Parent Entity. If no Link Name is provided, deletes all Links from the Parent Entity\n");
    s.push_str("   - tap (-s, --show)   <Parent Entity | here> [Link Name]             Shows the value of an existing Link from the Parent Entity. If no Link Name is provided, shows all Link Values from the Parent Entity\n");
    s.push_str("   - tap (-u, --upsert) <Parent Entity | here> <Link Name> <Value>     Upsert an existing Link in the Parent Entity. It will create the Link and Parent Entity if it doesn't exist\n");
    s.push_str("  Utilities:\n");
    s.push_str("   - tap (-i, --init)                                                  Initializes Tap (Shell Auto-Completion, etc.)\n");
    s.push_str("   - tap --import     <Browser | Tap>                                  Imports a bookmark file into Tap. Supports both Tap Files and the following browsers' bookmark manager files: Chrome, Edge, Firefox, Opera, Safari\n");
    s.push_str("   - tap --export     <Browser | Tap>                                  Exports Tap to a bookmark file. Supported Browsers: Chrome, Edge, Firefox, Opera, Safari\n");
    s.push_str("   - tap --tui                                                         Opens a terminal user interface to facilitate adding, updating, and deleting links\n");
    s.push_str(
        "   - tap --update                                                      Updates Tap\n",
    );
    s.push_str(" - tap --help                                                          Display this help message\n");
    s.push_str(" - tap (-v, --version)                                                 Display the version\n");
    s
}
