use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::{fmt, fs, fs::File, path::PathBuf};

type LinkValue = (String, String);
type IndexEntry = (String, usize);

type IndexOffsetLength = (usize, usize);

/// A struct containing the data file and index file. The ReadDataStore struct utilizes the
/// index file to speed up reads via Seeks. The DataStore struct does not utilize the index file.
/// Note: The ReadDataStore struct is intended to be used in a read-only context and therefore
/// does not expose the ability to add or delete data.  
pub(crate) struct ReadDataStore {
    data: Data,
    _index: Index,
}

impl ReadDataStore {
    pub fn new(path: Option<PathBuf>, parent: String) -> Result<Self, TapDataStoreError> {
        let index = Index::new(path.clone())?;
        let index_offset_length = index.find_parent_offset_and_length(parent)?;
        let data = Data::new(path, Some(index_offset_length))?;
        Ok(Self {
            data,
            _index: index,
        })
    }

    pub fn read_link(&self, parent: &str, link: &str) -> Result<LinkValue, TapDataStoreError> {
        let links = self.data.get(parent, Some(link))?;
        // This is ok as data.get will return an error if the link is not found
        Ok(links[0].clone())
    }

    pub fn read_parent(&self, parent: &str) -> Result<Vec<LinkValue>, TapDataStoreError> {
        self.data.get(parent, None)
    }

    pub fn links(&self, parent: &str) -> Result<Vec<String>, TapDataStoreError> {
        let links = self.read_parent(parent)?;
        Ok(links.iter().map(|(l, _)| l.clone()).collect())
    }
}

pub(crate) struct DataStore {
    data: Data,
    index: Index,
}

impl DataStore {
    pub fn new(path: Option<PathBuf>) -> Result<Self, TapDataStoreError> {
        let data = Data::new(path.clone(), None)?;
        let index = Index::new(path)?;
        Ok(Self { data, index })
    }

    pub fn add_link(
        &mut self,
        parent: String,
        link: String,
        value: String,
    ) -> Result<(), TapDataStoreError> {
        self.data.add_link(&parent, &link, &value)?;
        let index_offsets = self.data.save_to_file()?;
        self.index.update(index_offsets);
        self.index.save_to_file()?;
        Ok(())
    }

    pub fn delete(
        &mut self,
        parent: String,
        link: Option<String>,
    ) -> Result<(), TapDataStoreError> {
        self.data.remove(&parent, link.as_deref())?;
        let index_offsets = self.data.save_to_file()?;
        self.index.update(index_offsets);
        self.index.save_to_file()?;
        Ok(())
    }

    /// Returns the first link in the list. This does not utilize the index and is therefore slow.
    /// To use the indexed version, create a new `ReadDataStore` struct and call `read_link` or `read_parent`.
    #[allow(dead_code)]
    pub fn read_link_slow(
        &self,
        parent: &str,
        link: &str,
    ) -> Result<Option<LinkValue>, TapDataStoreError> {
        let links = self.data.get(parent, Some(link))?;
        Ok(links.first().cloned())
    }

    /// Returns the first link in the list. This does not utilize the index and is therefore slow.
    /// To use the indexed version, create a new `ReadDataStore` struct and call `read_link` or `read_parent`.
    #[allow(dead_code)]
    pub fn read_parent_slow(&self, parent: &str) -> Result<Vec<LinkValue>, TapDataStoreError> {
        self.data.get(parent, None)
    }

    pub fn upsert_link(
        &mut self,
        parent: String,
        link: String,
        value: String,
    ) -> Result<(), TapDataStoreError> {
        self.data.upsert_link(&parent, &link, &value)?;
        let index_offsets = self.data.save_to_file()?;
        self.index.update(index_offsets);
        self.index.save_to_file()?;
        Ok(())
    }

    pub fn import(
        &mut self,
        path: PathBuf,
        import_type: ImportType,
    ) -> Result<(), TapDataStoreError> {
        self.data.import(import_type, path.clone())?;
        let index_offsets = self.data.save_to_file()?;
        self.index.update(index_offsets);
        self.index.save_to_file()?;
        Ok(())
    }
}

pub(super) struct Data {
    path: PathBuf,
    state: Vec<(String, Vec<LinkValue>)>,
}

