use std::{fmt, fs, fs::File, io::Read, path::PathBuf};

struct DataStore {
    data_store: Option<PathBuf>,
    index_store: Option<PathBuf>,
}

impl DataStore {
    /// Creates the `tap_data` and `tap_index` data store files if they don't exist in the executable's parent directory.
    ///
    /// - When one of the files already exists, the path to the existing file is returned.
    /// - When a file doesn't exist, it is created.
    /// - If the file(s) can't be created, an error is returned with the message of the error.
    pub fn new() -> Result<Self, TapDataStoreError> {
        let executable_parent_dir = get_parent_dir_of_tap()?;
        let tap_data_path = executable_parent_dir.join(".tap_data");
        let tap_index_path = executable_parent_dir.join(".tap_index");
        for (path, kind) in [
            (
                &tap_data_path,
                TapDataStoreErrorKind::DataFileCreationFailed,
            ),
            (
                &tap_index_path,
                TapDataStoreErrorKind::IndexFileCreationFailed,
            ),
        ] {
            if !path.exists() {
                File::create_new(path).map_err(|e| TapDataStoreError {
                    kind,
                    message: e.to_string(),
                })?;
            }
        }

        Ok(DataStore {
            data_store: Some(tap_data_path),
            index_store: Some(tap_index_path),
        })
    }

    #[cfg(test)]
    /// Creates the `tap_data` and `tap_index` data store files in the current directory with
    /// unique names. This is used in the testing environment to ensure that the files created
    /// are unique to a single test run and can be deleted after the test is finished.
    fn new_test() -> Result<Self, TapDataStoreError> {
        let executable_parent_dir = get_parent_dir_of_tap()?;

        // NOTE: Workaround where tests running at same time instant were using same file
        // Instead, this will create a unique file for each test using test name
        let thread = std::thread::current();
        let test_name = thread.name().expect("Could not get thread name");

        let tap_data_path = executable_parent_dir.join(format!(".tap_data_{}", test_name));
        let tap_index_path = executable_parent_dir.join(format!(".tap_index_{}", test_name));

        for (path, kind) in [
            (
                &tap_data_path,
                TapDataStoreErrorKind::DataFileCreationFailed,
            ),
            (
                &tap_index_path,
                TapDataStoreErrorKind::IndexFileCreationFailed,
            ),
        ] {
            File::create_new(path).map_err(|e| TapDataStoreError {
                kind,
                message: e.to_string(),
            })?;
        }

        Ok(DataStore {
            data_store: Some(tap_data_path),
            index_store: Some(tap_index_path),
        })
    }

    /// Deletes the `tap_data` and `tap_index` data store files. As a side effect, the `data_store`
    /// and `index_store` are set to `None`
    fn cleanup(&mut self) -> Result<(), TapDataStoreError> {
        for (path, kind) in [
            (
                &self.data_store,
                TapDataStoreErrorKind::DataFileDeletionFailed,
            ),
            (
                &self.index_store,
                TapDataStoreErrorKind::IndexFileDeletionFailed,
            ),
        ] {
            if let Some(path) = path {
                if path.exists() {
                    fs::remove_file(path).map_err(|e| TapDataStoreError {
                        kind,
                        message: e.to_string(),
                    })?;
                }
            }
        }
        self.data_store = None;
        self.index_store = None;
        Ok(())
    }

    /// Finds the index of the parent in the index store (if it exists).
    ///
    /// - Returns `None` if the parent is not found in the index store.
    /// - Returns `(parent, offset, length)` if the parent is found in the index store
    fn find_index(&self, parent: &str) -> Result<Option<(String, u32, u32)>, TapDataStoreError> {
        let mut res: Option<(String, u32, u32)> = None;
        let mut buffer = String::new();
        // Open index file
        if let Some(index_store) = &self.index_store {
            let mut index_file = File::open(index_store).map_err(|e| TapDataStoreError {
                kind: TapDataStoreErrorKind::IndexFileOpenFailed,
                message: e.to_string(),
            })?;
            index_file
                .read_to_string(&mut buffer)
                .map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::IndexFileReadFailed,
                    message: e.to_string(),
                })?;
            // Search for parent entity in file
            let search_pattern = format!("{parent}|");
            if let Some(index) = buffer.find(&search_pattern) {
                let sub_str = &buffer[index..];
                let elems: Vec<&str> = sub_str.splitn(3, '|').collect();
                res = Some((
                    elems[0].to_string(),
                    elems[1].parse::<u32>().map_err(|e| TapDataStoreError {
                        kind: TapDataStoreErrorKind::IndexFileParseFailed,
                        message: e.to_string(),
                    })?,
                    elems[2].parse::<u32>().map_err(|e| TapDataStoreError {
                        kind: TapDataStoreErrorKind::IndexFileParseFailed,
                        message: e.to_string(),
                    })?,
                ));
            }
        }
        Ok(res)
    }

    pub fn add_link(&self, parent: &str, link: &str, value: &str) -> Result<(), TapDataStoreError> {
        validate_parent_and_link(parent, link)?;
        // TODO: Add link
        // TODO: Update index
        todo!("Implement data store add link")
    }

    /// Reads one or all links from the parent in the data store (depending on if a link is specified).
    pub fn read(&self, parent: &str, link: Option<&str>) -> Result<String, TapDataStoreError> {
        todo!("Implement data store read")
    }

    /// Removes one or all links from the parent in the data store (depending on if a link is specified).
    pub fn remove(&self, parent: &str, link: Option<&str>) -> Result<(), TapDataStoreError> {
        todo!("Implement data store remove link")
    }

    /// Upsert a link in the data store
    pub fn upsert(&self, parent: &str, link: &str, value: &str) -> Result<(), TapDataStoreError> {
        todo!("Implement data store upsert")
    }

    pub fn data_store(&self) -> &Option<PathBuf> {
        &self.data_store
    }

    pub fn index_store(&self) -> &Option<PathBuf> {
        &self.index_store
    }
}

