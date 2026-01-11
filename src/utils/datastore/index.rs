use std::{
    collections::HashSet,
    fmt,
    io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
};

/// An Index entry to the Index file.
/// Format: parent_entity, latest_byte_offset, length, generation
type Idx = (String, usize, usize, usize);

enum IndexType {
    Entry(Idx),
    Tombstone(String),
}

/// Build & maintains an append-only Index structure
///
/// Purpose:
///     - A file cache that stores line entries of the following formats:
///         - `parent_entity|latest_byte_offset|length|generation` (an index)
///         - `-parent_entity` (a removed index)
///     - Enables faster reads by Data struct by utilizing the `latest_byte_offset` and `length` to
///     Seek only the bytes required to search over for parent->link relationship
///
/// Supported operations:
///     - Create index(es)
///     - Read index
///     - Update index(es)
///     - Upsert index(es)
///     - Delete index(es)
///     - Compact index representation
pub(in super::super::datastore) struct Index<RW: Read + Write + Seek> {
    buf: RW,
}

impl<RW: Read + Write + Seek> Index<RW> {
    /// Creates a new `Index` representation. An Index representation is any type
    /// which implements the Read, Seek, and Write traits.
    ///
    /// In Production, a file is used under the name `tap_index.txt` to store the index data. The index file
    /// is stored in the proper platform-specific, user-accessible location:
    ///
    /// Linux: `$XDG_DATA_HOME/Tap/tap_index.txt` OR `$HOME/.local/share/Tap/tap_index.txt`
    /// macOS: `$HOME/Library/Application Support/dev.CharlieKarafotias.Tap/tap_index.txt`
    /// Windows: `{FOLDERID_LocalAppData}\CharlieKarafotias\Tap\data\tap_index.txt`
    pub fn new(buf: RW) -> Self {
        Index { buf }
    }
    /// Create new indexes
    ///
    /// NOTE: this is slower than idx_upsert as a check for the existance of the index(es) provided
    /// is performed prior to creating the indexes. If you do not need to check for existance, use
    /// idx_upsert.
    ///
    /// Errors:
    ///     - `AlreadyExists`: one or more of the indexes specified already exists in the Index structure (the indexes listed in the error were not added to the Index structure)
    ///     - `Write`: a write operation to the Index failed
    pub fn idx_create(&mut self, entries: Vec<Idx>) -> Result<(), IndexError> {
        let mut parents = self.idx_parents()?;
        let mut err_parent_already_exist: Vec<String> = Vec::with_capacity(entries.len());

        let mut to_write: Vec<String> = Vec::new();

        for entry in entries {
            if !parents.insert(entry.0.clone()) {
                err_parent_already_exist.push(entry.0);
            } else {
                to_write.push(format!("{}|{}|{}|{}", entry.0, entry.1, entry.2, entry.3));
            }
        }

        if !to_write.is_empty() {
            let mut w = BufWriter::new(&mut self.buf);
            w.seek(SeekFrom::End(0)).map_err(|e| IndexError {
                kind: IndexErrorKind::Write,
                message: format!("Failed to seek to end: {e}"),
            })?;
            for line in to_write {
                writeln!(w, "{line}").map_err(|e| IndexError {
                    kind: IndexErrorKind::Write,
                    message: format!("Failed to write line {line}: {e}"),
                })?;
            }
            w.flush().map_err(|e| IndexError {
                kind: IndexErrorKind::Write,
                message: format!("Flush failed: {e}"),
            })?;
        }

        if !err_parent_already_exist.is_empty() {
            Err(IndexError {
                kind: IndexErrorKind::AlreadyExists,
                message: format!(
                    "The following indexes already exist and therefore were not added: {err_parent_already_exist:?}"
                ),
            })
        } else {
            Ok(())
        }
    }
    /// Reads an index from the Index file
    ///
    /// Errors:
    ///     - `Read`: the read operation of the Index file failed
    ///     - `NotFound`: the index specified was not found in the Index file
    pub fn idx_read(&mut self, parent_entity: &str) -> Result<Idx, IndexError> {
        let mut reader = BufReader::new(&mut self.buf);
        reader.seek(SeekFrom::Start(0)).map_err(|e| IndexError {
            kind: IndexErrorKind::Read,
            message: format!("Failed to seek to start: {e}"),
        })?;

        let mut result: Option<Idx> = None;

        for l in reader.lines() {
            let line = l.map_err(|e| IndexError {
                kind: IndexErrorKind::Read,
                message: format!("Line read failed - {e}"),
            })?;

            match parse_line(&line)? {
                IndexType::Entry(idx) if idx.0 == parent_entity => {
                    result = Some(idx);
                }
                IndexType::Tombstone(parent) if parent == parent_entity => {
                    result = None;
                }
                _ => {}
            }
        }
        result.ok_or(IndexError {
            kind: IndexErrorKind::NotFound,
            message: format!("The following index was not found: {parent_entity}"),
        })
    }
    /// Updates indexes in the Index structure by appending them to end of structure
    ///
    /// NOTE: the index must exist in order to be updated. Use idx_upsert if you wish to
    /// create/update instead. This is also slower than idx_upsert as the index check exists.
    ///
    /// Errors:
    ///     - `Read`: a read operation to the Index failed
    ///     - `NotFound`: the index(es) specified was not found
    ///     - `Write`: a write operation to the Index failed
    pub fn idx_update(&mut self, entries: Vec<Idx>) -> Result<(), IndexError> {
        let parents = self.idx_parents()?;
        let mut err_parent_not_found: Vec<String> = Vec::with_capacity(entries.len());
        let mut to_write: Vec<String> = Vec::new();

        for entry in entries {
            if parents.contains(&entry.0) {
                to_write.push(format!("{}|{}|{}|{}", entry.0, entry.1, entry.2, entry.3));
            } else {
                err_parent_not_found.push(entry.0);
            }
        }

        if !to_write.is_empty() {
            let mut w = BufWriter::new(&mut self.buf);
            w.seek(SeekFrom::End(0)).map_err(|e| IndexError {
                kind: IndexErrorKind::Write,
                message: format!("Failed to seek to end: {e}"),
            })?;
            for line in to_write {
                writeln!(w, "{line}").map_err(|e| IndexError {
                    kind: IndexErrorKind::Write,
                    message: format!("Failed to write line {line}: {e}"),
                })?;
            }
            w.flush().map_err(|e| IndexError {
                kind: IndexErrorKind::Write,
                message: format!("Flush failed: {e}"),
            })?;
        }

        if !err_parent_not_found.is_empty() {
            Err(IndexError {
                kind: IndexErrorKind::NotFound,
                message: format!(
                    "The following indexes were not found and therefore not updated: {err_parent_not_found:?}"
                ),
            })
        } else {
            Ok(())
        }
    }
    /// Upserts indexes in the Index structure by appending them to end of structure. If the index
    /// does not exist, it is created.
    ///
    /// NOTE: this is faster than idx_update as its an append only operation that does not need to
    /// check parents first
    ///
    /// Errors:
    ///     - `Write`: a write operation to the Index failed
    pub fn idx_upsert(&mut self, entries: Vec<Idx>) -> Result<(), IndexError> {
        let mut writer = BufWriter::new(&mut self.buf);
        writer.seek(SeekFrom::End(0)).map_err(|e| IndexError {
            kind: IndexErrorKind::Write,
            message: format!("Failed to seek to end: {e}"),
        })?;

        for entry in entries {
            writeln!(writer, "{}|{}|{}|{}", entry.0, entry.1, entry.2, entry.3);
        }
        writer.flush().map_err(|e| IndexError {
            kind: IndexErrorKind::Write,
            message: format!("Flush failed: {e}"),
        })?;
        Ok(())
    }
    /// Deletes one or more indexes from the Index structure
    ///
    /// NOTE: to speed up implementation, a tombstone is appended to the end without checking if
    /// the indexes passed in actually exist or not.
    ///
    /// Errors:
    ///     - `Write`: a write operation to the Index failed
    pub fn idx_delete(&mut self, entries: Vec<&str>) -> Result<(), IndexError> {
        let mut writer = BufWriter::new(&mut self.buf);
        writer.seek(SeekFrom::End(0)).map_err(|e| IndexError {
            kind: IndexErrorKind::Write,
            message: format!("Failed to seek to end: {e}"),
        })?;

        for entry in entries {
            writeln!(writer, "-{entry}");
        }
        writer.flush().map_err(|e| IndexError {
            kind: IndexErrorKind::Write,
            message: format!("Flush failed: {e}"),
        })?;
        Ok(())
    }
    /// Compacts the Index file by removing older generations
    ///
    /// Errors:
    ///     - TODO
    pub fn idx_compact() -> () {
        todo!()
    }