// Publicly exposed
impl Data {
    pub fn new(
        path: Option<PathBuf>,
        index_offset_length: Option<IndexOffsetLength>,
    ) -> Result<Self, TapDataStoreError> {
        // Determine path
        let (file_exists, path) = if let Some(path) = path {
            (path.exists(), path)
        } else {
            let mut tap_data_path = get_parent_dir_of_tap()?;

            // NOTE: workaround for command tests running at the same time.
            // Use test pathing for tests, otherwise use standard
            // I want to use this over cfg!(test) as I do not want to compile test code in prod builds
            #[allow(unused_mut)]
            let mut test_path: Option<PathBuf> = None;
            #[cfg(test)]
            {
                test_path = Some(get_test_file_path(FileType::Data)?);
            }
            if let Some(test_path) = test_path {
                tap_data_path = test_path;
            } else {
                tap_data_path = tap_data_path.join(".tap_data");
            }
            (tap_data_path.exists(), tap_data_path)
        };

        // Parse file if it exists
        if file_exists {
            #[allow(unused_assignments)]
            let mut file_as_str: String = String::new();

            // If index_offset_length is set, then we are reading from the index file
            if let Some((offset, length)) = index_offset_length {
                let mut f = File::open(&path).map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::FileOpenFailed,
                    message: format!("Could not open data file at {}: {e}", path.display()),
                })?;
                f.seek(SeekFrom::Start(offset as u64))
                    .map_err(|e| TapDataStoreError {
                        kind: TapDataStoreErrorKind::FileSeekFailed,
                        message: format!(
                            "Could not seek to offset {offset} in data file at {}: {e}",
                            path.display()
                        ),
                    })?;
                let mut buf = if length == 0 {
                    // If length is 0, then we are reading the rest of the file
                    let metadata = f.metadata().map_err(|e| TapDataStoreError {
                        kind: TapDataStoreErrorKind::FileReadMetadataFailed,
                        message: format!(
                            "Could not read metadata for data file at {}: {e}",
                            path.display()
                        ),
                    })?;
                    let length = metadata.len() - offset as u64;
                    vec![0u8; length as usize]
                } else {
                    vec![0u8; length]
                };
                f.read_exact(&mut buf).map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::FileReadFailed,
                    message: format!("Could not read data file at {}: {e}", path.display()),
                })?;
                file_as_str = buf.iter().map(|b| *b as char).collect();
            } else {
                file_as_str = fs::read_to_string(&path).map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::FileReadFailed,
                    message: format!("Could not read data file at {}: {e}", path.display()),
                })?;
            }
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
    }

    pub fn add_link(
        &mut self,
        parent: &str,
        link: &str,
        value: &str,
    ) -> Result<(), TapDataStoreError> {
        validate_parent(parent)?;
        validate_link(link)?;
        if let Some((_, links)) = self.state.iter_mut().find(|(p, _)| p == parent) {
            if links.iter().any(|(l, _)| l.trim() == link) {
                return Err(TapDataStoreError {
                    kind: TapDataStoreErrorKind::LinkAlreadyExists,
                    message: format!("Link {link} already exists for parent {parent}"),
                });
            }
            links.push((link.trim().to_string(), value.trim().to_string()));
        } else {
            self.state.push((
                parent.to_string(),
                vec![(link.trim().to_string(), value.trim().to_string())],
            ));
        }
        Ok(())
    }

    pub fn get(
        &self,
        parent: &str,
        link: Option<&str>,
    ) -> Result<Vec<LinkValue>, TapDataStoreError> {
        validate_parent(parent)?;
        let links = self
            .state
            .iter()
            .find(|(p, _)| p.trim() == parent)
            .map(|(_, links)| links.clone())
            .ok_or(TapDataStoreError {
                kind: TapDataStoreErrorKind::ParentEntityNotFound,
                message: format!("Parent '{parent}' not found"),
            })?;
        if let Some(link) = link {
            validate_link(link)?;
            let link = link.trim();
            let found_link = links.iter().find(|(l, _)| l.trim() == link);
            if let Some(found_link) = found_link {
                return Ok(vec![found_link.clone()]);
            } else {
                return Err(TapDataStoreError {
                    kind: TapDataStoreErrorKind::LinkNotFound,
                    message: format!("Link '{link}' not found in parent '{parent}'"),
                });
            }
        }
        Ok(links)
    }

    pub fn remove(&mut self, parent: &str, link: Option<&str>) -> Result<(), TapDataStoreError> {
        validate_parent(parent)?;
        if let Some(link) = link {
            validate_link(link)?;
        }
        if let Some(parent_idx) = self.state.iter().position(|(p, _)| p == parent) {
            let (_, links) = &mut self.state[parent_idx];
            // If there is a link to remove, find and remove. Otherwise, remove parent
            if let Some(link) = link {
                if let Some(index) = links.iter().position(|(l, _)| l.trim() == link) {
                    links.remove(index);
                    // If no links left, remove parent as well
                    if links.is_empty() {
                        self.state.remove(parent_idx);
                    }
                } else {
                    return Err(TapDataStoreError {
                        kind: TapDataStoreErrorKind::LinkNotFound,
                        message: format!("Link '{link}' not found in parent '{parent}'"),
                    });
                }
            } else {
                self.state.remove(parent_idx);
            }
        } else {
            return Err(TapDataStoreError {
                kind: TapDataStoreErrorKind::ParentEntityNotFound,
                message: format!("Parent '{parent}' not found"),
            });
        }
        Ok(())
    }

    pub fn upsert_link(
        &mut self,
        parent: &str,
        link: &str,
        value: &str,
    ) -> Result<(), TapDataStoreError> {
        validate_parent(parent)?;
        validate_link(link)?;
        if let Some((_, links)) = self.state.iter_mut().find(|(p, _)| p == parent) {
            // If link already exists, update, else add
            if let Some((_, v)) = links.iter_mut().find(|(l, _)| l.trim() == link) {
                *v = value.trim().to_string();
            } else {
                links.push((link.trim().to_string(), value.trim().to_string()));
            }
        } else {
            // If parent does not exist, add parent and new link/value pair
            self.state.push((
                parent.to_string(),
                vec![(link.trim().to_string(), value.trim().to_string())],
            ));
        }
        Ok(())
    }

    pub fn import(
        &mut self,
        file_type: ImportType,
        path: PathBuf,
    ) -> Result<(), TapDataStoreError> {
        validate_path(&file_type, &path)?;
        let file_exists = path.try_exists().map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::FileOpenFailed,
            message: format!("Unable to determine if file {} exists: {e}", path.display()),
        })?;
        if !file_exists {
            return Err(TapDataStoreError {
                kind: TapDataStoreErrorKind::FileOpenFailed,
                message: format!("File {} does not exist", path.display()),
            });
        }
        match file_type {
            ImportType::Tap => {
                let file_as_str = fs::read_to_string(&path).map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::FileReadFailed,
                    message: format!("Could not read data file at {}: {e}", path.display()),
                })?;
                let state = Data::parse_file(&file_as_str)?;
                // TODO: refactor to hashmap? This would speed up import
                state.iter().for_each(|(parent, links)| {
                    links.iter().for_each(|(link, value)| {
                        self.upsert_link(parent, link, value).unwrap();
                    })
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod data_public {
    use super::{Data, FileType, ImportType, TapDataStoreErrorKind, get_test_file_path};
    use std::fs;
    use std::path::PathBuf;

    // TODO: GH-45 Move the following to an integration test
    #[test]
    #[ignore = "GH-45: Should really be an integration test - move this out"]
    fn test_new_no_path_correct_file_name() {
        let expected_file_name = ".tap_data";
        let mut data = Data::new(None, None).unwrap();
        assert!(data.path.to_str().unwrap().ends_with(expected_file_name));
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_new_with_path_correct_file_name() {
        let expected_file_name =
            get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(PathBuf::from(&expected_file_name)), None).unwrap();
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
        let mut data = Data::new(Some(data_path), None).unwrap();
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

    #[test]
    fn test_set_state_correct_reader_eof() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        fs::write(
            &data_path,
            "parent1->\nlink1|value1\nlink2|value2\nparent2 ->\nlink3|value3\nlink4|value4",
        )
        .unwrap();
        let mut data = Data::new(Some(data_path), Some((36, 0))).unwrap();
        assert_eq!(
            data.state,
            vec![(
                "parent2 ".to_string(),
                vec![
                    ("link3".to_string(), "value3".to_string()),
                    ("link4".to_string(), "value4".to_string())
                ]
            ),]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_set_state_correct_reader() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        fs::write(
            &data_path,
            "parent1->\nlink1|value1\nlink2|value2\nparent2 ->\nlink3|value3\nlink4|value4",
        )
        .unwrap();
        let mut data = Data::new(Some(data_path), Some((0, 35))).unwrap();
        assert_eq!(
            data.state,
            vec![(
                "parent1".to_string(),
                vec![
                    ("link1".to_string(), "value1".to_string()),
                    ("link2".to_string(), "value2".to_string())
                ]
            ),]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_add_link_when_parent_doesnt_exist() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        let res = data.add_link("parent1", "link1", "value1");
        assert!(res.is_ok());
        assert_eq!(
            data.state,
            vec![(
                "parent1".to_string(),
                vec![("link1".to_string(), "value1".to_string())]
            )]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_add_link_when_link_doesnt_exist() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![
                ("google".to_string(), "www.google.com".to_string()),
                ("yahoo".to_string(), "www.yahoo.com".to_string()),
            ],
        )];
        let res = data.add_link("search-engines", "link1", "value1");
        assert!(res.is_ok());
        assert_eq!(
            data.state,
            vec![(
                "search-engines".to_string(),
                vec![
                    ("google".to_string(), "www.google.com".to_string()),
                    ("yahoo".to_string(), "www.yahoo.com".to_string()),
                    ("link1".to_string(), "value1".to_string())
                ]
            )]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_add_link_when_link_already_exists() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![("google".to_string(), "www.google.com".to_string())],
        )];
        let res = data.add_link("search-engines", "google", "something else");
        assert_eq!(
            res.unwrap_err().kind,
            TapDataStoreErrorKind::LinkAlreadyExists
        );
        assert_eq!(
            data.state,
            vec![(
                "search-engines".to_string(),
                vec![("google".to_string(), "www.google.com".to_string()),]
            )]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_get_parent_when_parent_exists() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![
                ("google".to_string(), "www.google.com".to_string()),
                ("yahoo".to_string(), "www.yahoo.com".to_string()),
            ],
        )];
        let res = data.get("search-engines", None);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            vec![
                ("google".to_string(), "www.google.com".to_string()),
                ("yahoo".to_string(), "www.yahoo.com".to_string()),
            ]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_get_parent_when_parent_does_not_exist() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        let res = data.get("search-engines", None);
        assert_eq!(
            res.unwrap_err().kind,
            TapDataStoreErrorKind::ParentEntityNotFound
        );
        assert_eq!(data.state, vec![]);
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_get_parent_and_link_when_parent_exists_and_link_exists() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![
                ("google".to_string(), "www.google.com".to_string()),
                ("yahoo".to_string(), "www.yahoo.com".to_string()),
            ],
        )];
        let res = data.get("search-engines", Some("google"));
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            vec![("google".to_string(), "www.google.com".to_string())]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_get_parent_and_link_when_parent_exists_and_link_does_not_exist() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![
                ("google".to_string(), "www.google.com".to_string()),
                ("yahoo".to_string(), "www.yahoo.com".to_string()),
            ],
        )];
        let res = data.get("search-engines", Some("link1"));
        assert_eq!(res.unwrap_err().kind, TapDataStoreErrorKind::LinkNotFound);
        assert_eq!(
            data.state,
            vec![(
                "search-engines".to_string(),
                vec![
                    ("google".to_string(), "www.google.com".to_string()),
                    ("yahoo".to_string(), "www.yahoo.com".to_string()),
                ],
            )]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_import_tap_bad_file() {
        let import_path = "rando_file.pdf";
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        let res = data.import(ImportType::Tap, PathBuf::from(import_path));
        assert_eq!(
            res.unwrap_err().kind,
            TapDataStoreErrorKind::InvalidFileExtension
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_import_tap_data_parent_exists_but_link_does_not() {
        let import_path = get_test_file_path(FileType::Tap).expect("Could not get test file path");
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![("google".to_string(), "www.google.com".to_string())],
        )];
        fs::write(&import_path, "search-engines->\nyahoo|www.yahoo.com\n").unwrap();

        let res = data.import(ImportType::Tap, import_path.clone());
        assert!(res.is_ok());
        assert_eq!(
            data.state,
            vec![(
                "search-engines".to_string(),
                vec![
                    ("google".to_string(), "www.google.com".to_string()),
                    ("yahoo".to_string(), "www.yahoo.com".to_string())
                ]
            ),]
        );
        data.cleanup().expect("Could not clean up data store");
        fs::remove_file(import_path).expect("Could not remove import file");
    }

    #[test]
    fn test_import_tap_data_parent_does_not_exist() {
        let import_path = get_test_file_path(FileType::Tap).expect("Could not get test file path");
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![("google".to_string(), "www.google.com".to_string())],
        )];
        fs::write(&import_path, "repo->\ngh|www.github.com\n").unwrap();

        let res = data.import(ImportType::Tap, import_path.clone());
        assert!(res.is_ok());
        assert_eq!(
            data.state,
            vec![
                (
                    "search-engines".to_string(),
                    vec![("google".to_string(), "www.google.com".to_string()),]
                ),
                (
                    "repo".to_string(),
                    vec![("gh".to_string(), "www.github.com".to_string()),]
                ),
            ]
        );
        data.cleanup().expect("Could not clean up data store");
        fs::remove_file(import_path).expect("Could not remove import file");
    }

    #[test]
    fn test_import_tap_data_parent_exists_and_link_exists() {
        let import_path = get_test_file_path(FileType::Tap).expect("Could not get test file path");
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![("google".to_string(), "www.google.com".to_string())],
        )];
        fs::write(&import_path, "search-engines->\ngoogle|abc\n").unwrap();

        let res = data.import(ImportType::Tap, import_path.clone());
        // TODO: currently the link is overwritten, make this a param instead?
        assert!(res.is_ok());
        assert_eq!(
            data.state,
            vec![(
                "search-engines".to_string(),
                vec![("google".to_string(), "abc".to_string()),]
            ),]
        );
        data.cleanup().expect("Could not clean up data store");
        fs::remove_file(import_path).expect("Could not remove import file");
    }

    #[test]
    fn test_remove_parent_when_parent_exists() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![("google".to_string(), "www.google.com".to_string())],
        )];
        let res = data.remove("search-engines", None);
        assert!(res.is_ok());
        assert_eq!(data.state, vec![]);
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_remove_parent_when_parent_does_not_exists() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        let res = data.remove("search-engines", None);
        assert_eq!(
            res.unwrap_err().kind,
            TapDataStoreErrorKind::ParentEntityNotFound
        );
        assert_eq!(data.state, vec![]);
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_remove_parent_and_link_when_parent_exists_and_link_exists() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![("google".to_string(), "www.google.com".to_string())],
        )];
        let res = data.remove("search-engines", Some("google"));
        assert!(res.is_ok());
        assert_eq!(data.state, vec![]);
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_remove_link_when_parent_exists_and_link_exists() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![
                ("google".to_string(), "www.google.com".to_string()),
                ("yahoo".to_string(), "www.yahoo.com".to_string()),
            ],
        )];
        let res = data.remove("search-engines", Some("google"));
        assert!(res.is_ok());
        assert_eq!(
            data.state,
            vec![(
                "search-engines".to_string(),
                vec![("yahoo".to_string(), "www.yahoo.com".to_string())],
            ),]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_remove_link_when_parent_does_not_exist_and_link_exists_in_other_parent() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![("google".to_string(), "www.google.com".to_string())],
        )];
        let res = data.remove("search-engines2", Some("google"));
        assert_eq!(
            res.unwrap_err().kind,
            TapDataStoreErrorKind::ParentEntityNotFound
        );
        assert_eq!(
            data.state,
            vec![(
                "search-engines".to_string(),
                vec![("google".to_string(), "www.google.com".to_string())],
            )]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_upsert_link_when_link_does_not_exists() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![("google".to_string(), "www.google.com".to_string())],
        )];
        let res = data.upsert_link("search-engines", "yahoo", "www.yahoo.com");
        assert!(res.is_ok());
        assert_eq!(
            data.state,
            vec![(
                "search-engines".to_string(),
                vec![
                    ("google".to_string(), "www.google.com".to_string()),
                    ("yahoo".to_string(), "www.yahoo.com".to_string())
                ]
            )]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_upsert_link_when_link_already_exists() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![("google".to_string(), "www.google.com".to_string())],
        )];
        let res = data.upsert_link("search-engines", "google", "something else");
        assert!(res.is_ok());
        assert_eq!(
            data.state,
            vec![(
                "search-engines".to_string(),
                vec![("google".to_string(), "something else".to_string()),]
            )]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_upsert_link_when_no_parent() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "search-engines".to_string(),
            vec![("google".to_string(), "www.google.com".to_string())],
        )];
        let res = data.upsert_link(
            "a-different-parent",
            "google",
            "the same link name should not matter for different parent",
        );
        assert!(res.is_ok());
        assert_eq!(
            data.state,
            vec![
                (
                    "search-engines".to_string(),
                    vec![("google".to_string(), "www.google.com".to_string()),]
                ),
                (
                    "a-different-parent".to_string(),
                    vec![(
                        "google".to_string(),
                        "the same link name should not matter for different parent".to_string()
                    ),]
                )
            ]
        );
        data.cleanup().expect("Could not clean up data store");
    }
}

