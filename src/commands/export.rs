use crate::{
    commands::{Command, CommandResult},
    utils::{
        cli_usage_table::DisplayCommandAsRow,
        tap_data_store::{DataStore, ImportExportType},
    },
};
use std::path::PathBuf;

pub(crate) struct Export {
    name: String,
    description: String,
    args: [String; 2],
}

impl Default for Export {
    fn default() -> Self {
        Self {
            name: "--export".to_string(),
            description: "Exports links to file".to_string(),
            args: ["<Browser|Tap>".to_string(), "<dest>".to_string()],
        }
    }
}

impl Export {
    fn invalid_export_type_message(&self, export_type: &str) -> String {
        format!(
            "unknown export type \"{export_type}\", see the Usage section with tap --export --help"
        )
    }
}

impl Command for Export {
    fn error_message(&self) -> String {
        "expected 2 arguments, see the Usage section with tap --export --help".to_string()
    }

    fn help_message(&self) -> String {
        format!(
            "Tap export exports all links from Tap to 1 of 2 options:\n\n  - {}\n  - {}\n\nExample Usage: {}",
            "A tap file",
            "A browser html bookmark file",
            "tap --export <Browser | Tap> <destination folder>"
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
                ("Browser", f) => {
                    let mut ds = DataStore::new(None).map_err(|e| e.to_string())?;
                    let path_to_export = ds
                        .export(PathBuf::from(f), ImportExportType::Browser)
                        .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value(format!(
                        "Successfully exported Bookmarks file to: {path_to_export}\nTo import into browser, use the \"Bookmark HTML file\" import type."
                    )))
                }
                ("Tap", f) => {
                    let mut ds = DataStore::new(None).map_err(|e| e.to_string())?;
                    let path_to_export = ds
                        .export(PathBuf::from(f), ImportExportType::Tap)
                        .map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value(format!(
                        "Successfully exported Tap file to: {path_to_export}"
                    )))
                }
                (bad_browser, _) => Err(self.invalid_export_type_message(bad_browser)),
            },
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Export {
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
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_export_run_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let cmd = Export::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let cmd = Export::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_bad_browser() {
        let args: Vec<String> = vec!["bad browser".to_string(), "path".to_string()];
        let cmd = Export::default();
        let expected: Result<CommandResult, String> =
            Err(cmd.invalid_export_type_message("bad browser"));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_browser() {
        let cmd = Export::default();
        let args = vec!["Browser", "test.html"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "Successfully exported Bookmarks file to: test.html\nTo import into browser, use the \"Bookmark HTML file\" import type.".to_string(),
        );
        let res = cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);

        // Clean up
        let path = Path::new("test.html");
        if path.exists() {
            fs::remove_file(path).expect("Could not remove test.html");
        }
    }

    #[test]
    fn test_export_run_tap() {
        let cmd = Export::default();
        let args = vec!["Tap", "test.tap"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            CommandResult::Value("Successfully exported Tap file to: test.tap".to_string());
        let res = cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);

        // Clean up
        let path = Path::new("test.tap");
        if path.exists() {
            fs::remove_file(path).expect("Could not remove test.tap");
        }
    }
}
