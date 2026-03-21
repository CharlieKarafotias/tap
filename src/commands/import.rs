use crate::{
    commands::{Command, CommandInfo, CommandResult},
    utils::{
        cli_usage_table::DisplayCommandAsRow,
        datastore::{DS, Datastore, ImportExportType},
    },
};
use std::io::{Read, Seek, Write};
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

impl CommandInfo for Import {
    fn error_message(&self) -> String {
        "expected 2 arguments, see the Usage section with tap --import --help".to_string()
    }

    fn help_message(&self) -> String {
        format!(
            "Tap import imports a browser bookmark file into Tap. Import will overwrite existing links.\n\nExample Usage: {}",
            "tap --import <Browser | Tap> <bookmark file>"
        )
    }
}

impl<RW: Read + Write + Seek> Command<RW> for Import {
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
            (Some("Browser"), Some(f), None) => Ok(CommandResult::Value(format!(
                "TODO: Implement import functionality from Browser: {f}"
            ))),
            (Some("Tap"), Some(f), None) => {
                ds.import(PathBuf::from(f), ImportExportType::Tap)
                    .map_err(|e| e.to_string())?;
                Ok(CommandResult::Value("Import complete".to_string()))
            }
            (Some(bad_type), Some(_), None) => Err(self.bad_type_message(bad_type)),
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
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_import_expected_help_arg() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["--help".to_string()].into_iter();
        let cmd = Import::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_unexpected_args() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["random".to_string()].into_iter();
        let cmd = Import::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_bad_browser() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["bad browser".to_string(), "path".to_string()].into_iter();
        let cmd = Import::default();
        let expected: Result<CommandResult, String> = Err(cmd.bad_type_message("bad browser"));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_browser() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let cmd = Import::default();
        let args = vec!["Browser".to_string(), "test.html".to_string()].into_iter();
        let expected = CommandResult::Value(
            "TODO: Implement import functionality from Browser: test.html".to_string(),
        );
        let res = cmd.run(args, ds).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_tap() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let cmd = Import::default();
        let args = vec!["Tap".to_string(), "./test.tap".to_string()].into_iter();
        let expected = CommandResult::Value("Import complete".to_string());
        let res = cmd.run(args, ds).expect("Could not display import");
        assert_eq!(res, expected);
    }
}