    /// Retrieves the parent entities from the Index file
    ///
    /// Errors:
    ///     - `Read`: the read operation of the Index file failed
    ///     - `Parse`: parsing of an Index entry in the Index file failed suggesting the Index file
    ///                is corrupted
    fn idx_parents(&mut self) -> Result<HashSet<String>, IndexError> {
        let mut reader = BufReader::new(&mut self.buf);
        reader.seek(SeekFrom::Start(0)).map_err(|e| IndexError {
            kind: IndexErrorKind::Read,
            message: format!("Failed to seek to start: {e}"),
        })?;
        let mut parents: HashSet<String> = HashSet::new();
        for line in reader.lines() {
            let line = line.map_err(|e| IndexError {
                kind: IndexErrorKind::Read,
                message: format!("Line read failed - {e}"),
            })?;
            match parse_line(&line)? {
                IndexType::Tombstone(parent) => parents.remove(&parent),
                IndexType::Entry((parent, _, _, _)) => parents.insert(parent),
            };
        }
        Ok(parents)
    }
}

/// Parses Index entry from from string representation into the IndexType format
///
/// Errors:
///     - Parse: the current line is not parsable into a Tombstone or Entry representation
fn parse_line(line: &str) -> Result<IndexType, IndexError> {
    // line like -parent_entity is a tombstone
    if line.starts_with('-') && !line.contains('|') {
        let parent: &str = line.splitn(2, '-').next().take().ok_or(IndexError {
            kind: IndexErrorKind::Parse,
            message: format!("Expected a parent entity to follow '-' but received line {line}"),
        })?;
        return Ok(IndexType::Tombstone(parent.to_string()));
    }
    // line like parent_entity|latest_offset|length|generation => add parent_entity to set
    let vals: Vec<&str> = line.split('|').collect();
    if vals.len() != 4 {
        return Err(IndexError {
            kind: IndexErrorKind::Parse,
            message: format!(
                "Expected index entry to contain 3 '|' separators, but received line {line}"
            ),
        });
    }
    Ok(IndexType::Entry((
            vals[0].parse::<String>().map_err(|_| IndexError { kind: IndexErrorKind::Parse, message: format!("Expected first part of Index entry to be a parent entry of string format. Received line {line}")})?,
            vals[1].parse::<usize>().map_err(|_| IndexError { kind: IndexErrorKind::Parse, message: format!("Expected second part of Index entry to be a positive number. Received line {line}")})?,
            vals[2].parse::<usize>().map_err(|_| IndexError { kind: IndexErrorKind::Parse, message: format!("Expected third part of Index entry to be a positive number. Received line {line}")})?,
            vals[3].parse::<usize>().map_err(|_| IndexError { kind: IndexErrorKind::Parse, message: format!("Expected fourth part of Index entry to be a positive number. Received line {line}")})?,
        )))
}

// Errors
#[derive(Debug, PartialEq)]
pub enum IndexErrorKind {
    AlreadyExists,
    NotFound,
    Parse,
    Read,
    Write,
}

#[derive(Debug)]
pub struct IndexError {
    kind: IndexErrorKind,
    message: String,
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (index error: {})", self.message, self.kind)
    }
}

impl fmt::Display for IndexErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IndexErrorKind::AlreadyExists => write!(f, "Index already exists"),
            IndexErrorKind::NotFound => write!(f, "Index not found"),
            IndexErrorKind::Read => write!(f, "Index read failed"),
            IndexErrorKind::Parse => write!(f, "Index file structure corrupted"),
            IndexErrorKind::Write => write!(f, "Index write failed"),
        }
    }
}
