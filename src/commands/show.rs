use crate::{
    commands::{Command, CommandInfo, CommandResult},
    utils::{
        cli_usage_table::DisplayCommandAsRow,
        command::get_current_directory_name,
        datastore::{DS, Datastore},
    },
};
use std::io::{Read, Seek, Write};
pub(crate) struct Show {
    name: String,
    description: String,
    args: [String; 2],
}

impl Default for Show {
    fn default() -> Self {
        Self {
            name: "-s, --show".to_string(),
            description: "Shows links".to_string(),
            args: ["<Parent|here>".to_string(), "[Link]".to_string()],
        }
    }
}

impl CommandInfo for Show {
    fn error_message(&self) -> String {
        "expected 0-2 arguments, see the Usage section with tap --show --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("Tap --show command will show either a specific Link's value or all Link values of a Parent Entity\n\n");
        s.push_str("Command Structure(s):\n");
        s.push_str(
            "  - tap --show                                     (Show all Parent Entity values)\n",
        );
        s.push_str("  - tap --show <Parent Entity | here> [Link Name]  (Show specific/all Link values)\n\n");
        s.push_str("Example Usage: \n");
        s.push_str("  - tap --show search-engines        (Show all Link values)\n");
        s.push_str("  - tap --show search-engines google (Show specific Link value)\n");
        s.push_str("  - tap --show here                  (Show all Link values of Parent Entity - uses name of current directory)\n");
        s
    }
}

impl<RW: Read + Write + Seek> Command<RW> for Show {
    fn run<I: Iterator<Item = String>>(
        &self,
        mut args: I,
        mut ds: Datastore<RW>,
    ) -> Result<CommandResult, String> {
        let arg1 = args.next();
        let arg2 = args.next();
        let arg3 = args.next();

        match (arg1.as_deref(), arg2.as_deref(), arg3.as_deref()) {
            (None, None, None) => {
                let parents = ds.parents().map_err(|e| e.to_string())?;
                let parent_entities: String = parents.iter().map(|s| format!("  {s}\n")).collect();
                Ok(CommandResult::Value(format!(
                    "Parent Entities:\n{}",
                    parent_entities.trim_end_matches('\n')
                )))
            }
            (Some("--help"), None, None) => Ok(CommandResult::Value(self.help_message())),
            (Some("here"), None, None) => {
                let parent_entity = get_current_directory_name().map_err(|e| e.to_string())?;
                let links = ds.read_parent(&parent_entity).map_err(|e| e.to_string())?;
                let links_string: String = links
                    .iter()
                    .map(|(_parent_entity, link, _val)| format!("  {link}\n"))
                    .collect();
                Ok(CommandResult::Value(format!(
                    "Links of parent entity {parent_entity}:\n{}",
                    links_string.trim_end_matches('\n')
                )))
            }
            (Some(parent_entity), None, None) => {
                let links = ds.read_parent(parent_entity).map_err(|e| e.to_string())?;
                let links_string: String = links
                    .iter()
                    .map(|(_parent_entity, link, _val)| format!("  {link}\n"))
                    .collect();
                Ok(CommandResult::Value(format!(
                    "Links of parent entity {parent_entity}:\n{}",
                    links_string.trim_end_matches('\n')
                )))
            }
            (Some("here"), Some(link_name), None) => {
                let parent_entity = get_current_directory_name().map_err(|e| e.to_string())?;
                let link_value = ds
                    .read_link(&parent_entity, link_name)
                    .map_err(|e| e.to_string())?;
                Ok(CommandResult::Value(format!(
                    "{}: {}",
                    link_value.1, link_value.2
                )))
            }

            (Some(parent_entity), Some(link_name), None) => {
                let link_value = ds
                    .read_link(parent_entity, link_name)
                    .map_err(|e| e.to_string())?;
                Ok(CommandResult::Value(format!(
                    "{}: {}",
                    link_value.1, link_value.2
                )))
            }
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Show {
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

    fn get_here_value() -> String {
        let current_dir = std::env::current_dir().unwrap();
        let here_parent = current_dir.file_name().unwrap().to_str().unwrap();
        here_parent.to_string()
    }

    // Helper to create a test datastore
    fn create_test_ds() -> Datastore<Cursor<Vec<u8>>> {
        let mut ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));

        // Add a parent entity with some links
        let parent_entity = "search-engines".to_string();
        ds.upsert_link(
            parent_entity.clone(),
            "google".to_string(),
            "https://google.com".to_string(),
        )
        .unwrap();
        ds.upsert_link(
            parent_entity.clone(),
            "bing".to_string(),
            "https://bing.com".to_string(),
        )
        .unwrap();

        // Also add a folder for "here" tests

        ds.upsert_link(
            get_here_value(),
            "duckduckgo".to_string(),
            "https://duckduckgo.com".to_string(),
        )
        .unwrap();

        ds
    }

    #[test]
    fn test_show_run_expected_help_arg() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["--help".to_string()].into_iter();
        let cmd = Show::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_run_unexpected_args() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec![
            "random".to_string(),
            "random2".to_string(),
            "random3".to_string(),
        ]
        .into_iter();
        let cmd = Show::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_run_expected_here_arg() {
        let mut ds = create_test_ds();
        let args = vec!["here".to_string()].into_iter();
        let cmd = Show::default();

        let parent_entity = get_here_value();
        let links = ds.read_parent(&parent_entity).unwrap();
        let links_string: String = links
            .iter()
            .map(|(_p, link, _val)| format!("  {link}\n"))
            .collect();
        let expected_str = format!(
            "Links of parent entity {parent_entity}:\n{}",
            links_string.trim_end_matches('\n')
        );

        let res = cmd.run(args, ds);
        assert_eq!(res, Ok(CommandResult::Value(expected_str)));
    }

    #[test]
    fn test_show_run_expected_here_and_link_args() {
        let mut ds = create_test_ds();
        let args = vec!["here".to_string(), "duckduckgo".to_string()].into_iter();
        let cmd = Show::default();

        let parent_entity = get_here_value();
        let (_p, link, val) = ds.read_link(&parent_entity, "duckduckgo").unwrap();
        let expected_str = format!("{link}: {val}");

        let res = cmd.run(args, ds);
        assert_eq!(res, Ok(CommandResult::Value(expected_str)));
    }

    #[test]
    fn test_show_run_expected_parent_entity_arg() {
        let mut ds = create_test_ds();
        let args = vec!["search-engines".to_string()].into_iter();
        let cmd = Show::default();

        let links = ds.read_parent("search-engines").unwrap();
        let links_string: String = links
            .iter()
            .map(|(_p, link, _val)| format!("  {link}\n"))
            .collect();
        let expected_str = format!(
            "Links of parent entity search-engines:\n{}",
            links_string.trim_end_matches('\n')
        );

        let res = cmd.run(args, ds);
        assert_eq!(res, Ok(CommandResult::Value(expected_str)));
    }

    #[test]
    fn test_show_run_expected_parent_entity_and_link_args() {
        let mut ds = create_test_ds();
        let args = vec!["search-engines".to_string(), "google".to_string()].into_iter();
        let cmd = Show::default();

        let (_p, link, val) = ds.read_link("search-engines", "google").unwrap();
        let expected_str = format!("{link}: {val}");

        let res = cmd.run(args, ds);
        assert_eq!(res, Ok(CommandResult::Value(expected_str)));
    }
}
