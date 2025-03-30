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

    fn run(&self, parsed_args: Vec<String>) -> Result<CommandResult, String> {
        if parsed_args.is_empty() {
            todo!("Implement Update Functionality")
            // Ok(CommandResult::Value("Implement Update Functionality".to_string()))
        } else {
            Ok(CommandResult::Value(self.help_message()))
        }
    }

    fn parse_args<'a>(&self, args: Vec<String>) -> Result<Vec<String>, String> {
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
    fn test_update_expected_no_args() {
        let args: Vec<String> = vec![];
        let update_cmd = Update::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = update_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_update_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let update_cmd = Update::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = update_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_update_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let update_cmd = Update::default();
        let expected: Result<Vec<String>, String> = Err(update_cmd.error_message());
        let res = update_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[should_panic] // TODO: remove after implementing update functionality
    fn test_update_run() {
        let update_cmd = Update::default();
        // TODO: Implement Update Functionality
        // let expected: Result<CommandResult, String> = Ok(
        //     CommandResult::Value(todo!("Implement Update Functionality"))
        // );
        let res = update_cmd.run(vec![]);
        // assert_eq!(res, expected);
    }
}
