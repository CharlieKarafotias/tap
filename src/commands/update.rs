use crate::commands::{Command, CommandResult};

pub(crate) struct Update {
    name: String,
    description: String,
}

impl Default for Update {
    fn default() -> Self {
        Self {
            name: "(--update)".to_string(),
            description: "Update Tap to the latest version".to_string(),
        }
    }
}

impl Command for Update {
    fn error_message(&self) -> String {
        "too many arguments, see the Usage section with tap --update --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("The update command updates Tap to the latest version.\n\n");
        s.push_str("Example Usage: tap --update");
        s
    }

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        match args.len() {
            0 => todo!("Implement Update Functionality"),
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
    #[should_panic] // TODO: remove after implementing update functionality
    fn test_update_run_expected_args() {
        let args: Vec<String> = vec![];
        let cmd = Update::default();
        let res = cmd.run(args);
    }

    #[test]
    fn test_update_run_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let cmd = Update::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_update_run_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let cmd = Update::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
