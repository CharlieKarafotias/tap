use crate::{
    commands::{Command, CommandInfo, CommandResult},
    utils::{
        cli_usage_table::DisplayCommandAsRow,
        command::get_current_directory_name,
        datastore::{DS, Datastore},
    },
};
use std::io::{Read, Seek, Write};

pub(crate) struct Delete {
    name: String,
    description: String,
    args: [String; 2],
}

impl Default for Delete {
    fn default() -> Self {
        Self {
            name: "-d, --delete".to_string(),
            description: "Deletes a link".to_string(),
            args: ["<Parent|here>".to_string(), "[Link]".to_string()],
        }
    }
}

impl CommandInfo for Delete {
    fn error_message(&self) -> String {
        "expected 1-2 arguments, see the Usage section with tap --delete --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("Tap --delete command will delete either a specific link or all links of a Parent Entity\n\n");
        s.push_str("Command Structure: tap --delete <Parent Entity | here> [Link Name]\n");
        s.push_str("Example Usage: \n\n");
        s.push_str("  - Delete all links: tap --delete search-engines\n");
        s.push_str("  - Delete specific link: tap --delete search-engines google\n");
        s.push_str("  - Delete all links associated to parent entity sharing name of current directory: tap --delete here\n");
        s
    }
}

impl<RW: Read + Write + Seek> Command<RW> for Delete {
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
            (Some("here"), None, None) => {
                let current_dir_name = get_current_directory_name().map_err(|e| e.to_string())?;
                ds.delete(&current_dir_name, None)
                    .map_err(|e| e.to_string())?;
                Ok(CommandResult::Value(format!(
                    "Successfully removed all links of parent '{current_dir_name}'"
                )))
            }
            (Some(parent_entity), None, None) => {
                ds.delete(parent_entity, None).map_err(|e| e.to_string())?;
                Ok(CommandResult::Value(format!(
                    "Successfully removed all links of parent '{parent_entity}'"
                )))
            }
            (Some("here"), Some(link_name), None) => {
                let current_dir_name = get_current_directory_name().map_err(|e| e.to_string())?;
                ds.delete(&current_dir_name, Some(link_name))
                    .map_err(|e| e.to_string())?;
                Ok(CommandResult::Value(format!(
                    "Successfully removed link '{link_name}' from parent '{current_dir_name}'"
                )))
            }
            (Some(parent_entity), Some(link_name), None) => {
                ds.delete(parent_entity, Some(link_name))
                    .map_err(|e| e.to_string())?;
                Ok(CommandResult::Value(format!(
                    "Successfully removed link '{link_name}' from parent '{parent_entity}'"
                )))
            }
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Delete {
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
    use std::io::Cursor;

    #[test]
    fn test_delete_run_expected_help_arg() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["--help".to_string()].into_iter();
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_run_unexpected_args() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec![
            "random".to_string(),
            "random2".to_string(),
            "random3".to_string(),
        ]
        .into_iter();
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_run_expected_here_arg() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["here".to_string()].into_iter();
        let current_dir = std::env::current_dir().unwrap();
        let current_dir_name = current_dir.file_name().unwrap().to_str().unwrap();
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(format!(
            "Successfully removed all links of parent '{current_dir_name}'"
        )));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_run_expected_here_and_link_args() {
        // Setup in memory db
        let mut ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));

        let current_dir = std::env::current_dir().unwrap();
        let current_dir_name = current_dir.file_name().unwrap().to_str().unwrap();

        ds.upsert_link(
            current_dir_name.to_string(),
            "google".to_string(),
            "www.google.com".to_string(),
        )
        .unwrap();

        // Setup call
        let args = vec!["here".to_string(), "google".to_string()].into_iter();
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(format!(
            "Successfully removed link 'google' from parent '{current_dir_name}'"
        )));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_run_expected_parent_entity_arg() {
        // Setup in memory db
        let mut ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        ds.upsert_link(
            "search-engines".to_string(),
            "google".to_string(),
            "www.google.com".to_string(),
        )
        .unwrap();

        let args = vec!["search-engines".to_string()].into_iter();
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "Successfully removed all links of parent 'search-engines'".to_string(),
        ));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_run_expected_parent_entity_and_link_args() {
        // Setup in memory db
        let mut ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        ds.upsert_link(
            "search-engines".to_string(),
            "google".to_string(),
            "www.google.com".to_string(),
        )
        .unwrap();

        let args = vec!["search-engines".to_string(), "google".to_string()].into_iter();
        let cmd = Delete::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(
            "Successfully removed link 'google' from parent 'search-engines'".to_string(),
        ));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }
}
