use crate::commands::{Command, CommandResult, display_version};

pub(crate) struct Version {
    name: String,
    description: String,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            name: "(-v, --version)".to_string(),
            description: "Display the version".to_string(),
        }
    }
}

impl Command for Version {
    fn error_message(&self) -> String {
        "too many arguments, see the Usage section with tap --version --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("The version command shows the current version.\n\n");
        s.push_str("Example Usage: tap --version");
        s
    }

    fn run(&self, parsed_args: &[String]) -> Result<CommandResult, String> {
        if parsed_args.is_empty() {
            Ok(CommandResult::Value(display_version()))
        } else {
            Ok(CommandResult::Value(self.help_message()))
        }
    }

    fn parse_args<'a>(&self, args: &'a [String]) -> Result<&'a [String], String> {
        match args.len() {
            0 => Ok(args),
            1 => {
                if args[0] == "--help" {
                    Ok(args)
                } else {
                    Err(self.error_message())
                }
            }
            _ => Err(self.error_message()),
        }
    }
}
