use std::{
    collections::HashSet,
    fmt,
    io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
};

/// An Index entry to the Index file.
/// Format: parent_entity, latest_byte_offset, length, generation
pub(super) type Idx = (String, usize, usize, usize);

#[derive(Debug, PartialEq)]
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
pub(super) struct Index<RW: Read + Write + Seek> {
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
            writeln!(writer, "{}|{}|{}|{}", entry.0, entry.1, entry.2, entry.3).map_err(|e| {
                IndexError {
                    kind: IndexErrorKind::Write,
                    message: format!(
                        "Failed to write line {}|{}|{}|{}: {e}",
                        entry.0, entry.1, entry.2, entry.3
                    ),
                }
            })?;
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
            writeln!(writer, "-{entry}").map_err(|e| IndexError {
                kind: IndexErrorKind::Write,
                message: format!("Failed to write line -{entry}: {e}"),
            })?;
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
    pub fn idx_parents(&mut self) -> Result<HashSet<String>, IndexError> {
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

/// Parses Index entry from string representation into the IndexType format
///
/// Errors:
///     - Parse: the current line is not parsable into a Tombstone or Entry representation
fn parse_line(line: &str) -> Result<IndexType, IndexError> {
    // line like -parent_entity is a tombstone
    if line.starts_with('-') && !line.contains('|') {
        let parent = line
            .strip_prefix('-')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(IndexError {
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
        Some(vals[0])
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(IndexError {
                kind: IndexErrorKind::Parse,
                message: format!(
                    "Expected first part of Index entry to be a parent entry of string format. Received line {line}"
                ),
            })?
            .to_string(),
        vals[1].parse::<usize>().map_err(|_| IndexError {
            kind: IndexErrorKind::Parse,
            message: format!(
                "Expected second part of Index entry to be a positive number. Received line {line}"
            ),
        })?,
        vals[2].parse::<usize>().map_err(|_| IndexError {
            kind: IndexErrorKind::Parse,
            message: format!(
                "Expected third part of Index entry to be a positive number. Received line {line}"
            ),
        })?,
        vals[3].parse::<usize>().map_err(|_| IndexError {
            kind: IndexErrorKind::Parse,
            message: format!(
                "Expected fourth part of Index entry to be a positive number. Received line {line}"
            ),
        })?,
    )))
}

// Errors
#[derive(Debug, PartialEq)]
pub(super) enum IndexErrorKind {
    AlreadyExists,
    NotFound,
    Parse,
    Read,
    Write,
}

#[derive(Debug, PartialEq)]
pub(super) struct IndexError {
    pub(super) kind: IndexErrorKind,
    pub(super) message: String,
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

#[cfg(test)]
mod ds_index_tests_private_api {
    use super::*;

    #[test]
    fn test_parse_line_entry_valid() {
        let expected = IndexType::Entry(("some_parent".to_string(), 20, 10, 5));
        assert_eq!(parse_line("some_parent|20|10|5").unwrap(), expected);
    }
    #[test]
    fn test_parse_line_tombstone_valid() {
        let expected = IndexType::Tombstone("some_parent".to_string());
        assert_eq!(parse_line("-some_parent").unwrap(), expected);
    }
    #[test]
    fn test_parse_line_against_empty_tombstone_returns_parse_error() {
        let expected = IndexError {
            kind: IndexErrorKind::Parse,
            message: "Expected a parent entity to follow '-' but received line -".to_string(),
        };
        assert_eq!(parse_line("-").unwrap_err(), expected);
    }
    #[test]
    fn test_parse_line_missing_3_separators_returns_parse_error() {
        let expected = IndexError {
            kind: IndexErrorKind::Parse,
            message: "Expected index entry to contain 3 '|' separators, but received line some_parent|1|2".to_string(),
        };
        assert_eq!(parse_line("some_parent|1|2").unwrap_err(), expected);
    }
    #[test]
    fn test_parse_line_parent_entity_not_string_returns_parse_error() {
        let expected = IndexError {
            kind: IndexErrorKind::Parse,
            message: "Expected first part of Index entry to be a parent entry of string format. Received line |1|2|3".to_string(),
        };
        assert_eq!(parse_line("|1|2|3").unwrap_err(), expected);
    }
    #[test]
    fn test_parse_line_latest_offset_not_usize_returns_parse_error() {
        let expected = IndexError {
            kind: IndexErrorKind::Parse,
            message: "Expected second part of Index entry to be a positive number. Received line some_parent|s|2|3".to_string(),
        };
        assert_eq!(parse_line("some_parent|s|2|3").unwrap_err(), expected);
    }
    #[test]
    fn test_parse_line_length_not_usize_returns_parse_error() {
        let expected = IndexError {
            kind: IndexErrorKind::Parse,
            message: "Expected third part of Index entry to be a positive number. Received line some_parent|1|a|3".to_string(),
        };
        assert_eq!(parse_line("some_parent|1|a|3").unwrap_err(), expected);
    }
    #[test]
    fn test_parse_line_generation_not_usize_returns_parse_error() {
        let expected = IndexError {
            kind: IndexErrorKind::Parse,
            message: "Expected fourth part of Index entry to be a positive number. Received line some_parent|1|2|b".to_string(),
        };
        assert_eq!(parse_line("some_parent|1|2|b").unwrap_err(), expected);
    }
}

#[cfg(test)]
mod ds_index_tests_public_api {
    use super::*;
    use std::io::Cursor;

    use std::io::{self, Read, Seek, SeekFrom, Write};

    struct FailingWriter {
        inner: Cursor<Vec<u8>>,
    }
    impl FailingWriter {
        fn new(initial: Vec<u8>) -> Self {
            Self {
                inner: Cursor::new(initial),
            }
        }
    }
    impl Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "forced write failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "forced flush failure"))
        }
    }
    impl Seek for FailingWriter {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
            self.inner.seek(pos)
        }
    }
    impl Read for FailingWriter {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.inner.read(buf)
        }
    }
    struct FailingReader;
    impl Write for FailingReader {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    impl Seek for FailingReader {
        fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
            Ok(0) // allow seek so we reach the write path
        }
    }
    impl Read for FailingReader {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "forced read failure"))
        }
    }

    #[test]
    fn test_create_index_stores_an_index() {
        let mut idx = Index::new(Cursor::new(Vec::new()));
        idx.idx_create(vec![("some_parent".to_string(), 1, 2, 3)])
            .unwrap();
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "some_parent|1|2|3\n");
    }
    #[test]
    fn test_create_multiple_indexes() {
        let mut idx = Index::new(Cursor::new(Vec::new()));
        idx.idx_create(vec![
            ("another_parent".to_string(), 4, 5, 6),
            ("some_parent".to_string(), 1, 2, 3),
        ])
        .unwrap();
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "another_parent|4|5|6\nsome_parent|1|2|3\n");
    }
    #[test]
    fn test_create_index_of_existing_errors_already_exists() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n".to_vec()));
        let result = idx
            .idx_create(vec![("another_parent".to_string(), 7, 8, 9)])
            .unwrap_err();
        assert_eq!(
            result,
            IndexError {
                kind: IndexErrorKind::AlreadyExists,
                message: "The following indexes already exist and therefore were not added: [\"another_parent\"]".to_string()
            }
        );
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "another_parent|4|5|6\n");
    }
    #[test]
    fn test_create_indexes_partial_success_with_error_already_exists() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n".to_vec()));
        let result = idx
            .idx_create(vec![
                ("another_parent".to_string(), 3, 2, 1),
                ("some_parent".to_string(), 1, 2, 3),
            ])
            .unwrap_err();
        assert_eq!(
            result,
            IndexError {
                kind: IndexErrorKind::AlreadyExists,
                message: "The following indexes already exist and therefore were not added: [\"another_parent\"]".to_string()
            }
        );
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "another_parent|4|5|6\nsome_parent|1|2|3\n");
    }
    #[test]
    fn test_create_index_with_write_failure_returns_write_error() {
        let mut idx = Index::new(FailingWriter::new(vec![]));

        let err = idx
            .idx_create(vec![("some_parent".to_string(), 1, 2, 3)])
            .unwrap_err();

        assert_eq!(err.kind, IndexErrorKind::Write);
        assert!(
            err.message.contains("Failed to write line") || err.message.contains("Flush failed")
        );
    }
    #[test]
    fn test_read_index_returns_an_index() {
        let mut idx = Index::new(Cursor::new(b"some_parent|4|5|6\n".to_vec()));
        let result = idx.idx_read("some_parent").unwrap();
        assert_eq!(result, ("some_parent".to_string(), 4, 5, 6));
        // ensure no write operation happened
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "some_parent|4|5|6\n");
    }
    #[test]
    fn test_read_index_no_existing_returns_not_found_error() {
        let mut idx = Index::new(Cursor::new(b"some_parent|4|5|6\n".to_vec()));
        let result = idx.idx_read("another_parent").unwrap_err();
        assert_eq!(
            result,
            IndexError {
                kind: IndexErrorKind::NotFound,
                message: "The following index was not found: another_parent".to_string()
            }
        );
        // ensure no write operation happened
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "some_parent|4|5|6\n");
    }
    #[test]
    fn test_read_index_with_read_failure_returns_read_error() {
        let mut idx = Index::new(FailingReader);
        let err = idx.idx_read("some_parent").unwrap_err();
        assert_eq!(err.kind, IndexErrorKind::Read);
        assert!(err.message.contains("Line read failed"));
    }
    #[test]
    fn test_update_index_appends_to_end() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n".to_vec()));
        idx.idx_update(vec![("another_parent".to_string(), 1, 2, 3)])
            .unwrap();
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "another_parent|4|5|6\nanother_parent|1|2|3\n");
    }
    #[test]
    fn test_update_indexes_appends_to_end() {
        let mut idx = Index::new(Cursor::new(
            b"some_parent|1|2|3\nanother_parent|4|5|6\n".to_vec(),
        ));
        idx.idx_update(vec![
            ("another_parent".to_string(), 1, 2, 3),
            ("some_parent".to_string(), 4, 5, 6),
        ])
        .unwrap();
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(
            res,
            "some_parent|1|2|3\nanother_parent|4|5|6\nanother_parent|1|2|3\nsome_parent|4|5|6\n"
        );
    }
    #[test]
    fn test_update_index_no_existing_returns_not_found_error() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n".to_vec()));
        let response = idx
            .idx_update(vec![("unknown".to_string(), 1, 2, 3)])
            .unwrap_err();
        assert_eq!(
            response,
            IndexError {
                kind: IndexErrorKind::NotFound,
                message:
                    "The following indexes were not found and therefore not updated: [\"unknown\"]"
                        .to_string()
            }
        );
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "another_parent|4|5|6\n");
    }
    #[test]
    fn test_update_indexes_partial_success_returns_not_found_error() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n".to_vec()));
        let response = idx
            .idx_update(vec![
                ("another_parent".to_string(), 1, 2, 3),
                ("unknown".to_string(), 1, 2, 3),
            ])
            .unwrap_err();
        assert_eq!(
            response,
            IndexError {
                kind: IndexErrorKind::NotFound,
                message:
                    "The following indexes were not found and therefore not updated: [\"unknown\"]"
                        .to_string()
            }
        );
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "another_parent|4|5|6\nanother_parent|1|2|3\n");
    }
    #[test]
    fn test_update_index_with_read_failure_returns_read_error() {
        let mut idx = Index::new(FailingReader);
        let err = idx
            .idx_update(vec![("another_parent".to_string(), 1, 2, 3)])
            .unwrap_err();
        assert_eq!(err.kind, IndexErrorKind::Read);
        assert!(err.message.contains("Line read failed"));
    }
    #[test]
    fn test_update_index_with_write_failure_returns_write_error() {
        let mut idx = Index::new(FailingWriter::new(b"another_parent|1|2|3\n".to_vec()));
        let err = idx
            .idx_update(vec![("another_parent".to_string(), 1, 2, 3)])
            .unwrap_err();
        assert_eq!(err.kind, IndexErrorKind::Write);
        assert!(
            err.message.contains("Failed to write line") || err.message.contains("Flush failed")
        );
    }
    #[test]
    fn test_upsert_index_no_existing_appends_to_end() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n".to_vec()));
        idx.idx_upsert(vec![("new_parent".to_string(), 1, 2, 3)])
            .unwrap();
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "another_parent|4|5|6\nnew_parent|1|2|3\n");
    }
    #[test]
    fn test_upsert_index_existing_appends_to_end() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n".to_vec()));
        idx.idx_upsert(vec![("another_parent".to_string(), 1, 2, 3)])
            .unwrap();
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "another_parent|4|5|6\nanother_parent|1|2|3\n");
    }
    #[test]
    fn test_upsert_indexes_existing_and_new() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n".to_vec()));
        idx.idx_upsert(vec![
            ("some_parent".to_string(), 1, 2, 3),
            ("some_other_parent".to_string(), 10, 22, 31),
            ("another_parent".to_string(), 1, 2, 31),
        ])
        .unwrap();
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(
            res,
            "another_parent|4|5|6\nsome_parent|1|2|3\nsome_other_parent|10|22|31\nanother_parent|1|2|31\n"
        );
    }
    #[test]
    fn test_upsert_index_with_write_failure_returns_write_error() {
        let mut idx = Index::new(FailingWriter::new(b"".to_vec()));
        let err = idx
            .idx_upsert(vec![("p".to_string(), 1, 2, 3)])
            .unwrap_err();
        assert_eq!(err.kind, IndexErrorKind::Write);
        assert!(
            err.message.contains("Failed to write line") || err.message.contains("Flush failed")
        );
    }
    #[test]
    fn test_delete_index() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n".to_vec()));
        idx.idx_delete(vec!["another_parent"]).unwrap();
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "another_parent|4|5|6\n-another_parent\n");
    }
    #[test]
    fn test_delete_multiple_indexes() {
        let mut idx = Index::new(Cursor::new(
            b"another_parent|4|5|6\nsome_parent|1|2|3\n".to_vec(),
        ));
        idx.idx_delete(vec!["some_parent", "another_parent"])
            .unwrap();
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(
            res,
            "another_parent|4|5|6\nsome_parent|1|2|3\n-some_parent\n-another_parent\n"
        );
    }
    #[test]
    fn test_delete_index_no_existing_adds_tombstone() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n".to_vec()));
        idx.idx_delete(vec!["unknown"]).unwrap();
        let bytes = idx.buf.get_ref();
        let res = std::str::from_utf8(bytes).unwrap();
        assert_eq!(res, "another_parent|4|5|6\n-unknown\n");
    }
    #[test]
    fn test_delete_with_write_failure_returns_write_error() {
        let mut idx = Index::new(FailingWriter::new(b"".to_vec()));
        let err = idx.idx_delete(vec!["p"]).unwrap_err();
        assert_eq!(err.kind, IndexErrorKind::Write);
        assert!(
            err.message.contains("Failed to write line") || err.message.contains("Flush failed")
        );
    }
    #[test]
    fn test_idx_parents_returns_all_parents() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n-unknown\nsome_parent|1|2|3\nanother_parent|2|1|1\nunknown|2|2|2\n-some_parent\n".to_vec()));
        let parents = idx.idx_parents().unwrap();
        assert_eq!(parents.len(), 2);
        assert!(parents.contains("unknown"));
        assert!(parents.contains("another_parent"));
    }
    #[test]
    fn test_idx_parents_with_read_failure_returns_read_error() {
        let mut idx = Index::new(FailingReader);
        let err = idx.idx_parents().unwrap_err();
        assert_eq!(err.kind, IndexErrorKind::Read);
        assert!(err.message.contains("Line read failed"));
    }
    #[test]
    fn test_idx_parents_with_parse_failure_returns_parse_error() {
        let mut idx = Index::new(Cursor::new(b"another_parent|4|5|6\n-unknown\nsome_parent|1|2|3\nanother_parent|2|1|1\nunknown|1|2|b\n-some_parent\n".to_vec()));
        let expected = IndexError {
            kind: IndexErrorKind::Parse,
            message: "Expected fourth part of Index entry to be a positive number. Received line unknown|1|2|b".to_string(),
        };

        let err = idx.idx_parents().unwrap_err();
        assert_eq!(err, expected);
    }

    // TODO: add idx_compact tests here once implemented
}
