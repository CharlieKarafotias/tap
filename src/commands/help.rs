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

    fn run(&self, _parsed_args: Vec<String>) -> Result<CommandResult, String> {
        Ok(CommandResult::Value(self.help_message()))
    }

    fn parse_args(&self, args: Vec<String>) -> Result<Vec<String>, String> {
        if !args.is_empty() {
            Err(self.error_message())
        } else {
            Ok(args)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // parse_args test
    #[test]
    fn test_help_expected_args() {
        let args = vec![];
        let help_cmd = Help::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = help_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_help_unexpected_args() {
        let args = vec!["--help".to_string()];
        let help_cmd = Help::default();
        let expected: Result<Vec<String>, String> = Err(help_cmd.error_message());
        let res = help_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_help_run() {
        let help_cmd = Help::default();
        let expected: Result<CommandResult, String> =
            Ok(CommandResult::Value(help_cmd.help_message()));
        let res = help_cmd.run(vec![]);
        assert_eq!(res, expected);
    }
}