// Private
impl Data {
    fn parse_file(file_as_str: &str) -> Result<Vec<(String, Vec<LinkValue>)>, TapDataStoreError> {
        fn no_parent_error(parent: &str, links: &[LinkValue]) -> Result<(), TapDataStoreError> {
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
                // TODO: GH-46 in future, would be nice to support escaped pipes
                let (link, value) = line
                    .split_once('|')
                    .ok_or(TapDataStoreError {
                        kind: TapDataStoreErrorKind::ParseError,
                        message: "A link/value line of a data file is expected to contain '|' character separating link and value. For example, google|https://google.com".to_string(),
                    })?;
                validate_link(link)?;
                temp_links.push((link.trim().to_string(), value.trim().to_string()));
            } else {
                if line.trim().is_empty() {
                    continue;
                }
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

    fn state_to_file_string(&mut self) -> (String, Vec<IndexEntry>) {
        // Track offsets for fast reads using index file
        let mut offsets: Vec<IndexEntry> = vec![];
        // Build return string
        let mut res = String::new();

        // Sort state based on parent, then by link
        self.state.sort_by(|a, b| a.0.trim().cmp(b.0.trim()));
        self.state.iter_mut().for_each(|(_, links)| {
            links.sort_by(|a, b| a.0.trim().cmp(b.0.trim()));
        });

        // Build return string & track offsets
        self.state.iter().for_each(|(parent, links)| {
            offsets.push((parent.trim().to_string(), res.len()));

            res.push_str(&format!("{}->\n", parent.trim()));
            links.iter().for_each(|(link, value)| {
                res.push_str(&format!("  {}|{}\n", link.trim(), value.trim()));
            });
        });
        (res, offsets)
    }

    fn save_to_file(&mut self) -> Result<Vec<IndexEntry>, TapDataStoreError> {
        let (str, offsets) = self.state_to_file_string();
        fs::write(&self.path, str).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::FileWriteFailed,
            message: format!("Could not write data file: {e}"),
        })?;
        Ok(offsets)
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
        let mut data = Data::new(Some(data_path), None).unwrap();
        let res = data.state_to_file_string();
        assert_eq!(res.0, "");
        assert_eq!(res.1, vec![]);
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_state_to_file_string_spacing() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
        data.state = vec![(
            "parent1".to_string(),
            vec![("link1".to_string(), "value1".to_string())],
        )];
        let res = data.state_to_file_string();
        assert_eq!(res.0, "parent1->\n  link1|value1\n");
        assert_eq!(res.1, vec![("parent1".to_string(), 0)]);
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_state_to_file_string_sorted() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
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
        let res = data.state_to_file_string();
        assert_eq!(
            res.0,
            "apple->\n  dev|https://developer.apple.com/\n  homepage|www.apple.com\nparent1->\n  link1|value1\n"
        );
        assert_eq!(
            res.1,
            vec![("apple".to_string(), 0), ("parent1".to_string(), 68)]
        );
        data.cleanup().expect("Could not clean up data store");
    }

