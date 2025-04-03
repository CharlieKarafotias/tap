use crate::commands::{Command, CommandResult, display_version};

pub(crate) struct Version {
    name: String,
    description: String,
    args: [String; 0],
}

impl Default for Version {
    fn default() -> Self {
        Self {
            name: "(-v, --version)".to_string(),
            description: "Display the version".to_string(),
            args: [],
        }
    }
}

impl Command for Version {
    fn error_message(&self) -> String {
        "too many arguments, see the Usage section with tap --version --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("The version command shows the current version.\n\n");
        s.push_str("Example Usage: tap --version");
        s
    }

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        match args.len() {
            0 => Ok(CommandResult::Value(display_version())),
            1 => {
                if args[0] == "--help" {
                    Ok(CommandResult::Value(self.help_message()))
                } else {
                    Err(self.error_message())
                }
            }
            _ => Err(self.error_message()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_run_expected_args() {
        let args: Vec<String> = vec![];
        let cmd = Version::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(display_version()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_version_run_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let cmd = Version::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_version_run_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let cmd = Version::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
