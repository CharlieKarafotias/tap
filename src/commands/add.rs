use crate::{
    commands::{Command, CommandResult},
    utils::cli_usage_table::DisplayCommandAsRow,
    utils::command::get_current_directory_name,
    utils::tap_data_store::DataStore,
};

pub(crate) struct Add {
    name: String,
    description: String,
    args: [String; 3],
}

impl Default for Add {
    fn default() -> Self {
        Self {
            name: "-a, --add".to_string(),
            description: "Add a new link".to_string(),
            args: [
                "<Parent|here>".to_string(),
                "<Link>".to_string(),
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
                ("here", link_name, value) => {
                    let mut ds = DataStore::new(None).map_err(|e| e.to_string())?;
                    let current_dir_name =
                        get_current_directory_name().map_err(|e| e.to_string())?;
                    ds.add_link(
                        current_dir_name.to_string(),
                        link_name.to_string(),
                        value.to_string(),
                    )
                    .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value(format!(
                        "Successfully added {link_name} with value {value} to parent entity {current_dir_name}"
                    )))
                }
                (parent_entity, link_name, value) => {
                    let mut ds = DataStore::new(None).map_err(|e| e.to_string())?;
                    ds.add_link(
                        parent_entity.to_string(),
                        link_name.to_string(),
                        value.to_string(),
                    )
                    .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value(format!(
                        "Successfully added {link_name} with value {value} to parent entity {parent_entity}"
                    )))
                }
            },
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Add {
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
        let current_dir = std::env::current_dir().unwrap();
        let current_dir_name = current_dir.file_name().unwrap().to_str().unwrap();
        let cmd = Add::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(format!(
            "Successfully added google with value https://google.com to parent entity {current_dir_name}"
        )));
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
        let expected: Result<CommandResult, String> =
            Ok(CommandResult::Value("Successfully added google with value https://google.com to parent entity search-engines".to_string()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
