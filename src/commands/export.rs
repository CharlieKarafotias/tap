use crate::commands::{Command, CommandResult};

pub(crate) struct Export {
    name: String,
    description: String,
}

impl Default for Export {
    fn default() -> Self {
        Self {
            name: "--export".to_string(),
            description: "Exports Tap links to a browser bookmark file".to_string(),
        }
    }
}

impl Export {
    fn bad_browser_message(&self, browser: &str) -> String {
        format!("unknown browser \"{browser}\", see the Usage section with tap --export --help")
    }
}

impl Command for Export {
    fn error_message(&self) -> String {
        "expected 2 arguments, see the Usage section with tap --export --help".to_string()
    }

    fn help_message(&self) -> String {
        format!(
            "Tap export exports all links from Tap to a bookmark file compatible with the following browsers:\n{}\n\nExample Usage: {}",
            "Chrome, Edge, Firefox, Opera, Safari, Tap",
            "tap --export [Chrome | Edge | Firefox | Opera | Safari | Tap] <destination folder>"
        )
    }

    fn run(&self, parsed_args: Vec<String>) -> Result<CommandResult, String> {
        if parsed_args.len() == 1 && parsed_args[0] == "--help" {
            return Ok(CommandResult::Value(self.help_message()));
        }
        match (parsed_args[0].as_str(), parsed_args[1].as_str()) {
            ("Chrome", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement export functionality to Chrome: {f}"
            ))),
            ("Edge", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement export functionality to Edge: {f}"
            ))),
            ("Firefox", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement export functionality to Firefox: {f}"
            ))),
            ("Opera", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement export functionality to Opera: {f}"
            ))),
            ("Safari", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement export functionality to Safari: {f}"
            ))),
            ("Tap", f) => Ok(CommandResult::Value(format!(
                "TODO: Implement export functionality to Tap: {f}"
            ))),
            _ => Err("Should never get here - Unsupported Export".to_string()),
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

    // parse_args test
    #[test]
    fn test_export_expected_two_args() {
        let args: Vec<String> = vec!["Chrome".to_string(), "random/path/to/file.json".to_string()];
        let export_cmd = Export::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = export_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let export_cmd = Export::default();
        let expected: Result<Vec<String>, String> = Ok(args.clone());
        let res = export_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let export_cmd = Export::default();
        let expected: Result<Vec<String>, String> = Err(export_cmd.error_message());
        let res = export_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_bad_browser() {
        let args: Vec<String> = vec!["bad browser".to_string(), "path".to_string()];
        let export_cmd = Export::default();
        let expected: Result<Vec<String>, String> =
            Err(export_cmd.bad_browser_message("bad browser"));
        let res = export_cmd.parse_args(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_chrome() {
        let export_cmd = Export::default();
        let args = vec!["Chrome", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Chrome: ./test.json".to_string(),
        );
        let res = export_cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_edge() {
        let export_cmd = Export::default();
        let args = vec!["Edge", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Edge: ./test.json".to_string(),
        );
        let res = export_cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_firefox() {
        let export_cmd = Export::default();
        let args = vec!["Firefox", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Firefox: ./test.json".to_string(),
        );
        let res = export_cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_opera() {
        let export_cmd = Export::default();
        let args = vec!["Opera", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Opera: ./test.json".to_string(),
        );
        let res = export_cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_safari() {
        let export_cmd = Export::default();
        let args = vec!["Safari", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Safari: ./test.json".to_string(),
        );
        let res = export_cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_tap() {
        let export_cmd = Export::default();
        let args = vec!["Tap", "./test.tap"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Tap: ./test.tap".to_string(),
        );
        let res = export_cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }
}
