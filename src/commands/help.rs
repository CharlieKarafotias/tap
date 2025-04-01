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

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        if !args.is_empty() {
            Err(self.error_message())
        } else {
            Ok(CommandResult::Value(self.help_message()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_unexpected_args() {
        let args = vec!["--help".to_string(), "me".to_string()];
        let cmd = Help::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_help_run() {
        let args = vec![];
        let cmd = Help::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
