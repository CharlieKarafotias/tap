use crate::commands::{Command, CommandResult};

pub(crate) struct Import {
    name: String,
    description: String,
}

impl Default for Import {
    fn default() -> Self {
        Self {
            name: "--import".to_string(),
            description: "Imports a bookmark file into Tap".to_string(),
        }
    }
}

impl Import {
    fn bad_browser_message(&self, browser: &str) -> String {
        format!("unknown browser \"{browser}\", see the Usage section with tap --import --help")
    }
}

impl Command for Import {
    fn error_message(&self) -> String {
        "expected 2 arguments, see the Usage section with tap --import --help".to_string()
    }

    fn help_message(&self) -> String {
        format!(
            "Tap import imports a bookmark file from one of the following browsers into Tap:\n{}\n\nExample Usage: {}",
            "Chrome, Edge, Firefox, Opera, Safari, Tap",
            "tap --import [Chrome | Edge | Firefox | Opera | Safari | Tap] <bookmark file>"
        )
    }

    fn run(&self, parsed_args: Vec<String>) -> Result<CommandResult, String> {
        if parsed_args.len() == 1 && parsed_args[0] == "--help" {
            return Ok(CommandResult::Value(self.help_message()));
        }
        match (parsed_args[0].as_str(), parsed_args[1].as_str()) {
            ("Chrome", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement import functionality from Chrome: {f}"
            ))),
            ("Edge", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement import functionality from Edge: {f}"
            ))),
            ("Firefox", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement import functionality from Firefox: {f}"
            ))),
            ("Opera", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement import functionality from Opera: {f}"
            ))),
            ("Safari", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement import functionality from Safari: {f}"
            ))),
            ("Tap", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement import functionality from Tap: {f}"
            ))),
            _ => Err("Should never get here - Unsupported Import".to_string()),
        }
    }

    fn parse_args(&self, args: Vec<String>) -> Result<Vec<String>, String> {
        match args.len() {
            0 => Err(self.error_message()),
            1 => {
                if args[0] == "--help" {
                    Ok(args)
                } else {
                    Err(self.error_message())
                }
            }
            2 => match (args[0].as_str(), args[1].as_str()) {
                ("Chrome", _f) => Ok(args),
                ("Edge", _f) => Ok(args),
                ("Firefox", _f) => Ok(args),
                ("Opera", _f) => Ok(args),
                ("Safari", _f) => Ok(args),
                ("Tap", _f) => Ok(args),
                _ => Err(self.bad_browser_message(&args[0])),
            },
            _ => Err(self.error_message()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::export::Export;

    // parse_args test
    #[test]
    fn test_import_expected_two_args() {
        let args: Vec<String> = vec!["Chrome".to_string(), "random/path/to/file.json".to_string()];
        let import_cmd = Import::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = import_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let import_cmd = Import::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = import_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let import_cmd = Import::default();
        let expected: Result<Vec<String>, String> = Err(import_cmd.error_message());
        let res = import_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_bad_browser() {
        let args: Vec<String> = vec!["bad browser".to_string(), "path".to_string()];
        let import_cmd = Import::default();
        let expected: Result<Vec<String>, String> =
            Err(import_cmd.bad_browser_message("bad browser"));
        let res = import_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_chrome() {
        let import_cmd = Import::default();
        let args = vec!["Chrome", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement import functionality from Chrome: ./test.json".to_string(),
        );
        let res = import_cmd.run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_edge() {
        let import_cmd = Import::default();
        let args = vec!["Edge", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement import functionality from Edge: ./test.json".to_string(),
        );
        let res = import_cmd.run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_firefox() {
        let import_cmd = Import::default();
        let args = vec!["Firefox", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement import functionality from Firefox: ./test.json".to_string(),
        );
        let res = import_cmd.run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_opera() {
        let import_cmd = Import::default();
        let args = vec!["Opera", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement import functionality from Opera: ./test.json".to_string(),
        );
        let res = import_cmd.run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_safari() {
        let import_cmd = Import::default();
        let args = vec!["Safari", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement import functionality from Safari: ./test.json".to_string(),
        );
        let res = import_cmd.run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_run_tap() {
        let import_cmd = Import::default();
        let args = vec!["Tap", "./test.tap"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement import functionality from Tap: ./test.tap".to_string(),
        );
        let res = import_cmd.run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }
}
