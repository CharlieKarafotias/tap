use std::{fmt, fs, fs::File, path::PathBuf};

enum FileType {
    Data,
    Index,
}

pub(crate) struct DataStore {
    data: Data,
    index: Index,
}

struct Data {
    path: PathBuf,
    state: Vec<(String, Vec<(String, String)>)>, // parent, (link, value)
}

// Publicly exposed
impl Data {
    pub fn new(path: Option<PathBuf>) -> Result<Self, TapDataStoreError> {
        if let Some(path) = path {
            if path.exists() {
                let file_as_str = fs::read_to_string(&path).map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::FileReadFailed,
                    message: format!("Could not read data file at {}: {e}", path.display()),
                })?;
                let state = Data::parse_file(&file_as_str)?;
                Ok(Self { path, state })
            } else {
                File::create_new(&path).map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::FileCreateFailed,
                    message: format!("Could not create data file: {e}"),
                })?;
                Ok(Self {
                    path,
                    state: vec![],
                })
            }
        } else {
            let executable_parent_dir = get_parent_dir_of_tap()?;
            let tap_data_path = executable_parent_dir.join(".tap_data");
            File::create_new(&tap_data_path).map_err(|e| TapDataStoreError {
                kind: TapDataStoreErrorKind::FileCreateFailed,
                message: format!("Could not create data file: {e}"),
            })?;
            Ok(Self {
                path: tap_data_path,
                state: vec![],
            })
        }
    }

    pub fn add_link(
        &mut self,
        parent: String,
        link: String,
        value: String,
    ) -> Result<(), TapDataStoreError> {
        todo!("Impl add link for Data")
    }

    pub fn get(&self, parent: String, link: Option<String>) -> Result<String, TapDataStoreError> {
        todo!("Impl get (links, link) for Data")
    }

    pub fn remove_link(&mut self, parent: String, link: String) -> Result<(), TapDataStoreError> {
        todo!("Impl remove link for Data")
    }

    pub fn upsert_link(
        &mut self,
        parent: String,
        link: String,
        value: String,
    ) -> Result<(), TapDataStoreError> {
        todo!("Impl upsert link for Data")
    }
}

#[cfg(test)]
mod data_public {
    use super::{Data, FileType, get_test_file_path};
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_new_no_path_correct_file_name() {
        let expected_file_name = ".tap_data";
        let mut data = Data::new(None).unwrap();
        assert!(data.path.to_str().unwrap().ends_with(expected_file_name));
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_new_with_path_correct_file_name() {
        let expected_file_name =
            get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(PathBuf::from(&expected_file_name))).unwrap();
        assert_eq!(data.path, expected_file_name);
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_set_state_correct() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        fs::write(
            &data_path,
            "parent1->\nlink1|value1\nlink2|value2\nparent2 ->\nlink3|value3\nlink4|value4",
        )
        .unwrap();
        let mut data = Data::new(Some(data_path)).unwrap();
        assert_eq!(
            data.state,
            vec![
                (
                    "parent1".to_string(),
                    vec![
                        ("link1".to_string(), "value1".to_string()),
                        ("link2".to_string(), "value2".to_string())
                    ]
                ),
                (
                    "parent2 ".to_string(),
                    vec![
                        ("link3".to_string(), "value3".to_string()),
                        ("link4".to_string(), "value4".to_string())
                    ]
                ),
            ]
        );
        data.cleanup().expect("Could not clean up data store");
    }
}

