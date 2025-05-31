use crate::{
    commands::{Command, CommandResult},
    utils::cli_usage_table::DisplayCommandAsRow,
    utils::tap_data_store::{DataStore, ImportExportType},
};
use std::path::PathBuf;

pub(crate) struct Import {
    name: String,
    description: String,
    args: [String; 2],
}

impl Default for Import {
    fn default() -> Self {
        Self {
            name: "--import".to_string(),
            description: "Imports links from file".to_string(),
            args: ["<Browser|Tap>".to_string(), "<bookmark file>".to_string()],
        }
    }
}

impl Import {
    fn bad_type_message(&self, import_type: &str) -> String {
        format!(
            "unknown import type \"{import_type}\", see the Usage section with tap --import --help"
        )
    }
}

impl Command for Import {
    fn error_message(&self) -> String {
        "expected 2 arguments, see the Usage section with tap --import --help".to_string()
    }

    fn help_message(&self) -> String {
        format!(
            "Tap import imports a browser bookmark file into Tap. Import will overwrite existing links.\n\nExample Usage: {}",
            "tap --import <Browser | Tap> <bookmark file>"
        )
    }

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        match args.len() {
            0 => Err(self.error_message()),
            1 => {
                if args[0] == "--help" {
                    Ok(CommandResult::Value(self.help_message()))
                } else {
                    Err(self.error_message())
                }
            }
            2 => match (args[0].as_str(), args[1].as_str()) {
                ("Browser", f) => Ok(CommandResult::Value(format!(
                    "TODO: Implement import functionality from Browser: {f}"
                ))),
                ("Tap", f) => {
                    let mut ds = DataStore::new(None).map_err(|e| e.to_string())?;
                    ds.import(PathBuf::from(f), ImportExportType::Tap)
                        .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value("Import complete".to_string()))
                }
                (bad_type, _) => Err(self.bad_type_message(bad_type)),
            },
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Import {
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
    fn test_import_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let cmd = Import::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let cmd = Import::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_bad_browser() {
        let args: Vec<String> = vec!["bad browser".to_string(), "path".to_string()];
        let cmd = Import::default();
        let expected: Result<CommandResult, String> = Err(cmd.bad_type_message("bad browser"));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_browser() {
        let cmd = Import::default();
        let args = vec!["Browser", "test.html"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement import functionality from Browser: test.html".to_string(),
        );
        let res = cmd.run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    #[ignore = "GH-45: Should be an integration test due to DataStore dependency"]
    fn test_import_run_tap() {
        let cmd = Import::default();
        let args = vec!["Tap", "./test.tap"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value("Import complete".to_string());
        let res = cmd.run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }
}
