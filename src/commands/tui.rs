use crate::{
    commands::{Command, CommandResult},
    utils::cli_usage_table::DisplayCommandAsRow,
};

pub(crate) struct Tui {
    name: String,
    description: String,
    args: [String; 0],
}

impl Default for Tui {
    fn default() -> Self {
        Self {
            name: "--tui".to_string(),
            description: "Launch the interactive UI".to_string(),
            args: [],
        }
    }
}

impl Command for Tui {
    fn error_message(&self) -> String {
        "too many arguments, see the Usage section with tap --tui --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("Opens a terminal user interface to facilitate adding, updating, and deleting links.\n\n");
        s.push_str("Example Usage: tap --tui");
        s
    }

    fn run(&self, args: Vec<String>) -> Result<CommandResult, String> {
        match args.len() {
            0 => todo!("Implement TUI Functionality"),
            1 => {
                if args[0] == "--help" {
                    Ok(CommandResult::Value(self.help_message()))
                } else {
                    Err(self.error_message())
                }
            }
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Tui {
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
    #[should_panic] // TODO: remove after implementing tui functionality
    fn test_tui_run_expected_args() {
        let args: Vec<String> = vec![];
        let cmd = Tui::default();
        let res = cmd.run(args);
    }

    #[test]
    fn test_tui_run_expected_help_arg() {
        let args: Vec<String> = vec!["--help".to_string()];
        let cmd = Tui::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_tui_run_unexpected_args() {
        let args: Vec<String> = vec!["random".to_string()];
        let cmd = Tui::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args);
        assert_eq!(res, expected);
    }
}