// Private
impl Data {
    fn parse_file(
        file_as_str: &str,
    ) -> Result<Vec<(String, Vec<(String, String)>)>, TapDataStoreError> {
        fn no_parent_error(
            parent: &str,
            links: &[(String, String)],
        ) -> Result<(), TapDataStoreError> {
            if !links.is_empty() && parent.is_empty() {
                return Err(TapDataStoreError {
                    kind: TapDataStoreErrorKind::ParseError,
                    message: format!(
                        "Links in a data file must have a parent. The following links do not have a parent: {}",
                        links
                            .iter()
                            .map(|(l, _)| l.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                });
            }
            Ok(())
        }

        fn update_state_reset_temps(
            parent: &mut str,
            links: &mut Vec<(String, String)>,
            state: &mut Vec<(String, Vec<(String, String)>)>,
        ) {
            if !parent.is_empty() && !links.is_empty() {
                state.push((parent.to_string(), links.clone()));
                links.clear();
            }
        }

        let mut state = vec![];
        let mut temp_parent = String::new();
        let mut temp_links: Vec<(String, String)> = vec![];
        for line in file_as_str.lines() {
            if line.ends_with("->") {
                // This is a parent line
                // If links not empty but no parent, this is an error
                no_parent_error(&temp_parent, &temp_links)?;
                // If temp holders not empty, done with current parent, add to state
                update_state_reset_temps(&mut temp_parent, &mut temp_links, &mut state);
                // NOTE: silent error if parent has no links (this is fine, not stored in internal state)
                temp_parent = line.trim_end_matches("->").to_string();
                validate_parent(&temp_parent)?;
            } else if line.contains('|') {
                // This is a link line
                // TODO: in future, would be nice to support escaped pipes
                let (link, value) = line
                    .split_once('|')
                    .ok_or(TapDataStoreError {
                        kind: TapDataStoreErrorKind::ParseError,
                        message: "A link/value line of a data file is expected to contain '|' character separating link and value. For example, google|https://google.com".to_string(),
                    })?;
                validate_link(link)?;
                temp_links.push((link.to_string(), value.to_string()));
            } else {
                return Err(TapDataStoreError {
                    kind: TapDataStoreErrorKind::ParseError,
                    message: format!(
                        "Unknown format for data file. Line '{line}' does not match expected format of parent ->\\n link|value"
                    ),
                });
            }
        }
        // When out of lines, update state
        no_parent_error(&temp_parent, &temp_links)?;
        update_state_reset_temps(&mut temp_parent, &mut temp_links, &mut state);

        Ok(state)
    }

    fn state_to_file_string(&self) -> String {
        // Sort state based on parent, then by link
        let mut sorted_state: Vec<(String, Vec<(String, String)>)> = self
            .state
            .iter()
            .map(|(parent, links)| {
                let mut sorted_links = links.clone();
                sorted_links.sort_by(|a, b| a.0.cmp(&b.0));
                (parent.clone(), sorted_links)
            })
            .collect();
        sorted_state.sort_by(|a, b| a.0.cmp(&b.0));

        let mut res = String::new();
        for (parent, links) in sorted_state {
            res.push_str(&format!("{}->\n", parent));
            for (link, value) in links {
                res.push_str(&format!("  {}|{}\n", link, value));
            }
        }
        res
    }

    fn save_to_file(&self) -> Result<(), TapDataStoreError> {
        let str = self.state_to_file_string();
        fs::write(&self.path, str).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::FileWriteFailed,
            message: format!("Could not write data file: {}", e),
        })
    }
}

#[cfg(test)]
mod data_private {
    use super::{Data, FileType, TapDataStoreErrorKind, get_test_file_path};
    use std::fs;
    use std::path::PathBuf;

    fn cleanup_test_file(file_path: &PathBuf) {
        fs::remove_file(file_path).expect("Could not remove test file");
    }

