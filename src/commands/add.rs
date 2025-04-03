use crate::commands::{Command, CommandResult};

pub(crate) struct Add {
    name: String,
    description: String,
    args: [String; 3],
}

impl Default for Add {
    fn default() -> Self {
        Self {
            name: "(-a, --add)".to_string(),
            description: "Add a new Link to the provided Parent Entity".to_string(),
            args: [
                "<Parent Entity | here>".to_string(),
                "<Link Name>".to_string(),
                "<Value>".to_string(),
            ],
        }
    }
}

impl Command for Add {
    fn error_message(&self) -> String {
        "expected 3 arguments, see the Usage section with tap --add --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("Tap --add command will add a new link to the Parent Entity\n\n");
        s.push_str("Command Structure: tap --add <Parent Entity | here> <Link Name> <Value>\n");
        s.push_str("Example Usage: \n\n");
        s.push_str("  - Add a link to search-engines Parent Entity: tap --add search-engines google https://google.com\n");
        s.push_str("  - Add a link to Parent Entity sharing name of current directory: tap --add here google https://google.com\n");
        s
    }

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        match args.len() {
            1 => {
                if args[0] == "--help" {
                    Ok(CommandResult::Value(self.help_message()))
                } else {
                    Err(self.error_message())
                }
            }
            3 => match (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                ("here", link_name, value) => Ok(CommandResult::Value(format!(
                    "TODO: Implement add functionality for here with Link Name {link_name} and Value {value}"
                ))),
                (parent_entity, link_name, value) => Ok(CommandResult::Value(format!(
                    "TODO: Implement add functionality for Parent Entity {parent_entity} with Link Name {link_name} and Value {value}"
                ))),
            },
            _ => Err(self.error_message()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::export::Export;

    #[test]
    fn test_add_run_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let cmd = Add::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_add_run_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let cmd = Add::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_add_run_expected_three_args_here() {
        let args: Vec<String> = vec![
            "here".to_string(),
            "google".to_string(),
            "https://google.com".to_string(),
        ];
        let cmd = Add::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement add functionality for here with Link Name google and Value https://google.com".to_string()
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_add_run_expected_three_args_parent_entity() {
        let args: Vec<String> = vec![
            "search-engines".to_string(),
            "google".to_string(),
            "https://google.com".to_string(),
        ];
        let cmd = Add::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement add functionality for Parent Entity search-engines with Link Name google and Value https://google.com".to_string()
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
