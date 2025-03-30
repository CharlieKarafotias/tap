use crate::commands::{Command, CommandResult};

pub(crate) struct Tui {
    name: String,
    description: String,
}

impl Default for Tui {
    fn default() -> Self {
        Self {
            name: "(--tui)".to_string(),
            description: "Launches UI for adding, updating, and deleting links".to_string(),
        }
    }
}

impl Command for Tui {
    fn error_message(&self) -> String {
        "too many arguments, see the Usage section with tap --tui --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("Opens a terminal user interface to facilitate adding, updating, and deleting links.\n\n");
        s.push_str("Example Usage: tap --tui");
        s
    }

    fn run(&self, parsed_args: Vec<String>) -> Result<CommandResult, String> {
        if parsed_args.is_empty() {
            todo!("Implement TUI Functionality")
            // Ok(CommandResult::Value("Implement TUI Functionality".to_string()))
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
    fn test_tui_expected_no_args() {
        let args: Vec<String> = vec![];
        let tui_cmd = Tui::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = tui_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_tui_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let tui_cmd = Tui::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = tui_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_tui_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let tui_cmd = Tui::default();
        let expected: Result<Vec<String>, String> = Err(tui_cmd.error_message());
        let res = tui_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[should_panic] // TODO: remove after implementing tui functionality
    fn test_tui_run() {
        let tui_cmd = Tui::default();
        // TODO: Implement tui Functionality
        // let expected: Result<CommandResult, String> = Ok(
        //     CommandResult::Value(todo!("Implement tui Functionality"))
        // );
        let res = tui_cmd.run(vec![]);
        // assert_eq!(res, expected);
    }
}
