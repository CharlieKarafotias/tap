use crate::utils::command::get_current_directory_name;
use crate::utils::os_implementations::open_link;
use crate::utils::tap_data_store::ReadDataStore;
use crate::{
    commands::{Command, CommandResult},
    utils::cli_usage_table::DisplayCommandAsRow,
};

pub(crate) struct Here {
    name: String,
    description: String,
    args: [String; 1],
}

impl Default for Here {
    fn default() -> Self {
        Self {
            name: "here".to_string(),
            description: "Open 1+ links (uses folder name)".to_string(),
            args: ["[Link]".to_string()],
        }
    }
}

impl Command for Here {
    fn error_message(&self) -> String {
        "expected 0-1 arguments, see the Usage section with tap here --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s: String = "".to_string();
        s.push_str("Tap here uses the current working directory as the Parent Entity and will open either all or a specific link.\n\n");
        s.push_str("Command Structure: tap here [Link Name]\n");
        s.push_str("Example Usage: \n\n");
        s.push_str("  - Open all Links: tap here\n");
        s.push_str("  - Open specific Link: tap here google\n");
        s
    }

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        match args.len() {
            0 => {
                let parent_entity = get_current_directory_name().map_err(|e| e.to_string())?;
                let ds =
                    ReadDataStore::new(None, parent_entity.clone()).map_err(|e| e.to_string())?;
                let res = ds.read_parent(&parent_entity).map_err(|e| e.to_string())?;
                let mut res_str = "Opening links: [".to_string();
                for (link, val) in res.iter() {
                    open_link(val).map_err(|e| e.to_string())?;
                    res_str.push_str(format!("{link},").as_str());
                }
                res_str.push(']');
                Ok(CommandResult::Value(res_str))
            }
            1 => match args[0].as_str() {
                "--help" => Ok(CommandResult::Value(self.help_message())),
                link => {
                    let parent_entity = get_current_directory_name().map_err(|e| e.to_string())?;
                    let ds = ReadDataStore::new(None, parent_entity.to_string())
                        .map_err(|e| e.to_string())?;
                    let (_, val) = ds
                        .read_link(&parent_entity, link)
                        .map_err(|e| e.to_string())?;
                    open_link(&val).map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value("Opening link...".to_string()))
                }
            },
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Here {
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
    fn test_here_run_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let cmd = Here::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_here_run_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string(), "random2".to_string()];
        let cmd = Here::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[ignore = "GH-45: Should be an integration test due to DataStore dependency & os dependency"]
    fn test_here_run_all_links() {
        let args: Vec<String> = vec![];
        let cmd = Here::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement here functionality".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    #[ignore = "GH-45: Should be an integration test due to DataStore dependency & os dependency"]
    fn test_here_run_specific_link() {
        let args: Vec<String> = vec!["google".to_string()];
        let cmd = Here::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "TODO: Implement open functionality for here with Link Name google".to_string(),
        ));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