    #[test]
    fn test_save_to_file() {
        let data_path = get_test_file_path(FileType::Data).expect("Could not get test file path");
        let mut data = Data::new(Some(data_path), None).unwrap();
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
            message: format!("Could not delete data file: {e}"),
        })?;
        Ok(())
    }
}

pub struct Index {
    path: PathBuf,
    state: Vec<IndexEntry>, // parent, offset
}

// Publicly exposed
impl Index {
    pub fn new(path: Option<PathBuf>) -> Result<Self, TapDataStoreError> {
        let (file_exists, path) = if let Some(path) = path {
            (path.exists(), path)
        } else {
            let mut tap_data_path = get_parent_dir_of_tap()?;

            // NOTE: workaround for command tests running at the same time.
            // Use test pathing for tests, otherwise use standard
            // I want to use this over cfg!(test) as I do not want to compile test code in prod builds
            #[allow(unused_mut)]
            let mut test_path: Option<PathBuf> = None;
            #[cfg(test)]
            {
                test_path = Some(get_test_file_path(FileType::Index)?);
            }
            if let Some(test_path) = test_path {
                tap_data_path = test_path;
            } else {
                tap_data_path = tap_data_path.join(".tap_index");
            }
            (tap_data_path.exists(), tap_data_path)
        };

        // Parse file if it exists
        if file_exists {
            let file_as_str = fs::read_to_string(&path).map_err(|e| TapDataStoreError {
                kind: TapDataStoreErrorKind::FileReadFailed,
                message: format!("Could not read index file at {}: {e}", path.display()),
            })?;
            let state = Index::parse_file(&file_as_str)?;
            Ok(Self { path, state })
        } else {
            File::create_new(&path).map_err(|e| TapDataStoreError {
                kind: TapDataStoreErrorKind::FileCreateFailed,
                message: format!("Could not create index file: {e}"),
            })?;
            Ok(Self {
                path,
                state: vec![],
            })
        }
    }

