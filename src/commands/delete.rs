use crate::commands::{Command, CommandResult};

pub(crate) struct Delete {
    name: String,
    description: String,
    args: [String; 2],
}

impl Default for Delete {
    fn default() -> Self {
        Self {
            name: "(-d, --delete)".to_string(),
            description: "Deletes an existing Link from the provided Parent Entity".to_string(),
            args: [
                "<Parent Entity | here>".to_string(),
                "[Link Name]".to_string(),
            ],
        }
    }
}

impl Command for Delete {
    fn error_message(&self) -> String {
        "expected 1-2 arguments, see the Usage section with tap --delete --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("Tap --delete command will delete either a specific link or all links of a Parent Entity\n\n");
        s.push_str("Command Structure: tap --delete <Parent Entity | here> [Link Name]\n");
        s.push_str("Example Usage: \n\n");
        s.push_str("  - Delete all links: tap --delete search-engines\n");
        s.push_str("  - Delete specific link: tap --delete search-engines google\n");
        s.push_str("  - Delete all links associated to parent entity sharing name of current directory: tap --delete here\n");
        s
    }

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        match args.len() {
            1 => match args[0].as_str() {
                "--help" => Ok(CommandResult::Value(self.help_message())),
                "here" => Ok(CommandResult::Value(
                    "TODO: Implement delete functionality for here".to_string(),
                )),
                parent_entity => Ok(CommandResult::Value(format!(
                    "TODO: Implement delete functionality for Parent Entity: {parent_entity}"
                ))),
            },
            2 => match (args[0].as_str(), args[1].as_str()) {
                ("here", link_name) => Ok(CommandResult::Value(format!(
                    "TODO: Implement delete functionality for here with Link Name {link_name}"
                ))),
                (parent_entity, link_name) => Ok(CommandResult::Value(format!(
                    "TODO: Implement delete functionality for Parent Entity {parent_entity} with Link Name {link_name}"
                ))),
            },
            _ => Err(self.error_message()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_run_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_run_unexpected_args() {
        let args: Vec<String> = vec![
            "random".to_string(),
            "random2".to_string(),
            "random3".to_string(),
        ];
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_run_expected_here_arg() {
        let args: Vec<String> = vec!["here".to_string()];
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement delete functionality for here".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_run_expected_here_and_link_args() {
        let args: Vec<String> = vec!["here".to_string(), "google".to_string()];
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement delete functionality for here with Link Name google".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_run_expected_parent_entity_arg() {
        let args: Vec<String> = vec!["search-engines".to_string()];
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement delete functionality for Parent Entity: search-engines".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_run_expected_parent_entity_and_link_args() {
        let args: Vec<String> = vec!["search-engines".to_string(), "google".to_string()];
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement delete functionality for Parent Entity search-engines with Link Name google".to_string()
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
