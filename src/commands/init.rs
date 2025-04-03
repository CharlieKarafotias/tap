use crate::commands::{Command, CommandResult};

pub(crate) struct Init {
    name: String,
    description: String,
    args: [String; 0],
}

impl Default for Init {
    fn default() -> Self {
        Self {
            name: "(-i, --init)".to_string(),
            description: "Initializes Tap (Shell Auto-Completion, etc.)".to_string(),
            args: [],
        }
    }
}

impl Command for Init {
    fn error_message(&self) -> String {
        "too many arguments, see the Usage section with tap --init --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("Initializes Tap (Shell Auto-Completion, etc.).\n\n");
        s.push_str("Example Usage: tap --init");
        s
    }

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        match args.len() {
            0 => todo!("Implement init Functionality"),
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
    #[should_panic] // TODO: remove after implementing init functionality
    fn test_init_run_expected_args() {
        let cmd = Init::default();
        let args: Vec<String> = vec![];
        let res = cmd.run(args);
    }

    #[test]
    fn test_init_run_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let cmd = Init::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_init_run_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let cmd = Init::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