    fn update(&mut self, offsets: Vec<IndexEntry>) {
        self.state = offsets
    }

    pub fn parents(&self) -> Vec<String> {
        self.state
            .iter()
            .map(|(parent, _)| parent.clone())
            .collect()
    }
}

#[cfg(test)]
mod index_public {
    use super::{FileType, Index, get_test_file_path};
    use std::fs;
    use std::path::PathBuf;

    // TODO: GH-45 Move the following to an integration test
    #[test]
    #[ignore = "GH-45: Should really be an integration test - move this out"]
    fn test_new_no_path_correct_file_name() {
        let expected_file_name = ".tap_index";
        let mut index = Index::new(None).unwrap();
        assert!(index.path.to_str().unwrap().ends_with(expected_file_name));
        index.cleanup().expect("Could not clean up index store");
    }

    #[test]
    fn test_new_with_path_correct_file_name() {
        let expected_file_name =
            get_test_file_path(FileType::Index).expect("Could not get test file path");
        let mut index = Index::new(Some(PathBuf::from(&expected_file_name))).unwrap();
        assert_eq!(index.path, expected_file_name);
        index.cleanup().expect("Could not clean index data store");
    }

    #[test]
    fn test_set_state_correct() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        fs::write(&index_path, "parent1|0\nparent2|14\n").unwrap();
        let mut index = Index::new(Some(index_path)).unwrap();
        assert_eq!(
            index.state,
            vec![("parent1".to_string(), 0), ("parent2".to_string(), 14),]
        );
        index.cleanup().expect("Could not clean up index store");
    }

    #[test]
    fn test_update_state() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        let mut index = Index::new(Some(index_path)).unwrap();
        index.update(vec![
            ("parent1".to_string(), 0),
            ("parent2".to_string(), 14),
        ]);
        assert_eq!(
            index.state,
            vec![("parent1".to_string(), 0), ("parent2".to_string(), 14)]
        );
        index.cleanup().expect("Could not clean up index store");
    }
}

