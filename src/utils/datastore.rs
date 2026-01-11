mod data;
mod index;

use super::super::utils::os_implementations::derive_data_local_dir_by_os;
use index::Index;
use std::{
    fmt::{self},
    fs,
    io::{Read, Write},
};

struct Datastore<RW: Read + Write> {
    // d: Data<RW>,
    i: Index<RW>,
}

impl Datastore<fs::File> {
    fn new() -> Result<Self, DataStoreError> {
        let path = derive_data_local_dir_by_os("dev", "CharlieKarafotias", "Tap").map_err(|e| {
            DataStoreError {
                kind: DataStoreErrorKind::OS,
                message: e.to_string(),
            }
        })?;
        let index_path = path.join("tap_index.txt");
        let data_path = path.join("tap_data.txt");

        // Check for existance of Tap directory (if it doesn't exist, create it)
        fs::create_dir_all(&path).map_err(|e| DataStoreError {
            kind: DataStoreErrorKind::OS,
            message: e.to_string(),
        })?;

        let data_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(data_path)
            .map_err(|e| DataStoreError {
                kind: DataStoreErrorKind::OS,
                message: e.to_string(),
            })?;
        let index_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(index_path)
            .map_err(|e| DataStoreError {
                kind: DataStoreErrorKind::OS,
                message: e.to_string(),
            })?;
        Ok(Datastore {
            // d: data_file,
            i: Index::new(index_file),
        })
    }
}

#[cfg(test)]
impl<RW: Read + Write> Datastore<RW> {
    fn new_in_memory(index: RW) -> Self {
        Datastore {
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
