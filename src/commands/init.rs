use crate::commands::{Command, CommandResult};

pub(crate) struct Init {
    name: String,
    description: String,
}

impl Default for Init {
    fn default() -> Self {
        Self {
            name: "(-i, --init)".to_string(),
            description: "Initializes Tap (Shell Auto-Completion, etc.)".to_string(),
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

    fn run(&self, parsed_args: Vec<String>) -> Result<CommandResult, String> {
        if parsed_args.is_empty() {
            todo!("Implement init Functionality")
            // Ok(CommandResult::Value("Implement init Functionality".to_string()))
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
    fn test_init_expected_no_args() {
        let args: Vec<String> = vec![];
        let init_cmd = Init::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = init_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_init_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let init_cmd = Init::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = init_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_init_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let init_cmd = Init::default();
        let expected: Result<Vec<String>, String> = Err(init_cmd.error_message());
        let res = init_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[should_panic] // TODO: remove after implementing init functionality
    fn test_init_run() {
        let init_cmd = Init::default();
        // TODO: Implement init Functionality
        // let expected: Result<CommandResult, String> = Ok(
        //     CommandResult::Value(todo!("Implement init Functionality"))
        // );
        let res = init_cmd.run(vec![]);
        // assert_eq!(res, expected);
    }
}
