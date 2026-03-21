mod data;
mod index;

use super::os_implementations::{OsImplementationError, derive_data_local_dir_by_os};
use data::{D, Data, DataError, DeleteTarget};
use index::{Index, IndexError, IndexErrorKind};
use std::{
    fmt::{self},
    fs,
    io::{Read, Seek, Write},
    path::PathBuf,
};

pub enum ImportExportType {
    Browser,
    Tap,
}

pub trait DS {
    fn delete(&mut self, parent_entity: &str, link: Option<&str>) -> Result<(), DataStoreError>;
    fn import(
        &mut self,
        path: PathBuf,
        import_type: ImportExportType,
    ) -> Result<(), DataStoreError>;
    fn export(
        &mut self,
        path: PathBuf,
        export_type: ImportExportType,
    ) -> Result<String, DataStoreError>;
    fn read_parent(&mut self, parent_entity: &str) -> Result<Vec<D>, DataStoreError>;
    fn read_link(&mut self, parent_entity: &str, link: &str) -> Result<D, DataStoreError>;
    fn parents(&mut self) -> Result<Vec<String>, DataStoreError>;
    fn upsert_link(
        &mut self,
        parent_entity: String,
        link_name: String,
        value: String,
    ) -> Result<(), DataStoreError>;
}

pub struct Datastore<RW: Read + Write + Seek> {
    d: Data<RW>,
    i: Index<RW>,
}

impl<RW: Read + Write + Seek> DS for Datastore<RW> {
    fn delete(&mut self, parent_entity: &str, link: Option<&str>) -> Result<(), DataStoreError> {
        match (parent_entity, link) {
            (p, None) => {
                // If there is no link, then delete without a check
                self.i.idx_delete(vec![p])?;
                self.d
                    .data_delete(DeleteTarget::Parent { parent_entity: p })?;
                Ok(())
            }
            (p, Some(l)) => {
                // IF there is link, then get the current Data position with Idx cache, and then
                // update both Data and Index structures
                let idx = self.i.idx_read(p)?;
                let new_idx = self.d.data_delete(DeleteTarget::Link {
                    idx: &idx,
                    parent_entity: p,
                    link: l,
                })?;
                if let Some(new_idx) = new_idx {
                    self.i.idx_upsert(vec![new_idx])?;
                    Ok(())
                } else {
                    Err(DataStoreError {
                        kind: DataStoreErrorKind::Data,
                        message:
                            "Expected new Idx to be returned by Data structure, but received None"
                                .to_string(),
                    })
                }
            }
        }
    }

    fn export(
        &mut self,
        path: PathBuf,
        export_type: ImportExportType,
    ) -> Result<String, DataStoreError> {
        todo!()
    }

    fn import(
        &mut self,
        path: PathBuf,
        import_type: ImportExportType,
    ) -> Result<(), DataStoreError> {
        todo!()
    }

    fn read_parent(&mut self, parent_entity: &str) -> Result<Vec<D>, DataStoreError> {
        let idx = self.i.idx_read(parent_entity)?;
        let links = self.d.data_read(&idx, parent_entity, None)?;
        Ok(links)
    }

    fn read_link(&mut self, parent_entity: &str, link: &str) -> Result<D, DataStoreError> {
        let idx = self.i.idx_read(parent_entity)?;
        let links = self.d.data_read(&idx, parent_entity, Some(link))?;
        let mut iter = links.into_iter();

        let first = iter.next().ok_or_else(|| DataStoreError {
            kind: DataStoreErrorKind::Data,
            message: "Expected Data structure to return one link, received 0".to_string(),
        })?;

        if iter.next().is_some() {
            return Err(DataStoreError {
                kind: DataStoreErrorKind::Data,
                message: "Expected Data structure to return one link, received more than one"
                    .to_string(),
            });
        }

        Ok(first)
    }

    fn parents(&mut self) -> Result<Vec<String>, DataStoreError> {
        Ok(self.i.idx_parents()?.into_iter().collect())
    }

    fn upsert_link(
        &mut self,
        parent_entity: String,
        link_name: String,
        value: String,
    ) -> Result<(), DataStoreError> {
        let idx = match self.i.idx_read(&parent_entity) {
            Ok(idx) => Some(idx),
            Err(e) if e.kind == IndexErrorKind::NotFound => None,
            Err(e) => return Err(e.into()),
        };
        let new_idx = self.d.data_upsert(
            idx.as_ref(),
            &parent_entity,
            vec![(parent_entity.clone(), link_name, value)],
        )?;
        self.i.idx_upsert(vec![new_idx])?;
        Ok(())
    }
}

impl Datastore<fs::File> {
    pub fn new() -> Result<Self, DataStoreError> {
        let path = derive_data_local_dir_by_os("dev", "CharlieKarafotias", "Tap")?;
        let index_path = path.join("tap_index.txt");
        let data_path = path.join("tap_data.txt");

        // Check for existence of Tap directory (if it doesn't exist, create it)
        fs::create_dir_all(&path)?;

        let data_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(data_path)?;
        let index_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(index_path)?;

        Ok(Datastore {
            d: Data::new(data_file),
            i: Index::new(index_file),
        })
    }
}

