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
            0 => Ok(CommandResult::Value(
                "TODO: Implement here functionality".to_string(),
            )),
            1 => match args[0].as_str() {
                "--help" => Ok(CommandResult::Value(self.help_message())),
                link => Ok(CommandResult::Value(format!(
                    "TODO: Implement open functionality for here with Link Name {link}"
                ))),
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