/// Returns the parent directory of the current executable
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

/// Checks if the parent and link names are valid and returns an error if they are not
/// If they are valid, returns `Ok(())`
fn validate_parent_and_link(parent: &str, link: &str) -> Result<(), TapDataStoreError> {
    // Check rules for parent and link
    let reserved_keywords = vec![
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
    ];
    if reserved_keywords.contains(&parent) {
        return Err(TapDataStoreError {
            kind: TapDataStoreErrorKind::ReservedKeyword,
            message: format!("Parent entity name {} is reserved", parent),
        });
    }
    if link.contains("|") {
        return Err(TapDataStoreError {
            kind: TapDataStoreErrorKind::VerticalBarInLinkName,
            message: format!(
                "Link name {} contains a vertical bar '|' which is reserved",
                link
            ),
        });
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum TapDataStoreErrorKind {
    CurrentTimeError,
    ExecutablePathNotFound,
    ExecutablePathParentDirectoryNotFound,
    DataFileCreationFailed,
    DataFileDeletionFailed,
    IndexFileCreationFailed,
    IndexFileDeletionFailed,
    IndexFileReadFailed,
    IndexFileOpenFailed,
    IndexFileParseFailed,
    ReservedKeyword,
    VerticalBarInLinkName,
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
            TapDataStoreErrorKind::DataFileCreationFailed => {
                write!(f, "Data file creation failed")
            }
            TapDataStoreErrorKind::DataFileDeletionFailed => {
                write!(f, "Data file deletion failed")
            }
            TapDataStoreErrorKind::IndexFileCreationFailed => {
                write!(f, "Index file creation failed")
            }
            TapDataStoreErrorKind::IndexFileDeletionFailed => {
                write!(f, "Index file deletion failed")
            }
            TapDataStoreErrorKind::IndexFileReadFailed => write!(f, "Index file read failed"),
            TapDataStoreErrorKind::IndexFileOpenFailed => write!(f, "Index file open failed"),
            TapDataStoreErrorKind::IndexFileParseFailed => {
                write!(f, "Index file parse failed")
            }
            TapDataStoreErrorKind::ReservedKeyword => write!(f, "Reserved keyword used"),
            TapDataStoreErrorKind::VerticalBarInLinkName => {
                write!(f, "Vertical bar '|' used in link name")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_store_init_create() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        let data_path = ds
            .data_store()
            .as_ref()
            .expect("Could not get data store path");
        let index_path = ds
            .index_store()
            .as_ref()
            .expect("Could not get index store path");
        assert!(&data_path.exists());
        assert!(&index_path.exists());
        assert!(&data_path.to_str().unwrap().contains(".tap_data"));
        assert!(&index_path.to_str().unwrap().contains(".tap_index"));

        ds.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_data_store_cleanup() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        println!("Data store path: {:?}", ds.data_store().clone().unwrap());
        println!("Index store path: {:?}", ds.index_store().clone().unwrap());
        let data_path = ds
            .data_store()
            .clone()
            .expect("Could not get data store path");
        let index_path = ds
            .index_store()
            .clone()
            .expect("Could not get index store path");
        println!("Data store path: {:?}", data_path.clone());
        println!("Index store path: {:?}", index_path.clone());
        assert!(&data_path.exists());
        assert!(&index_path.exists());

        ds.cleanup().expect("Could not clean up data store");
        assert!(!&data_path.exists());
        assert!(!&index_path.exists());
        assert!(ds.data_store().is_none());
        assert!(ds.index_store().is_none());
    }

    #[test]
    fn test_validate_parent_valid() {
        let parent = "valid-parent-name";
        let link = "valid-link-name";
        assert!(validate_parent_and_link(parent, link).is_ok());
    }

    #[test]
    fn test_validate_parent_invalid() {
        let parent = "--version";
        let link = "valid-link-name";
        let res = validate_parent_and_link(parent, link);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().kind,
            TapDataStoreErrorKind::ReservedKeyword
        );
    }

    #[test]
    fn test_validate_link_invalid() {
        let parent = "valid-parent-name";
        let link = "|invalid-link-name";
        let res = validate_parent_and_link(parent, link);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().kind,
            TapDataStoreErrorKind::VerticalBarInLinkName
        );
    }

    #[test]
    fn test_validate_link_valid() {
        let parent = "valid-parent-name";
        let link = "valid-link-name";
        assert!(validate_parent_and_link(parent, link).is_ok());
    }

    #[test]
    fn test_find_index_not_found() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        let res = ds.find_index("parent-not-in-index-store");
        assert!(res.is_ok());
        assert!(res.unwrap().is_none());
        ds.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_find_index_found() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        // Add parent to index
        fs::write(
            ds.index_store().as_ref().unwrap(),
            "parent-in-index-store|0|20",
        )
        .unwrap();

        let res = ds.find_index("parent-in-index-store");
        assert!(res.is_ok());
        let expected = Some(("parent-in-index-store".to_string(), 0, 20));
        assert_eq!(res.unwrap(), expected);
        ds.cleanup().expect("Could not clean up data store");
    }
}