    #[test]
    fn test_parse_file_empty() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        fs::write(&data_path, "").unwrap();
        let res = Data::parse_file(fs::read_to_string(&data_path).unwrap().as_str())
            .expect("Could not parse file");
        assert_eq!(res, vec![]);
        cleanup_test_file(&data_path);
    }
    #[test]
    fn test_parse_file_valid_one_parent() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        fs::write(&data_path, "parent1->\nlink1|value1\nlink2|value2").unwrap();
        let res = Data::parse_file(fs::read_to_string(&data_path).unwrap().as_str())
            .expect("Could not parse file");
        assert_eq!(
            res,
            vec![(
                "parent1".to_string(),
                vec![
                    ("link1".to_string(), "value1".to_string()),
                    ("link2".to_string(), "value2".to_string())
                ]
            ),]
        );
        cleanup_test_file(&data_path);
    }

    #[test]
    fn test_parse_file_valid_two_parents() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        fs::write(&data_path, "search engines->\ngoogle|www.google.com\nyahoo|www.yahoo.com\ncoding->\ngh|https://github.com").unwrap();
        let res = Data::parse_file(fs::read_to_string(&data_path).unwrap().as_str())
            .expect("Could not parse file");
        assert_eq!(
            res,
            vec![
                (
                    "search engines".to_string(),
                    vec![
                        ("google".to_string(), "www.google.com".to_string()),
                        ("yahoo".to_string(), "www.yahoo.com".to_string())
                    ]
                ),
                (
                    "coding".to_string(),
                    vec![("gh".to_string(), "https://github.com".to_string())]
                ),
            ]
        );
        cleanup_test_file(&data_path);
    }

    #[test]
    fn test_parse_file_invalid_links() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        fs::write(
            &data_path,
            "search engines->\ngoogle|www.google.com\nyahoo|www.yahoo.com\ninvalid link",
        )
        .unwrap();
        let res = Data::parse_file(fs::read_to_string(&data_path).unwrap().as_str());
        assert_eq!(res.unwrap_err().kind, TapDataStoreErrorKind::ParseError);
        cleanup_test_file(&data_path);
    }

    #[test]
    fn test_parse_file_invalid_parent() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        fs::write(&data_path, "invalid parent->\n").unwrap();
        let res = Data::parse_file(fs::read_to_string(&data_path).unwrap().as_str())
            .expect("Could not parse file");
        // Silent error, if parent has no links no big deal
        assert_eq!(res, vec![]);
        cleanup_test_file(&data_path);
    }

    #[test]
    fn test_parse_file_invalid_random_file() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        fs::write(
            &data_path,
            "Something that is completely not a data file was read",
        )
        .unwrap();
        let res = Data::parse_file(fs::read_to_string(&data_path).unwrap().as_str());
        assert_eq!(res.unwrap_err().kind, TapDataStoreErrorKind::ParseError);
        cleanup_test_file(&data_path);
    }

    #[test]
    fn test_state_to_file_string_empty() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path)).unwrap();
        assert_eq!(data.state_to_file_string(), "");
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_state_to_file_string_spacing() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path)).unwrap();
        data.state = vec![(
            "parent1".to_string(),
            vec![("link1".to_string(), "value1".to_string())],
        )];
        assert_eq!(data.state_to_file_string(), "parent1->\n  link1|value1\n");
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_state_to_file_string_sorted() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path)).unwrap();
        data.state = vec![
            (
                "parent1".to_string(),
                vec![("link1".to_string(), "value1".to_string())],
            ),
            (
                "apple".to_string(),
                vec![
                    ("homepage".to_string(), "www.apple.com".to_string()),
                    (
                        "dev".to_string(),
                        "https://developer.apple.com/".to_string(),
                    ),
                ],
            ),
        ];
        assert_eq!(
            data.state_to_file_string(),
            "apple->\n  dev|https://developer.apple.com/\n  homepage|www.apple.com\nparent1->\n  link1|value1\n"
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_save_to_file() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path)).unwrap();
        data.state = vec![(
            "parent1".to_string(),
            vec![("link1".to_string(), "value1".to_string())],
        )];
        data.save_to_file().expect("Could not save to file");
        let res = fs::read_to_string(&data.path).unwrap();
        assert_eq!(res, "parent1->\n  link1|value1\n");
        data.cleanup().expect("Could not clean up data store");
    }
}

// Test only
#[cfg(test)]
impl Data {
    fn cleanup(&mut self) -> Result<(), TapDataStoreError> {
        fs::remove_file(&self.path).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::FileDeleteFailed,
            message: format!("Could not delete data file: {}", e),
        })?;
        Ok(())
    }
}

struct Index {
    path: PathBuf,
    state: Vec<(String, usize, usize)>, // parent, offset, length
}
impl Index {
    fn new(path: PathBuf) -> Self {
        // IF file exists, read and update state, else create new
        // Steal new_test logic from below
        Self {
            path,
            state: vec![],
        }
    }

    fn state_to_file_string(&self) -> String {
        todo!("Impl state to file str for Data")
    }

