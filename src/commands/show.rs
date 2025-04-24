use crate::utils::command::get_current_directory_name;
use crate::utils::tap_data_store::ReadDataStore;
use crate::{
    commands::{Command, CommandResult},
    utils::cli_usage_table::DisplayCommandAsRow,
    utils::tap_data_store::Index,
};

pub(crate) struct Show {
    name: String,
    description: String,
    args: [String; 2],
}

impl Default for Show {
    fn default() -> Self {
        Self {
            name: "-s, --show".to_string(),
            description: "Shows links".to_string(),
            args: ["<Parent|here>".to_string(), "[Link]".to_string()],
        }
    }
}

impl Command for Show {
    fn error_message(&self) -> String {
        "expected 0-2 arguments, see the Usage section with tap --show --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("Tap --show command will show either a specific Link's value or all Link values of a Parent Entity\n\n");
        s.push_str("Command Structure(s):\n");
        s.push_str(
            "  - tap --show                                     (Show all Parent Entity values)\n",
        );
        s.push_str("  - tap --show <Parent Entity | here> [Link Name]  (Show specific/all Link values)\n\n");
        s.push_str("Example Usage: \n");
        s.push_str("  - tap --show search-engines        (Show all Link values)\n");
        s.push_str("  - tap --show search-engines google (Show specific Link value)\n");
        s.push_str("  - tap --show here                  (Show all Link values of Parent Entity - uses name of current directory)\n");
        s
    }

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        match args.len() {
            0 => {
                // Use Index parents
                let index = Index::new(None).unwrap();
                let parents = index.parents();
                let parent_entities: String = parents.iter().map(|s| format!("  {s}\n")).collect();
                Ok(CommandResult::Value(format!(
                    "Parent Entities:\n{}",
                    parent_entities.trim_end_matches('\n')
                )))
            }
            1 => match args[0].as_str() {
                "--help" => Ok(CommandResult::Value(self.help_message())),
                "here" => {
                    let parent_entity = get_current_directory_name().map_err(|e| e.to_string())?;
                    let ds = ReadDataStore::new(None, parent_entity.to_string())
                        .map_err(|e| e.to_string())?;
                    let links = ds.links(&parent_entity).map_err(|e| e.to_string())?;
                    let links_string: String = links.iter().map(|s| format!("  {s}\n")).collect();
                    Ok(CommandResult::Value(format!(
                        "Links of parent entity {parent_entity}:\n{}",
                        links_string.trim_end_matches('\n')
                    )))
                }
                parent_entity => {
                    let ds = ReadDataStore::new(None, parent_entity.to_string())
                        .map_err(|e| e.to_string())?;
                    let links = ds.links(parent_entity).map_err(|e| e.to_string())?;
                    let links_string: String = links.iter().map(|s| format!("  {s}\n")).collect();
                    Ok(CommandResult::Value(format!(
                        "Links of parent entity {parent_entity}:\n{}",
                        links_string.trim_end_matches('\n')
                    )))
                }
            },
            2 => match (args[0].as_str(), args[1].as_str()) {
                ("here", link_name) => {
                    let parent_entity = get_current_directory_name().map_err(|e| e.to_string())?;
                    let ds = ReadDataStore::new(None, parent_entity.to_string())
                        .map_err(|e| e.to_string())?;
                    let link_value = ds
                        .read_link(&parent_entity, link_name)
                        .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value(format!(
                        "{}: {}",
                        link_value.0, link_value.1
                    )))
                }
                (parent_entity, link_name) => {
                    let ds = ReadDataStore::new(None, parent_entity.to_string())
                        .map_err(|e| e.to_string())?;
                    let link_value = ds
                        .read_link(parent_entity, link_name)
                        .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value(format!(
                        "{}: {}",
                        link_value.0, link_value.1
                    )))
                }
            },
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Show {
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
    fn test_show_run_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let cmd = Show::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_run_unexpected_args() {
        let args: Vec<String> = vec![
            "random".to_string(),
            "random2".to_string(),
            "random3".to_string(),
        ];
        let cmd = Show::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[ignore = "GH-45: Should really be an integration test - move this out"]
    fn test_show_run_expected_here_arg() {
        let args: Vec<String> = vec!["here".to_string()];
        let cmd = Show::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement show functionality for here".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[ignore = "GH-45: Should really be an integration test - move this out"]
    fn test_show_run_expected_here_and_link_args() {
        let args: Vec<String> = vec!["here".to_string(), "google".to_string()];
        let cmd = Show::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement show functionality for here with Link Name google".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[ignore = "GH-45: Should really be an integration test - move this out"]
    fn test_show_run_expected_parent_entity_arg() {
        let args: Vec<String> = vec!["search-engines".to_string()];
        let cmd = Show::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement show functionality for Parent Entity: search-engines".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[ignore = "GH-45: Should really be an integration test - move this out"]
    fn test_show_run_expected_parent_entity_and_link_args() {
        let args: Vec<String> = vec!["search-engines".to_string(), "google".to_string()];
        let cmd = Show::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement show functionality for Parent Entity search-engines with Link Name google".to_string()
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
