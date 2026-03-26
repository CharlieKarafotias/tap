use crate::{
    commands::{Command, CommandInfo, CommandResult},
    utils::{
        cli_usage_table::DisplayCommandAsRow,
        datastore::{DS, Datastore, ImportExportType},
    },
};
use std::io::{Read, Seek, Write};
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

impl CommandInfo for Export {
    fn error_message(&self) -> String {
        "expected 2 arguments, see the Usage section with tap --export --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut str = String::new();
        str.push_str("Tap export exports all links from Tap to 1 of 2 options:\n");
        str.push_str("  - Browser\n");
        str.push_str("  - Tap\n\n");
        str.push_str("Example Usage: \n");
        str.push_str("  tap --export <Browser | Tap> <destination folder>");
        str
    }
}

impl<RW: Read + Write + Seek> Command<RW> for Export {
    fn run<I: Iterator<Item = String>>(
        &self,
        mut args: I,
        mut ds: Datastore<RW>,
    ) -> Result<CommandResult, String> {
        let arg1 = args.next();
        let arg2 = args.next();
        let arg3 = args.next();

        match (arg1.as_deref(), arg2.as_deref(), arg3.as_deref()) {
            (Some("--help"), None, None) => Ok(CommandResult::Value(self.help_message())),
            (Some("Browser"), Some(f), None) => {
                let path_to_export = ds
                    .export(PathBuf::from(f), ImportExportType::Browser)
                    .map_err(|e| e.to_string())?;
                Ok(CommandResult::Value(format!(
                    "Successfully exported Bookmarks file to: {path_to_export}\nTo import into browser, use the \"Bookmark HTML file\" import type."
                )))
            }
            (Some("Tap"), Some(f), None) => {
                let path_to_export = ds
                    .export(PathBuf::from(f), ImportExportType::Tap)
                    .map_err(|e| e.to_string())?;
                Ok(CommandResult::Value(format!(
                    "Successfully exported Tap file to: {path_to_export}"
                )))
            }
            (Some(bad_browser), Some(_), None) => {
                Err(self.invalid_export_type_message(bad_browser))
            }
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
    use std::io::Cursor;
    use std::path::Path;

    #[test]
    fn test_export_run_expected_help_arg() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["--help".to_string()].into_iter();
        let cmd = Export::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_unexpected_args() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["random".to_string()].into_iter();
        let cmd = Export::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_bad_browser() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["bad browser".to_string(), "path".to_string()].into_iter();
        let cmd = Export::default();
        let expected: Result<CommandResult, String> =
            Err(cmd.invalid_export_type_message("bad browser"));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_browser() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let cmd = Export::default();
        let args = vec!["Browser".to_string(), "test.html".to_string()].into_iter();
        let expected = CommandResult::Value(
            "Successfully exported Bookmarks file to: test.html\nTo import into browser, use the \"Bookmark HTML file\" import type.".to_string(),
        );
        let res = cmd.run(args, ds).expect("Could not display export");
        assert_eq!(res, expected);

        // Clean up
        let path = Path::new("test.html");
        if path.exists() {
            fs::remove_file(path).expect("Could not remove test.html");
        }
    }

    #[test]
    fn test_export_run_tap() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let current_dir = std::env::current_dir().unwrap();
        let path = PathBuf::from(current_dir).join("test.tap");
        let cmd = Export::default();
        let args = vec!["Tap".to_string(), path.to_string_lossy().to_string()].into_iter();
        let expected =
            CommandResult::Value("Successfully exported Tap file to: test.tap".to_string());
        let res = cmd.run(args, ds).expect("Could not display export");
        assert_eq!(res, expected);

        // Clean up
        if path.exists() {
            fs::remove_file(path).expect("Could not remove test.tap");
        }
    }
}
