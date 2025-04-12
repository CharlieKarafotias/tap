use crate::{
    commands::{Command, CommandResult},
    utils::cli_usage_table::DisplayCommandAsRow,
};

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
            "tap --export <Chrome | Edge | Firefox | Opera | Safari | Tap> <destination folder>"
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
                (bad_browser, _) => Err(self.bad_browser_message(bad_browser)),
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
        let expected: Result<CommandResult, String> = Err(cmd.bad_browser_message("bad browser"));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_chrome() {
        let cmd = Export::default();
        let args = vec!["Chrome", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Chrome: ./test.json".to_string(),
        );
        let res = cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_edge() {
        let cmd = Export::default();
        let args = vec!["Edge", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Edge: ./test.json".to_string(),
        );
        let res = cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_firefox() {
        let cmd = Export::default();
        let args = vec!["Firefox", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Firefox: ./test.json".to_string(),
        );
        let res = cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_opera() {
        let cmd = Export::default();
        let args = vec!["Opera", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Opera: ./test.json".to_string(),
        );
        let res = cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_safari() {
        let cmd = Export::default();
        let args = vec!["Safari", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Safari: ./test.json".to_string(),
        );
        let res = cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_run_tap() {
        let cmd = Export::default();
        let args = vec!["Tap", "./test.tap"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = CommandResult::Value(
            "TODO: Implement export functionality to Tap: ./test.tap".to_string(),
        );
        let res = cmd.run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }
}
