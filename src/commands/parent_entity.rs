use crate::{
    commands::{Command, CommandResult},
    utils::cli_usage_table::DisplayCommandAsRow,
    utils::os_implementations::open_link,
    utils::tap_data_store::ReadDataStore,
};

pub(crate) struct ParentEntity {
    name: String,
    description: String,
    args: [String; 1],
}

impl Default for ParentEntity {
    fn default() -> Self {
        Self {
            name: "<Parent>".to_string(),
            description: "Open 1/all Links of Parent".to_string(),
            args: ["[Link]".to_string()],
        }
    }
}

impl Command for ParentEntity {
    fn error_message(&self) -> String {
        "expected 1-2 arguments, see the Usage section with tap --parent-entity --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s: String = "".to_string();
        s.push_str("Tap's core functionality is to open links. Tap Parent Entity command enables you to specify a Parent Entity and open either all or a specific link.\n\n");
        s.push_str("Command Structure: tap <Parent Entity> [Link Name]\n");
        s.push_str("Example Usage: \n\n");
        s.push_str("  - Open all Links of Parent Entity named search-engine: tap search-engine\n");
        s.push_str("  - Open specific Link named google in Parent Entity named search-engine: tap search-engine google\n");
        s
    }

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        match args.len() {
            1 => {
                let parent_entity = args[0].as_str();
                let ds = ReadDataStore::new(None, parent_entity.to_string())
                    .map_err(|e| e.to_string())?;
                let res = ds.read_parent(parent_entity).map_err(|e| e.to_string())?;
                let mut res_str = "Opening links: [".to_string();
                for (link, val) in res.iter() {
                    open_link(val).map_err(|e| e.to_string())?;
                    res_str.push_str(format!("{link},").as_str());
                }
                res_str.push(']');
                Ok(CommandResult::Value(res_str))
            }
            2 => match (args[0].as_str(), args[1].as_str()) {
                ("--parent-entity", "--help") => Ok(CommandResult::Value(self.help_message())),
                (parent_entity, link) => {
                    let ds = ReadDataStore::new(None, parent_entity.to_string())
                        .map_err(|e| e.to_string())?;
                    let (_, val) = ds
                        .read_link(parent_entity, link)
                        .map_err(|e| e.to_string())?;
                    open_link(&val).map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value("Opening link...".to_string()))
                }
            },
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for ParentEntity {
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
    fn test_parent_entity_run_expected_help_arg() {
        let args: Vec<String> = vec!["--parent-entity".to_string(), "--help".to_string()];
        let cmd = ParentEntity::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parent_entity_run_unexpected_args() {
        let args: Vec<String> = vec![
            "random".to_string(),
            "random2".to_string(),
            "random3".to_string(),
        ];
        let cmd = ParentEntity::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[ignore = "GH-45: Should be an integration test due to DataStore dependency & os dependency"]
    fn test_parent_entity_run_all_links() {
        let args: Vec<String> = vec!["search-engine".to_string()];
        let cmd = ParentEntity::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement open functionality for Parent Entity: search-engine".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[ignore = "GH-45: Should be an integration test due to DataStore dependency & os dependency"]
    fn test_parent_entity_run_specific_link() {
        let args: Vec<String> = vec!["search-engine".to_string(), "google".to_string()];
        let cmd = ParentEntity::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value("TODO: Implement open functionality for Parent Entity search-engine with Link Name google".to_string()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
