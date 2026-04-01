use crate::{
    commands::{Command, CommandInfo, CommandResult},
    utils::{
        cli_usage_table::DisplayCommandAsRow,
        datastore::{DS, Datastore, Truncate},
        os_implementations::{LinkOpener, RealLinkOpener},
    },
};
use std::io::{Read, Seek, Write};

pub(crate) struct ParentEntity<T: LinkOpener> {
    name: String,
    description: String,
    args: [String; 1],
    opener: T,
}

impl Default for ParentEntity<RealLinkOpener> {
    fn default() -> Self {
        Self {
            name: "<Parent>".to_string(),
            description: "Open 1/all Links of Parent".to_string(),
            args: ["[Link]".to_string()],
            opener: RealLinkOpener,
        }
    }
}

#[cfg(test)]
impl<T: LinkOpener> ParentEntity<T> {
    fn with_opener(opener: T) -> Self {
        Self {
            name: "<Parent>".to_string(),
            description: "Open 1/all Links of Parent".to_string(),
            args: ["[Link]".to_string()],
            opener,
        }
    }
}

impl<T: LinkOpener> CommandInfo for ParentEntity<T> {
    fn error_message(&self) -> String {
        "expected 1-2 arguments, see the Usage section with tap --parent-entity --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s: String = "".to_string();
        s.push_str("Tap's core functionality is to open links. Tap Parent Entity command enables you to specify a Parent Entity and open either all or a specific link.\n\n");
        s.push_str("Command Structure: tap <Parent Entity> [Link Name]\n");
        s.push_str("Example Usage: \n\n");
        s.push_str("  - Open all Links of Parent Entity named search-engine: tap search-engine\n");
        s.push_str("  - Open specific Link named google in Parent Entity named search-engine: tap search-engine google\n");
        s
    }
}

impl<RW: Read + Write + Seek + Truncate, T: LinkOpener> Command<RW> for ParentEntity<T> {
    fn consumes_arg() -> bool {
        false
    }

    fn run<I: Iterator<Item = String>>(
        &self,
        mut args: I,
        mut ds: Datastore<RW>,
    ) -> Result<CommandResult, String> {
        let parent_entity = args.next();
        let link = args.next();
        let more_than_2_args = args.next();

        match (parent_entity, link, more_than_2_args) {
            (Some(parent), None, None) => {
                let res = ds.read_parent(&parent).map_err(|e| e.to_string())?;
                let mut res_str = "Opening links: [".to_string();
                for (_parent_entity, link, val) in res.iter() {
                    self.opener.open(val).map_err(|e| e.to_string())?;
                    res_str.push_str(format!("{link},").as_str());
                }
                res_str.push(']');
                Ok(CommandResult::Value(res_str))
            }
            (Some(parent), Some(link), None) => match (parent.as_str(), link.as_str()) {
                ("--parent-entity", "--help") => Ok(CommandResult::Value(self.help_message())),
                (parent_entity, link) => {
                    let (_parent_entity, _link, val) = ds
                        .read_link(parent_entity, link)
                        .map_err(|e| e.to_string())?;
                    self.opener.open(&val).map_err(|e| e.to_string())?;
                    Ok(CommandResult::Value("Opening link...".to_string()))
                }
            },
            _ => Err(self.error_message()),
        }
    }
}

impl<T: LinkOpener> DisplayCommandAsRow for ParentEntity<T> {
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
    use crate::utils::os_implementations::{
        LinkOpener, OsImplementationError, OsImplementationErrorKind,
    };
    use std::io::Cursor;

    // Helper to create a test datastore with a parent entity and links
    fn create_test_ds() -> Datastore<Cursor<Vec<u8>>> {
        let mut ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));

        // Add a parent entity "search-engine" with links
        ds.upsert_link(
            "search-engine".to_string(),
            "google".to_string(),
            "https://google.com".to_string(),
        )
        .unwrap();
        ds.upsert_link(
            "search-engine".to_string(),
            "bing".to_string(),
            "https://bing.com".to_string(),
        )
        .unwrap();

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
    fn test_parent_entity_run_expected_help_arg() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["--parent-entity".to_string(), "--help".to_string()].into_iter();
        let cmd = ParentEntity::with_opener(MockLinkOpener { should_fail: false });
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parent_entity_run_unexpected_args() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec![
            "random".to_string(),
            "random2".to_string(),
            "random3".to_string(),
        ]
        .into_iter();
        let cmd = ParentEntity::with_opener(MockLinkOpener { should_fail: false });
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parent_entity_run_all_links() {
        let mut ds = create_test_ds();
        let args = vec!["search-engine".to_string()].into_iter();
        let cmd = ParentEntity::with_opener(MockLinkOpener { should_fail: false });

        // Build expected string: "Opening links: [google,bing,]"
        let res_links = ds.read_parent("search-engine").unwrap();
        let expected_str = {
            let mut s = "Opening links: [".to_string();
            for (_p, link, _val) in res_links.iter() {
                s.push_str(format!("{link},").as_str());
            }
            s.push(']');
            s
        };

        let res = cmd.run(args, ds);
        assert_eq!(res, Ok(CommandResult::Value(expected_str)));
    }

    #[test]
    fn test_parent_entity_run_specific_link() {
        let ds = create_test_ds();
        let args = vec!["search-engine".to_string(), "google".to_string()].into_iter();
        let cmd = ParentEntity::with_opener(MockLinkOpener { should_fail: false });
        let expected_str = "Opening link...".to_string();
        let res = cmd.run(args, ds);
        assert_eq!(res, Ok(CommandResult::Value(expected_str)));
    }
}