    fn save_to_file(&self) {
        let str = self.state_to_file_string();
        // write str to file
        todo!("Impl save to file for Data")
    }
}

// Utils
/// Returns the parent directory of the current executable.
/// ## Errors
/// - `TapDataStoreErrorKind::ExecutablePathNotFound` - if unable to get current executable path
/// - `TapDataStoreErrorKind::ExecutablePathParentDirectoryNotFound` - if unable to get parent directory
fn get_parent_dir_of_tap() -> Result<PathBuf, TapDataStoreError> {
    let executable_path = std::env::current_exe().map_err(|e| TapDataStoreError {
        kind: TapDataStoreErrorKind::ExecutablePathNotFound,
        message: e.to_string(),
    })?;
    Ok(executable_path
        .parent()
        .ok_or(TapDataStoreError {
            kind: TapDataStoreErrorKind::ExecutablePathParentDirectoryNotFound,
            message: "".to_string(),
        })?
        .to_path_buf())
}

/// Check if the parent name is valid
/// ## Errors
/// - `TapDataStoreErrorKind::ReservedKeyword` - if parent uses a reserved keyword
fn validate_parent(parent: &str) -> Result<(), TapDataStoreError> {
    // Check rules for parent
    if vec![
        "-a",
        "--add",
        "-d",
        "--delete",
        "--export",
        "--help",
        "-i",
        "--init",
        "--import",
        "-s",
        "--show",
        "-u",
        "--update",
        "--upsert",
        "-v",
        "--version",
        "--parent-entity",
        "here",
        "|",
    ]
    .contains(&parent)
    {
        return Err(TapDataStoreError {
            kind: TapDataStoreErrorKind::ReservedKeyword,
            message: format!("Parent entity name {} is reserved", parent),
        });
    }
    Ok(())
}

/// Check if the link name is valid
/// ## Errors
/// - `TapDataStoreErrorKind::ReservedKeyword` - if link name uses a reserved keyword
fn validate_link(link: &str) -> Result<(), TapDataStoreError> {
    if link.contains("|") {
        return Err(TapDataStoreError {
            kind: TapDataStoreErrorKind::ReservedKeyword,
            message: format!(
                "Link name {} contains a vertical bar '|' which is reserved",
                link
            ),
        });
    }
    Ok(())
}

#[cfg(test)]
/// Returns a test file path for either an index or data file. A test file name is of the format:
/// - Index files: .tap_index_{test_name}_{timestamp}
/// - Data files: .tap_data_{test_name}_{timestamp_millis}
/// ## Errors
/// - `TapDataStoreErrorKind::CurrentTimeError` - if unable to get current system time
/// - Will panic if unable to get thread name
fn get_test_file_path(file_type: FileType) -> Result<PathBuf, TapDataStoreError> {
    let mut path_buf = get_parent_dir_of_tap()?;
    // NOTE: Workaround where tests running at same time instant were using same file
    // Instead, this will create a unique file for each test using test name
    let thread = std::thread::current();
    let test_name = thread.name().expect("Could not get thread name");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::CurrentTimeError,
            message: e.to_string(),
        })?
        .as_millis();

    path_buf = match file_type {
        FileType::Data => path_buf.join(format!(".tap_data_{}_{}", test_name, timestamp)),
        FileType::Index => path_buf.join(format!(".tap_index_{}_{:?}", test_name, timestamp)),
    };
    Ok(path_buf)
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_validate_parent_success() {
        assert!(validate_parent("test").is_ok());
        assert!(validate_parent("search-engines").is_ok());
        assert!(validate_parent("valid-parent-name").is_ok());
        assert!(validate_parent("here-is-where-we-see-if-the-parent-is-valid").is_ok());
        assert!(validate_parent("well-what-do-we-have-here").is_ok());
        assert!(validate_parent("Sure, spaces also are valid!").is_ok());
        assert!(validate_parent("parent-entity").is_ok());
    }

    #[test]
    fn test_validate_parent_failure() {
        assert!(validate_parent("-a").is_err());
        assert!(validate_parent("--add").is_err());
        assert!(validate_parent("-d").is_err());
        assert!(validate_parent("--delete").is_err());
        assert!(validate_parent("--export").is_err());
        assert!(validate_parent("--help").is_err());
        assert!(validate_parent("-i").is_err());
        assert!(validate_parent("--init").is_err());
        assert!(validate_parent("--import").is_err());
        assert!(validate_parent("-s").is_err());
        assert!(validate_parent("--show").is_err());
        assert!(validate_parent("-u").is_err());
        assert!(validate_parent("--update").is_err());
        assert!(validate_parent("--upsert").is_err());
        assert!(validate_parent("-v").is_err());
        assert!(validate_parent("--version").is_err());
        assert!(validate_parent("--parent-entity").is_err());
        assert_eq!(
            validate_parent("here").unwrap_err().kind,
            TapDataStoreErrorKind::ReservedKeyword
        );
    }

    #[test]
    fn test_validate_link_success() {
        assert!(validate_link("test").is_ok());
        assert!(validate_link("search-engines").is_ok());
        assert!(validate_link("valid-link-name").is_ok());
        assert!(validate_link("here-is-where-we-see-if-the-link-is-valid").is_ok());
        assert!(validate_link("well-what-do-we-have-here").is_ok());
        assert!(validate_link("Sure, spaces also are valid!").is_ok());
    }

    #[test]
    fn test_validate_link_failure() {
        assert!(validate_link("|").is_err());
        assert!(validate_link("search|engines").is_err());
        assert_eq!(
            validate_link("search|engines").unwrap_err().kind,
            TapDataStoreErrorKind::ReservedKeyword
        );
    }
}

