use crate::{
    commands::{Command, CommandInfo, CommandResult},
    utils::{
        cli_usage_table::DisplayCommandAsRow,
        command::get_current_directory_name,
        datastore::{DS, Datastore, Truncate},
        os_implementations::{LinkOpener, RealLinkOpener},
    },
};
use std::io::{Read, Seek, Write};

pub(crate) struct Here<T: LinkOpener> {
    name: String,
    description: String,
    args: [String; 1],
    opener: T,
}

impl Default for Here<RealLinkOpener> {
    fn default() -> Self {
        Self {
            name: "here".to_string(),
            description: "Open 1+ links (uses folder name)".to_string(),
            args: ["[Link]".to_string()],
            opener: RealLinkOpener,
        }
    }
}

#[cfg(test)]
impl<T: LinkOpener> Here<T> {
    fn with_opener(opener: T) -> Self {
        Self {
            name: "here".to_string(),
            description: "Open 1+ links (uses folder name)".to_string(),
            args: ["[Link]".to_string()],
            opener,
        }
    }
}

impl<T: LinkOpener> CommandInfo for Here<T> {
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
}

impl<RW: Read + Write + Seek + Truncate, T: LinkOpener> Command<RW> for Here<T> {
    fn run<I: Iterator<Item = String>>(
        &self,
        mut args: I,
        mut ds: Datastore<RW>,
    ) -> Result<CommandResult, String> {
        let arg1 = args.next();
        let arg2 = args.next();

        match (arg1.as_deref(), arg2.as_deref()) {
            (None, None) => {
                let parent_entity = get_current_directory_name().map_err(|e| e.to_string())?;
                let res = ds.read_parent(&parent_entity).map_err(|e| e.to_string())?;
                let mut res_str = "Opening links: [".to_string();
                for (_parent_entity, link, val) in res.iter() {
                    self.opener.open(val).map_err(|e| e.to_string())?;
                    res_str.push_str(format!("{link},").as_str());
                }
                res_str.push(']');
                Ok(CommandResult::Value(res_str))
            }
            (Some("--help"), None) => Ok(CommandResult::Value(self.help_message())),
            (Some(link), None) => {
                let parent_entity = get_current_directory_name().map_err(|e| e.to_string())?;
                let (_parent_entity, _link, val) = ds
                    .read_link(&parent_entity, link)
                    .map_err(|e| e.to_string())?;
                self.opener.open(&val).map_err(|e| e.to_string())?;
                Ok(CommandResult::Value("Opening link...".to_string()))
            }
            _ => Err(self.error_message()),
        }
    }
}

impl<T: LinkOpener> DisplayCommandAsRow for Here<T> {
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
    use crate::utils::datastore::{DS, Datastore};
    use crate::utils::os_implementations::{
        LinkOpener, OsImplementationError, OsImplementationErrorKind,
    };

    // Helper function to create an in-memory datastore with sample links
    fn create_test_ds() -> Datastore<Cursor<Vec<u8>>> {
        let mut ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let current_dir = std::env::current_dir().unwrap();
        let parent_entity = current_dir.file_name().unwrap().to_str().unwrap();

        let link_name = "google".to_string();
        let link_val = "https://google.com".to_string();
        ds.upsert_link(
            parent_entity.to_string(),
            link_name.clone(),
            link_val.clone(),
        )
        .expect("Failed to insert test link");
        ds
    }

    struct MockLinkOpener {
        pub should_fail: bool,
    }

    impl LinkOpener for MockLinkOpener {
        fn open(&self, _link: &str) -> Result<(), OsImplementationError> {
            if self.should_fail {
                Err(OsImplementationError {
                    kind: OsImplementationErrorKind::CommandFailedToStart,
                    message: "mock fail".into(),
                })
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_here_run_expected_help_arg() {
        let ds = create_test_ds();
        let args = vec!["--help".to_string()].into_iter();
        let cmd = Here::with_opener(MockLinkOpener { should_fail: false });
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_here_run_unexpected_args() {
        let ds = create_test_ds();
        let args = vec!["random".to_string(), "random2".to_string()].into_iter();
        let cmd = Here::with_opener(MockLinkOpener { should_fail: false });
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_here_run_all_links() {
        let ds = create_test_ds();
        let args = vec![].into_iter();
        let cmd = Here::with_opener(MockLinkOpener { should_fail: false });
        let expected_str = "Opening links: [google,]".to_string();
        let res = cmd.run(args, ds);
        assert_eq!(res, Ok(CommandResult::Value(expected_str)));
    }

    #[test]
    fn test_here_run_specific_link() {
        let ds = create_test_ds();
        let args = vec!["google".to_string()].into_iter();
        let cmd = Here::with_opener(MockLinkOpener { should_fail: false });
        let expected_str = "Opening link...".to_string();
        let res = cmd.run(args, ds);
        assert_eq!(res, Ok(CommandResult::Value(expected_str)));
    }
}
