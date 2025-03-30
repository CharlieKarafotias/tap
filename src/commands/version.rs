use crate::commands::{Command, CommandResult, display_version};

pub(crate) struct Version {
    name: String,
    description: String,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            name: "(-v, --version)".to_string(),
            description: "Display the version".to_string(),
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

    fn run(&self, parsed_args: Vec<String>) -> Result<CommandResult, String> {
        if parsed_args.is_empty() {
            Ok(CommandResult::Value(display_version()))
        } else {
            Ok(CommandResult::Value(self.help_message()))
        }
    }

    fn parse_args(&self, args: Vec<String>) -> Result<Vec<String>, String> {
        match args.len() {
            0 => Ok(args),
            1 => {
                if args[0] == "--help" {
                    Ok(args)
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

    // parse_args test
    #[test]
    fn test_version_expected_no_args() {
        let args: Vec<String> = vec![];
        let version_cmd = Version::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = version_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_version_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let version_cmd = Version::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = version_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_version_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let version_cmd = Version::default();
        let expected: Result<Vec<String>, String> = Err(version_cmd.error_message());
        let res = version_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_version_run() {
        let version_cmd = Version::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(display_version()));
        let res = version_cmd.run(vec![]);
        assert_eq!(res, expected);
    }
}
