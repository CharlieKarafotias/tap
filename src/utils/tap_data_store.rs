use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::{fmt, fs, fs::File, path::PathBuf};

enum FileType {
    Data,
    Index,
}

pub(crate) struct DataStore {
    data_store: Option<PathBuf>,
    index_store: Option<PathBuf>,
}

// TODO: refactor for these to be pub methods & create one for private methods
impl DataStore {
    /// Creates the `tap_data` and `tap_index` data store files if they don't exist in the executable's parent directory.
    ///
    /// - When one of the files already exists, the path to the existing file is returned.
    /// - When a file doesn't exist, it is created.
    /// - If the file(s) can't be created, an error is returned with the message of the error.
    pub fn new() -> Result<Self, TapDataStoreError> {
        #[cfg(test)]
        if cfg!(test) {
            return Self::new_test();
        }
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
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map_err(|e| TapDataStoreError {
                kind: TapDataStoreErrorKind::CurrentTimeError,
                message: e.to_string(),
            })?
            .as_millis();

        let tap_data_path =
            executable_parent_dir.join(format!(".tap_data_{}_{}", test_name, timestamp));
        let tap_index_path =
            executable_parent_dir.join(format!(".tap_index_{}_{:?}", test_name, timestamp));

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

    fn read_file_to_string(&self, file_type: FileType) -> Result<String, TapDataStoreError> {
        let path = match file_type {
            FileType::Data => &self.data_store,
            FileType::Index => &self.index_store,
        }
        .as_ref()
        .ok_or(TapDataStoreError {
            kind: TapDataStoreErrorKind::FilePathNotFound,
            message: "File path not defined in data store structure".to_string(),
        })?;

        let res = fs::read_to_string(path).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::FileReadFailed,
            message: e.to_string(),
        })?;
        Ok(res)
    }

    /// Finds the index of the parent in the index store (if it exists).
    ///
    /// - Returns `None` if the parent is not found in the index store.
    /// - Returns `(parent, offset, length)` if the parent is found in the index store
    fn find_index(
        &self,
        parent: &str,
    ) -> Result<Option<(String, usize, usize)>, TapDataStoreError> {
        let mut res: Option<(String, usize, usize)> = None;
        let buffer = self.read_file_to_string(FileType::Index)?;
        // Search for parent entity in file
        let search_pattern = format!("{parent}|");
        if let Some(index) = buffer.find(&search_pattern) {
            let sub_str = &buffer[index..];
            let mut elems: Vec<&str> = sub_str.splitn(3, '|').collect();

            // for last element in elems, trim trailing newline/EOF character
            elems[2] = elems[2]
                .split('\n')
                .next()
                .ok_or(TapDataStoreError {
                    kind: TapDataStoreErrorKind::IndexFileParseFailed,
                    message: "Could not parse the length of the parent entity".to_string(),
                })?
                .trim();

            res = Some((
                elems[0].to_string(),
                elems[1].parse::<usize>().map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::IndexFileParseFailed,
                    message: e.to_string(),
                })?,
                elems[2].parse::<usize>().map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::IndexFileParseFailed,
                    message: e.to_string(),
                })?,
            ));
        }
        Ok(res)
    }

    fn upsert_index(
        &self,
        parent: &str,
        offset: usize,
        length: usize,
    ) -> Result<(), TapDataStoreError> {
        let mut buffer = self.read_file_to_string(FileType::Index)?;
        let val_to_insert = format!("{parent}|{offset}|{length}\n");
        // If parent already exists in index file, update it & update offsets for following parents
        if let Some(index) = buffer.find(&format!("{parent}|")) {
            let sub_str = &buffer[index..];
            let new_line_index = sub_str.find('\n').ok_or(TapDataStoreError {
                kind: TapDataStoreErrorKind::IndexFileParseFailed,
                message: "Could not find the newline character for parent".to_string(),
            })?;
            // Also replace the newline character so +1 on new_line_index
            buffer.replace_range(index..=new_line_index, val_to_insert.as_str());
            // TODO: update offsets for parents after the parent
        } else {
            // append new parent to end of index file
            buffer.push_str(val_to_insert.as_str());
        }

        let index_path = self.index_store.as_ref().ok_or(TapDataStoreError {
            kind: TapDataStoreErrorKind::FilePathNotFound,
            message: "Could not open index file".to_string(),
        })?;
        fs::write(index_path, buffer).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::IndexFileWriteFailed,
            message: e.to_string(),
        })?;
        Ok(())
    }

    /// Returns all links of the parent in the data store. If buffer is specified, it will use the buffer instead of reading from the data store file
    // TODO: Use index caching instead of reading whole data file (when read is implemented)
    fn get_links_of_parent(
        &self,
        parent: &str,
        buf: Option<&str>,
    ) -> Result<Vec<String>, TapDataStoreError> {
        let data_file;
        let mut links = Vec::new();
        let mut buffer = if let Some(b) = buf {
            b
        } else {
            data_file = self.read_file_to_string(FileType::Data)?;
            if let Some((_, offset, length)) = self.find_index(parent)? {
                &data_file[offset..(offset + length)]
            } else {
                // No index for the parent so no links to return
                return Ok(links);
            }
        };
        buffer = buffer
            .strip_prefix(format!("{parent}->\n").as_str())
            .ok_or(TapDataStoreError {
                kind: TapDataStoreErrorKind::DataFileImproperFormat,
                message: "Could not parse the parent from the data file".to_string(),
            })?;

        for line in buffer.lines() {
            // If in format of another parent, then break
            if line.ends_with("->") {
                break;
            }
            let (link, _) = line.split_once('|').ok_or(TapDataStoreError {
                kind: TapDataStoreErrorKind::DataFileImproperFormat,
                message: "Could not parse the link from the data file".to_string(),
            })?;
            links.push(link.trim().to_string());
        }
        Ok(links)
    }

    pub fn add_link(&self, parent: &str, link: &str, value: &str) -> Result<(), TapDataStoreError> {
        validate_parent_and_link(parent, link)?;
        let parent_idx = self.find_index(parent)?;
        let data_path = self.data_store().as_ref().ok_or(TapDataStoreError {
            kind: TapDataStoreErrorKind::FilePathNotFound,
            message: "Could not open data file".to_string(),
        })?;

        if let Some((_, offset, length)) = parent_idx {
            let mut buf = self.read_file_to_string(FileType::Data)?;
            let existing_links = self.get_links_of_parent(parent, Some(&buf))?;

            // TODO: Could this also be upsert with additional param. If this is called upsert, and new param called update if exists bool
            if existing_links.contains(&link.to_string()) {
                return Err(TapDataStoreError {
                    kind: TapDataStoreErrorKind::LinkAlreadyExists,
                    message: format!("Link {link} already exists for parent {parent}"),
                });
            }

            // append link/value
            // TODO: store updated values for index
            let mut parent_entity_data = buf[offset..(offset + length)].to_string();
            parent_entity_data.push_str(&format!("  {link}|{value}\n"));
            buf.replace_range(offset..(offset + length), parent_entity_data.as_str());

            fs::write(data_path, buf).map_err(|e| TapDataStoreError {
                kind: TapDataStoreErrorKind::FileWriteFailed,
                message: e.to_string(),
            })?;

            // TODO: upsert_index will need to handle updating other index entries
            self.upsert_index(parent, offset, parent_entity_data.len())?;
        } else {
            // No existing links, add link and update index
            let str = format!("{parent}->\n  {link}|{value}\n");

            let mut f = OpenOptions::new()
                .append(true)
                .open(data_path)
                .map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::FileOpenFailed,
                    message: e.to_string(),
                })?;
            let length_of_file_before_write =
                f.seek(SeekFrom::End(0)).map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::FileReadFailed,
                    message: e.to_string(),
                })?;
            f.write_all(str.as_bytes()).map_err(|e| TapDataStoreError {
                kind: TapDataStoreErrorKind::FileWriteFailed,
                message: e.to_string(),
            })?;
            // Add parent to index
            self.upsert_index(parent, length_of_file_before_write as usize, str.len())?;
        }

        Ok(())
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
    DataFileImproperFormat,
    FileReadFailed,
    FilePathNotFound,
    FileWriteFailed,
    FileOpenFailed,
    IndexFileCreationFailed,
    IndexFileDeletionFailed,
    IndexFileParseFailed,
    IndexFileWriteFailed,
    LinkAlreadyExists,
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
            TapDataStoreErrorKind::DataFileImproperFormat => {
                write!(f, "Data file has an improper format")
            }
            TapDataStoreErrorKind::FileReadFailed => write!(f, "File read failed"),
            TapDataStoreErrorKind::FilePathNotFound => write!(f, "File path not found"),
            TapDataStoreErrorKind::FileOpenFailed => write!(f, "File open failed"),
            TapDataStoreErrorKind::FileWriteFailed => write!(f, "File write failed"),
            TapDataStoreErrorKind::IndexFileCreationFailed => {
                write!(f, "Index file creation failed")
            }
            TapDataStoreErrorKind::IndexFileDeletionFailed => {
                write!(f, "Index file deletion failed")
            }
            TapDataStoreErrorKind::IndexFileParseFailed => {
                write!(f, "Index file parse failed")
            }
            TapDataStoreErrorKind::IndexFileWriteFailed => {
                write!(f, "Index file write failed")
            }
            TapDataStoreErrorKind::LinkAlreadyExists => {
                write!(f, "Link already exists")
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

    #[test]
    fn test_find_index_middle() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        // Add parent to index
        fs::write(
            ds.index_store().as_ref().unwrap(),
            "parent-1|0|20\nparent-2|20|40\nparent-3|40|60",
        )
        .unwrap();

        let res = ds.find_index("parent-2");
        println!("res: {res:?}");
        assert!(res.is_ok());
        let expected = Some(("parent-2".to_string(), 20, 40));
        assert_eq!(res.unwrap(), expected);
        ds.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_find_index_end() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        // Add parent to index
        fs::write(
            ds.index_store().as_ref().unwrap(),
            "parent-1|0|20\nparent-2|20|40\nparent-3|40|60",
        )
        .unwrap();

        let res = ds.find_index("parent-3");
        assert!(res.is_ok());
        let expected = Some(("parent-3".to_string(), 40, 60));
        assert_eq!(res.unwrap(), expected);
        ds.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_upsert_index_no_previous_index() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        let res = ds.upsert_index("parent", 0, 20);
        assert!(res.is_ok());

        let res_2 = ds.find_index("parent").expect("Could not find index");
        let expected = Some(("parent".to_string(), 0, 20));
        assert_eq!(res_2, expected);
        ds.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_upsert_index_with_previous_index() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        // Add parent to index
        fs::write(ds.index_store().as_ref().unwrap(), "parent|0|20\n").unwrap();
        let _ = ds
            .upsert_index("parent", 0, 50)
            .expect("Could not update index");

        // TODO: determine why find_index does not work
        let res_2 = ds.find_index("parent").expect("Could not find index");
        let expected = Some(("parent".to_string(), 0, 50));
        assert_eq!(res_2, expected);
        ds.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_get_links_of_parent_with_valid_buffer() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        let res = ds.get_links_of_parent("some-parent", Some("some-parent->\n  link1|value1\n  link2|value2\nother-parent->\n  link3|value3\n")).expect("Could not get links of parent");
        assert_eq!(res, vec!["link1".to_string(), "link2".to_string()]);
        ds.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_get_links_of_parent_with_valid_data_file() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        ds.add_link("another-parent", "my-link", "my-value")
            .expect("Could not add link");
        let res = ds
            .get_links_of_parent("another-parent", None)
            .expect("Could not get links of parent");
        assert_eq!(res, vec!["my-link".to_string()]);
        ds.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_get_links_of_parent_empty_vec_when_no_parent() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        let res = ds
            .get_links_of_parent("some-parent", None)
            .expect("Could not get links of parent");
        assert!(res.is_empty());
        ds.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_get_links_of_parent_with_invalid_buffer() {
        let mut ds = DataStore::new_test().expect("Could not create data store");
        let res = ds.get_links_of_parent(
            "some-parent",
            Some("some-parent-\n  link1|value1\n  link2|value2\nother-parent->\n  link3|value3\n"),
        );
        assert_eq!(
            res.unwrap_err().kind,
            TapDataStoreErrorKind::DataFileImproperFormat
        );
        ds.cleanup().expect("Could not clean up data store");
    }
}