#[cfg(test)]
impl<RW: Read + Write + Seek> Datastore<RW> {
    pub fn new_in_memory(data: RW, index: RW) -> Self {
        Datastore {
            d: Data::new(data),
            i: Index::new(index),
        }
    }
}

// Errors
#[derive(Debug, PartialEq)]
pub enum DataStoreErrorKind {
    Data,
    Index,
    OS,
}

#[derive(Debug)]
pub struct DataStoreError {
    kind: DataStoreErrorKind,
    message: String,
}

impl fmt::Display for DataStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (datastore error: {})", self.message, self.kind)
    }
}

impl fmt::Display for DataStoreErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataStoreErrorKind::Data => write!(f, "Data Buffer Error"),
            DataStoreErrorKind::Index => write!(f, "Index Buffer Error"),
            DataStoreErrorKind::OS => write!(f, "OS Error"),
        }
    }
}

impl From<DataError> for DataStoreError {
    fn from(value: DataError) -> Self {
        DataStoreError {
            kind: DataStoreErrorKind::Data,
            message: value.message,
        }
    }
}

impl From<IndexError> for DataStoreError {
    fn from(value: IndexError) -> Self {
        DataStoreError {
            kind: DataStoreErrorKind::Index,
            message: value.message,
        }
    }
}

impl From<OsImplementationError> for DataStoreError {
    fn from(value: OsImplementationError) -> Self {
        DataStoreError {
            kind: DataStoreErrorKind::OS,
            message: value.message,
        }
    }
}

impl From<std::io::Error> for DataStoreError {
    fn from(err: std::io::Error) -> Self {
        DataStoreError {
            kind: DataStoreErrorKind::OS,
            message: err.to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn setup() -> Datastore<Cursor<Vec<u8>>> {
        Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]))
    }

    #[test]
    fn test_upsert_new_parent() {
        let mut ds = setup();

        ds.upsert_link(
            "search".to_string(),
            "google".to_string(),
            "https://google.com".to_string(),
        )
        .unwrap();

        let res = ds.read_parent("search").unwrap();

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].1, "google");
        assert_eq!(res[0].2, "https://google.com");
    }

    #[test]
    fn test_upsert_existing_link_updates_value() {
        let mut ds = setup();

        ds.upsert_link("search".into(), "google".into(), "old".into())
            .unwrap();

        ds.upsert_link("search".into(), "google".into(), "new".into())
            .unwrap();

        let res = ds.read_parent("search").unwrap();

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].2, "new");
    }

    #[test]
    fn test_read_link_success() {
        let mut ds = setup();

        ds.upsert_link("search".into(), "google".into(), "g".into())
            .unwrap();
        ds.upsert_link("search".into(), "yahoo".into(), "y".into())
            .unwrap();

        let res = ds.read_link("search", "yahoo").unwrap();

        assert_eq!(res.1, "yahoo");
        assert_eq!(res.2, "y");
    }

    #[test]
    fn test_read_link_not_found() {
        let mut ds = setup();

        ds.upsert_link("search".into(), "google".into(), "g".into())
            .unwrap();

        let err = ds.read_link("search", "bing").unwrap_err();

        assert_eq!(err.kind, DataStoreErrorKind::Data);
    }

    #[test]
    fn test_parents() {
        let mut ds = setup();

        ds.upsert_link("search".into(), "google".into(), "g".into())
            .unwrap();
        ds.upsert_link("social".into(), "twitter".into(), "t".into())
            .unwrap();

        let mut parents = ds.parents().unwrap();
        parents.sort();

        assert_eq!(parents, vec!["search".to_string(), "social".to_string()]);
    }

    #[test]
    fn test_delete_parent() {
        let mut ds = setup();

        ds.upsert_link("search".into(), "google".into(), "g".into())
            .unwrap();

        ds.delete("search", None).unwrap();

        let err = ds.read_parent("search").unwrap_err();
        assert_eq!(err.kind, DataStoreErrorKind::Index);
    }

    #[test]
    fn test_delete_link() {
        let mut ds = setup();

        ds.upsert_link("search".into(), "google".into(), "g".into())
            .unwrap();
        ds.upsert_link("search".into(), "yahoo".into(), "y".into())
            .unwrap();

        ds.delete("search", Some("google")).unwrap();

        let res = ds.read_parent("search").unwrap();

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].1, "yahoo");
    }

    #[test]
    fn test_delete_missing_link_errors() {
        let mut ds = setup();

        ds.upsert_link("search".into(), "google".into(), "g".into())
            .unwrap();

        let err = ds.delete("search", Some("bing")).unwrap_err();

        assert_eq!(err.kind, DataStoreErrorKind::Data);
    }
}
