use crate::{
    commands::{Command, CommandInfo, CommandResult, display_version},
    utils::{cli_usage_table::DisplayCommandAsRow, datastore::Datastore},
};
use std::io::{Read, Seek, Write};

pub(crate) struct Version {
    name: String,
    description: String,
    args: [String; 0],
}

impl Default for Version {
    fn default() -> Self {
        Self {
            name: "-v, --version".to_string(),
            description: "Show tap version".to_string(),
            args: [],
        }
    }
}

impl CommandInfo for Version {
    fn error_message(&self) -> String {
        "too many arguments, see the Usage section with tap --version --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("The version command shows the current version.\n\n");
        s.push_str("Example Usage: tap --version");
        s
    }
}

impl<RW: Read + Write + Seek> Command<RW> for Version {
    fn run<I: Iterator<Item = String>>(
        &self,
        mut args: I,
        _ds: Datastore<RW>,
    ) -> Result<CommandResult, String> {
        match args.next().as_deref() {
            None => Ok(CommandResult::Value(display_version())),
            Some("--help") => Ok(CommandResult::Value(self.help_message())),
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Version {
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
    fn test_version_run_expected_args() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec![].into_iter();
        let cmd = Version::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(display_version()));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_version_run_help_arg() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["--help".to_string()].into_iter();
        let cmd = Version::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_version_run_unexpected_args() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["random".to_string()].into_iter();
        let cmd = Version::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }
}