// What if datastore and indexstore are structs instead stored in a DataStore struct
// Then add methods to datastore and index store for internal workings
// The public Datastore only exposes 6 methods: new, new_test(test), cleanup, add_link, upsert_link, delete_link, read(parent, optional_link)

// TODO: add tests

// test parse index empty
// test parse index valid 1 parent,offset,length
// test parse index valid 2 parents,offsets,lengths
// test parse index invalid parent, no offsets, lengths (proper parse error)
// test parse index invalid parent, offsets, no lengths (proper parse error)
// test parse index invalid parent, offsets, no lengths (proper parse error)
// test parse index invalid random file with strings (proper parse error)

// test new Index (provided path, no previous exist)
// test new Index (provided path, previous exist)
// test new Index (no path, should create .tap_index)

// test add_link
// test upsert_link
// test delete_link
// test read

// Errors
#[derive(Debug, PartialEq)]
pub enum TapDataStoreErrorKind {
    CurrentTimeError,
    ExecutablePathNotFound,
    ExecutablePathParentDirectoryNotFound,
    FileCreateFailed,
    FileDeleteFailed,
    FileReadFailed,
    FileWriteFailed,
    LinkAlreadyExists,
    ParseError,
    ReservedKeyword,
}

#[derive(Debug)]
pub struct TapDataStoreError {
    kind: TapDataStoreErrorKind,
    message: String,
}

impl fmt::Display for TapDataStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (tap data store error: {})", self.message, self.kind)
    }
}

impl fmt::Display for TapDataStoreErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TapDataStoreErrorKind::CurrentTimeError => write!(f, "Current time error"),
            TapDataStoreErrorKind::ExecutablePathNotFound => {
                write!(f, "Executable path not found")
            }
            TapDataStoreErrorKind::ExecutablePathParentDirectoryNotFound => {
                write!(f, "Executable path parent directory not found")
            }
            TapDataStoreErrorKind::FileCreateFailed => write!(f, "File create failed"),
            TapDataStoreErrorKind::FileDeleteFailed => write!(f, "File delete failed"),
            TapDataStoreErrorKind::FileReadFailed => write!(f, "File read failed"),
            TapDataStoreErrorKind::FileWriteFailed => write!(f, "File write failed"),
            TapDataStoreErrorKind::LinkAlreadyExists => write!(f, "Link already exists"),
            TapDataStoreErrorKind::ParseError => write!(f, "Parse error"),
            TapDataStoreErrorKind::ReservedKeyword => write!(f, "Reserved keyword used"),
        }
    }
}