// Privately exposed
impl Index {
    /// If the index goes to the end of the file, return 0 for length. This indicates to
    /// the caller that they need to make a buffer large enough to read to EOF.
    fn find_parent_offset_and_length(
        &self,
        parent: String,
    ) -> Result<IndexOffsetLength, TapDataStoreError> {
        let elem = self
            .state
            .iter()
            .position(|(p, _)| p.trim() == parent.trim());
        match elem {
            Some(elem) => {
                if elem == self.state.len() - 1 {
                    return Ok((self.state[elem].1, 0));
                }
                Ok((
                    self.state[elem].1,
                    self.state[elem + 1].1 - self.state[elem].1,
                ))
            }
            None => Err(TapDataStoreError {
                kind: TapDataStoreErrorKind::ParseError,
                message: format!("Could not find parent '{parent}' in index"),
            }),
        }
    }

    fn parse_file(file_as_str: &str) -> Result<Vec<IndexEntry>, TapDataStoreError> {
        let mut state = vec![];
        for line in file_as_str.lines() {
            if line.contains('|') {
                let (parent, offset) = line.split_once('|').ok_or(TapDataStoreError {
                    kind: TapDataStoreErrorKind::ParseError,
                    message: format!(
                        "A parent, offset line of an index file is expected to contain '|' character separating parent and offset. Line '{line}' does not match expected format of parent|offset\n"
                    ),
                })?;
                let offset_parsed: usize = offset.parse().map_err(|e| TapDataStoreError {
                    kind: TapDataStoreErrorKind::ParseError,
                    message: format!(
                        "Line '{line}' of index file does not have a valid offset: {e}\n"
                    ),
                })?;
                state.push((parent.to_string(), offset_parsed));
            } else {
                return Err(TapDataStoreError {
                    kind: TapDataStoreErrorKind::ParseError,
                    message: format!(
                        "Unknown format for index file. Line '{line}' does not match expected format of parent|offset\n"
                    ),
                });
            }
        }
        Ok(state)
    }

