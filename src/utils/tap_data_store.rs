use std::{fmt, fs, fs::File, path::PathBuf, time::SystemTime};

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
                File::create(path).map_err(|e| TapDataStoreError {
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
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| TapDataStoreError {
                kind: TapDataStoreErrorKind::CurrentTimeError,
                message: e.to_string(),
            })?;
        let tap_data_path =
            executable_parent_dir.join(format!(".tap_data_{}", timestamp.as_secs()));
        let tap_index_path =
            executable_parent_dir.join(format!(".tap_index_{}", timestamp.as_secs()));

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
                File::create(path).map_err(|e| TapDataStoreError {
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

    pub fn add_link(&self, parent: &str, link: &str, value: &str) -> Result<(), TapDataStoreError> {
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
        let data_path = ds
            .data_store()
            .clone()
            .expect("Could not get data store path");
        let index_path = ds
            .index_store()
            .clone()
            .expect("Could not get index store path");
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
}
