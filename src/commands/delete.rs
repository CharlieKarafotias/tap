use crate::{
    commands::{Command, CommandResult},
    utils::cli_usage_table::DisplayCommandAsRow,
    utils::command::get_current_directory_name,
    utils::tap_data_store::DataStore,
};

pub(crate) struct Delete {
    name: String,
    description: String,
    args: [String; 2],
}

impl Default for Delete {
    fn default() -> Self {
        Self {
            name: "-d, --delete".to_string(),
            description: "Deletes a link".to_string(),
            args: ["<Parent|here>".to_string(), "[Link]".to_string()],
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
                "here" => {
                    let mut ds = DataStore::new(None).map_err(|e| e.to_string())?;
                    let current_dir_name =
                        get_current_directory_name().map_err(|e| e.to_string())?;
                    ds.delete(current_dir_name.to_string(), None)
                        .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value(format!(
                        "Successfully removed all links of parent '{current_dir_name}'"
                    )))
                }
                parent_entity => {
                    let mut ds = DataStore::new(None).map_err(|e| e.to_string())?;
                    ds.delete(parent_entity.to_string(), None)
                        .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value(format!(
                        "Successfully removed all links of parent '{parent_entity}'"
                    )))
                }
            },
            2 => match (args[0].as_str(), args[1].as_str()) {
                ("here", link_name) => {
                    let mut ds = DataStore::new(None).map_err(|e| e.to_string())?;
                    let current_dir_name =
                        get_current_directory_name().map_err(|e| e.to_string())?;
                    ds.delete(current_dir_name.to_string(), Some(link_name.to_string()))
                        .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value(format!(
                        "Successfully removed link '{link_name}' from parent '{current_dir_name}'"
                    )))
                }
                (parent_entity, link_name) => {
                    let mut ds = DataStore::new(None).map_err(|e| e.to_string())?;
                    ds.delete(parent_entity.to_string(), Some(link_name.to_string()))
                        .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value(format!(
                        "Successfully removed link '{link_name}' from parent '{parent_entity}'"
                    )))
                }
            },
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Delete {
    fn args(&self) -> Vec<String> {
        self.args.to_vec()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
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

    // TODO: GH-45 Move the following to an integration test
    #[test]
    #[ignore = "GH-45: Should be an integration test due to DataStore dependency"]
    fn test_delete_run_expected_here_arg() {
        let args: Vec<String> = vec!["here".to_string()];
        let current_dir = std::env::current_dir().unwrap();
        let current_dir_name = current_dir.file_name().unwrap().to_str().unwrap();
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(format!(
            "Successfully removed all links of parent '{current_dir_name}'"
        )));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    // TODO: GH-45 Move the following to an integration test
    #[test]
    #[ignore = "GH-45: Should be an integration test due to DataStore dependency"]
    fn test_delete_run_expected_here_and_link_args() {
        let args: Vec<String> = vec!["here".to_string(), "google".to_string()];
        let current_dir = std::env::current_dir().unwrap();
        let current_dir_name = current_dir.file_name().unwrap().to_str().unwrap();
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(format!(
            "Successfully removed link 'google' from parent '{current_dir_name}'"
        )));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    // TODO: GH-45 Move the following to an integration test
    #[test]
    #[ignore = "GH-45: Should be an integration test due to DataStore dependency"]
    fn test_delete_run_expected_parent_entity_arg() {
        let args: Vec<String> = vec!["search-engines".to_string()];
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "Successfully removed all links of parent 'search-engines'".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    // TODO: GH-45 Move the following to an integration test
    #[test]
    #[ignore = "GH-45: Should be an integration test due to DataStore dependency"]
    fn test_delete_run_expected_parent_entity_and_link_args() {
        let args: Vec<String> = vec!["search-engines".to_string(), "google".to_string()];
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "Successfully removed link 'google' from parent 'search-engines'".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