    fn state_to_file_string(&mut self) -> String {
        // Sort by parent
        self.state.sort_by(|a, b| a.0.trim().cmp(b.0.trim()));
        let mut res = String::new();
        for (parent, offset) in &self.state {
            res.push_str(&format!("{}|{offset}\n", parent.trim()));
        }
        res
    }

    fn save_to_file(&mut self) -> Result<(), TapDataStoreError> {
        let str = self.state_to_file_string();
        fs::write(&self.path, str).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::FileWriteFailed,
            message: format!("Could not write index file: {e}"),
        })
    }
}

#[cfg(test)]
mod index_private {
    use super::{FileType, Index, TapDataStoreErrorKind, get_test_file_path};
    use std::fs;
    use std::path::PathBuf;

    fn cleanup_test_file(file_path: &PathBuf) {
        fs::remove_file(file_path).expect("Could not remove test file");
    }

    #[test]
    fn test_parse_file_empty() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        fs::write(&index_path, "").unwrap();
        let res = Index::parse_file(fs::read_to_string(&index_path).unwrap().as_str())
            .expect("Could not parse file");
        assert_eq!(res, vec![]);
        cleanup_test_file(&index_path);
    }
    #[test]
    fn test_parse_file_valid_one_index() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        fs::write(&index_path, "parent1|0\n").unwrap();
        let res = Index::parse_file(fs::read_to_string(&index_path).unwrap().as_str())
            .expect("Could not parse file");
        assert_eq!(res, vec![("parent1".to_string(), 0)]);
        cleanup_test_file(&index_path);
    }

    #[test]
    fn test_parse_file_valid_two_indices() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        fs::write(&index_path, "search engines|0\ncoding|50\n").unwrap();
        let res = Index::parse_file(fs::read_to_string(&index_path).unwrap().as_str())
            .expect("Could not parse file");
        assert_eq!(
            res,
            vec![
                ("search engines".to_string(), 0),
                ("coding".to_string(), 50)
            ]
        );
        cleanup_test_file(&index_path);
    }

    #[test]
    fn test_parse_file_invalid_offset() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        fs::write(
            &index_path,
            "valid|0\nsearch engines|not an offset\nanother valid|10\n",
        )
        .unwrap();
        let res = Index::parse_file(fs::read_to_string(&index_path).unwrap().as_str());
        assert_eq!(res.unwrap_err().kind, TapDataStoreErrorKind::ParseError);
        cleanup_test_file(&index_path);
    }

    #[test]
    fn test_parse_file_invalid_random_file() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        fs::write(
            &index_path,
            "Something that is completely not a index file was read",
        )
        .unwrap();
        let res = Index::parse_file(fs::read_to_string(&index_path).unwrap().as_str());
        assert_eq!(res.unwrap_err().kind, TapDataStoreErrorKind::ParseError);
        cleanup_test_file(&index_path);
    }

    #[test]
    fn test_state_to_file_string_empty() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        let mut data = Index::new(Some(index_path)).unwrap();
        let res = data.state_to_file_string();
        assert_eq!(res, "");
        data.cleanup().expect("Could not clean up index store");
    }

    #[test]
    fn test_state_to_file_string_valid() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        let mut index = Index::new(Some(index_path)).unwrap();
        index.state = vec![("parent1".to_string(), 0)];
        let res = index.state_to_file_string();
        assert_eq!(res, "parent1|0\n");
        index.cleanup().expect("Could not clean up index store");
    }

    #[test]
    fn test_state_to_file_string_sorted() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        let mut index = Index::new(Some(index_path)).unwrap();
        index.state = vec![("parent1".to_string(), 0), ("apple".to_string(), 50)];
        let res = index.state_to_file_string();
        assert_eq!(res, "apple|50\nparent1|0\n");
        assert_eq!(
            index.state,
            vec![("apple".to_string(), 50), ("parent1".to_string(), 0)]
        );
        index.cleanup().expect("Could not clean up index store");
    }

    #[test]
    fn test_save_to_file() {
        let index_path = get_test_file_path(FileType::Index).expect("Could not get test file path");
        let mut index = Index::new(Some(index_path)).unwrap();
        index.state = vec![("parent1".to_string(), 0)];
        index.save_to_file().expect("Could not save to file");
        let res = fs::read_to_string(&index.path).unwrap();
        assert_eq!(res, "parent1|0\n");
        index.cleanup().expect("Could not clean up index store");
    }
}

