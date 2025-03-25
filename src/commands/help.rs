use crate::commands::{Command, CommandResult, display_commands, display_version};

pub(crate) struct Help {
    name: String,
    description: String,
}

impl Default for Help {
    fn default() -> Self {
        Self {
            name: "--help".to_string(),
            description: "Display this help message".to_string(),
        }
    }
}

impl Command for Help {
    fn error_message(&self) -> String {
        "too many arguments, see the Usage section with tap --help".to_string()
    }

    fn help_message(&self) -> String {
        format!(
            "{}\n{}\n\n{}",
            display_version(),
            env!("CARGO_PKG_DESCRIPTION"),
            display_commands(),
        )
    }

    fn run(&self, _parsed_args: &[String]) -> Result<CommandResult, String> {
        Ok(CommandResult::Value(self.help_message()))
    }

    fn parse_args<'a>(&self, args: &'a [String]) -> Result<&'a [String], String> {
        if !args.is_empty() {
            Err(self.error_message())
        } else {
            Ok(args)
        }
    }
}
