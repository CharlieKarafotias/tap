use std::{fmt, fs, fs::File, path::PathBuf};

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

/// Creates the `tap_data` and `tap_index` data store files if they don't exist in the executable's parent directory.
///
/// When one of the files already exists, the path to the existing file is returned.
/// When a file doesn't exist, this function creates them.
/// If the file(s) can't be created, an error is returned with the message of the error.
///
/// NOTE:
///   - When running tests, the files are created with the format `.tap_data_<timestamp>` and `.tap_index_<timestamp>`
///     and should be deleted after the test suite is finished.
///   - When running in production, the files are created in the executable's parent directory and persist.
pub(crate) fn data_store_init() -> Result<(Option<PathBuf>, Option<PathBuf>), TapDataStoreError> {
    let executable_parent_dir = get_parent_dir_of_tap()?;

    // Check if test env or production env
    let (tap_data_path, tap_index_path) = if cfg!(test) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map_err(|e| TapDataStoreError {
                kind: TapDataStoreErrorKind::CurrentTimeError,
                message: e.to_string(),
            })?;

        let tap_data_path =
            executable_parent_dir.join(format!(".tap_data_{}", timestamp.as_secs()));
        let tap_index_path =
            executable_parent_dir.join(format!(".tap_index_{}", timestamp.as_secs()));
        (tap_data_path, tap_index_path)
    } else {
        let tap_data_path = executable_parent_dir.join(".tap_data");
        let tap_index_path = executable_parent_dir.join(".tap_index");
        (tap_data_path, tap_index_path)
    };

    let tap_data_path_exists = tap_data_path.exists();
    let tap_index_path_exists = tap_index_path.exists();
    if !tap_data_path_exists {
        File::create(&tap_data_path).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::DataFileCreationFailed,
            message: e.to_string(),
        })?;
    }
    if !tap_index_path_exists {
        File::create(&tap_index_path).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::DataFileCreationFailed,
            message: e.to_string(),
        })?;
    }

    Ok((Some(tap_data_path), Some(tap_index_path)))
}

/// Deletes the `tap_data` and `tap_index` data store files if they exist in the executable's
/// parent directory.
///
/// NOTE:
///   - This does not handle cleanup of test files at this time
pub(crate) fn data_store_cleanup() -> Result<(), TapDataStoreError> {
    let executable_parent_dir = get_parent_dir_of_tap()?;

    let tap_data_path = executable_parent_dir.join(".tap_data");
    let tap_index_path = executable_parent_dir.join(".tap_index");

    if tap_data_path.exists() {
        fs::remove_file(&tap_data_path).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::DataFileDeletionFailed,
            message: e.to_string(),
        })?;
    }
    if tap_index_path.exists() {
        fs::remove_file(&tap_index_path).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::DataFileCreationFailed,
            message: e.to_string(),
        })?;
    }

    Ok(())
}

fn data_store_add() -> Result<(), TapDataStoreError> {
    data_store_init()?;
    todo!("Implement data store add");
}

fn data_store_remove() {
    todo!("Implement data store remove");
}

fn data_store_upsert() {
    todo!("Implement data store upsert");
}

fn data_store_get() {
    todo!("Implement data store get");
}

#[derive(Debug)]
pub enum TapDataStoreErrorKind {
    CurrentTimeError,
    ExecutablePathNotFound,
    ExecutablePathParentDirectoryNotFound,
    DataFileCreationFailed,
    DataFileDeletionFailed,
    IndexFileCreationFailed,
    IndexFileDeletionFailed,
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
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::fs;
//     use super::*;
//
//     fn cleanup_test_file(path: PathBuf) {
//         fs::remove_file(path).unwrap();
//     }
//
//     #[test]
//     fn test_data_store_init_create() {
//         let res = data_store_init().unwrap();
//         let tap_data_path = res.0.unwrap();
//         let tap_index_path = res.1.unwrap();
//
//         assert!(&tap_data_path.exists());
//         assert!(&tap_index_path.exists());
//         assert!(&tap_data_path.starts_with(".tap_data"));
//         assert!(&tap_index_path.starts_with(".tap_index"));
//
//         cleanup_test_file(tap_data_path);
//         cleanup_test_file(tap_index_path);
//     }
//
//     #[test]
//     fn test_data_store_cleanup() {
//         unimplemented!();
//     }
//
//     #[test]
//     fn test_data_store_add() {
//         unimplemented!();
//     }
//
//     #[test]
//     fn test_data_store_remove() {
//         unimplemented!();
//     }
//
//     #[test]
//     fn test_data_store_upsert() {
//         unimplemented!();
//     }
//
//     #[test]
//     fn test_data_store_get() {
//         unimplemented!();
//     }
// }