#[cfg(test)]
impl Index {
    fn cleanup(&mut self) -> Result<(), TapDataStoreError> {
        fs::remove_file(&self.path).map_err(|e| TapDataStoreError {
            kind: TapDataStoreErrorKind::FileDeleteFailed,
            message: format!("Could not delete index file: {e}"),
        })?;
        Ok(())
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
            message: format!("Parent entity name {parent} is reserved"),
        });
    }
    Ok(())
}

/// Check if the link name is valid
/// ## Errors
/// - `TapDataStoreErrorKind::ReservedKeyword` - if link name uses a reserved keyword
fn validate_link(link: &str) -> Result<(), TapDataStoreError> {
    // TODO: GH-47 Add new error for invalid file URI / URL
    if link.contains("|") {
        return Err(TapDataStoreError {
            kind: TapDataStoreErrorKind::ReservedKeyword,
            message: format!("Link name {link} contains a vertical bar '|' which is reserved"),
        });
    }
    Ok(())
}

/// Checks if the file extension is valid for the import file type
fn validate_path(file_type: &ImportType, path: &Path) -> Result<(), TapDataStoreError> {
    match file_type {
        ImportType::Tap => {
            let extension = path.extension().ok_or(TapDataStoreError {
                kind: TapDataStoreErrorKind::InvalidFileExtension,
                message: format!(
                    "Unable to get file extension for tap import file: {}",
                    path.display()
                ),
            })?;
            if extension != "tap" {
                Err(TapDataStoreError {
                    kind: TapDataStoreErrorKind::InvalidFileExtension,
                    message: format!(
                        "Invalid file extension for tap import file: {}",
                        path.display()
                    ),
                })?;
            }
            Ok(())
        }
    }?;
    Ok(())
}

#[cfg(test)]
enum FileType {
    Data,
    Index,
    Tap,
}

pub enum ImportType {
    Tap,
}

#[cfg(test)]
/// Returns a test file path for either an index or data file. A test file name is of the format:
/// - Index files: .tap_index_{test_name}_{timestamp}
/// - Data files: .tap_data_{test_name}_{timestamp_millis}
/// - Tap files: {test_name}_{timestamp_millis}.tap
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
        FileType::Data => path_buf.join(format!(".tap_data_{test_name}_{timestamp}")),
        FileType::Index => path_buf.join(format!(".tap_index_{test_name}_{timestamp}")),
        FileType::Tap => path_buf.join(format!("{test_name}_{timestamp}.tap")),
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

// Errors
#[derive(Debug, PartialEq)]
pub enum TapDataStoreErrorKind {
    #[cfg(test)]
    CurrentTimeError,
    ExecutablePathNotFound,
    ExecutablePathParentDirectoryNotFound,
    FileCreateFailed,
    #[cfg(test)]
    FileDeleteFailed,
    FileReadFailed,
    FileReadMetadataFailed,
    FileOpenFailed,
    FileSeekFailed,
    FileWriteFailed,
    InvalidFileExtension,
    LinkAlreadyExists,
    LinkNotFound,
    ParentEntityNotFound,
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
            #[cfg(test)]
            TapDataStoreErrorKind::CurrentTimeError => write!(f, "Current time error"),
            TapDataStoreErrorKind::ExecutablePathNotFound => {
                write!(f, "Executable path not found")
            }
            TapDataStoreErrorKind::ExecutablePathParentDirectoryNotFound => {
                write!(f, "Executable path parent directory not found")
            }
            TapDataStoreErrorKind::FileCreateFailed => write!(f, "File create failed"),
            #[cfg(test)]
            TapDataStoreErrorKind::FileDeleteFailed => write!(f, "File delete failed"),
            TapDataStoreErrorKind::FileOpenFailed => write!(f, "File open failed"),
            TapDataStoreErrorKind::FileSeekFailed => write!(f, "File seek failed"),
            TapDataStoreErrorKind::FileReadFailed => write!(f, "File read failed"),
            TapDataStoreErrorKind::FileReadMetadataFailed => {
                write!(f, "File read metadata failed")
            }
            TapDataStoreErrorKind::FileWriteFailed => write!(f, "File write failed"),
            TapDataStoreErrorKind::InvalidFileExtension => write!(f, "Invalid file extension"),
            TapDataStoreErrorKind::LinkAlreadyExists => write!(f, "Link already exists"),
            TapDataStoreErrorKind::LinkNotFound => write!(f, "Link not found"),
            TapDataStoreErrorKind::ParentEntityNotFound => write!(f, "Parent entity not found"),
            TapDataStoreErrorKind::ParseError => write!(f, "Parse error"),
            TapDataStoreErrorKind::ReservedKeyword => write!(f, "Reserved keyword used"),
        }
    }
}
