use super::index::Idx;
use std::{
    fmt,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
};

/// A Data entry
/// Format: parent_entity, link_name, link_value
pub(super) type D = (String, String, String);

pub(super) enum DeleteTarget<'a> {
    Parent {
        parent_entity: &'a str,
    },
    Link {
        idx: &'a Idx,
        parent_entity: &'a str,
        link: &'a str,
    },
}

/// Build & maintains an append-only Data structure
///
/// Purpose:
///     - A structure that stores line data entries in the following formats:
///         - `parent_entity->\nlink1|value1\nlink2|value2\n` (a data entry with 2 links)
///         - `-parent_entity` (a removed data entry)
///     - Enables faster writes of data utilizing append-only structure
///     - Uses Index struct's `latest_byte_offset` and `length` to
///       Seek only the bytes required to search over for parent->link relationship
///
/// Supported operations:
///     - Read data (all elements, or individual link/value)
///     - Create/Update data entries
///     - Create/Upsert data entries
///     - Delete data entries
///     - Compact data representation
pub(super) struct Data<RW: Read + Write + Seek> {
    buf: RW,
}

impl<RW: Read + Write + Seek> Data<RW> {
    /// Creates a new `Data` representation. A Data representation is any type which implements the
    /// Read, Seek, and Write traits.
    ///
    /// In Production, a file is used under the name `tap_data.txt` to store the index data. The index file
    /// is stored in the proper platform-specific, user-accessible location:
    ///
    /// Linux: `$XDG_DATA_HOME/Tap/tap_index.txt` OR `$HOME/.local/share/Tap/tap_data.txt`
    /// macOS: `$HOME/Library/Application Support/dev.CharlieKarafotias.Tap/tap_data.txt`
    /// Windows: `{FOLDERID_LocalAppData}\CharlieKarafotias\Tap\data\tap_data.txt`
    pub fn new(buf: RW) -> Self {
        Data { buf }
    }
    /// Reads a data entry from the Data file.
    ///
    /// Supports reading all link/values under a parent_entry or reading a specific link
    ///
    /// Errors:
    ///     - `Corrupt`: the index structure provided a buf location that doesn't match the expected
    ///        parent_entity
    ///     - `Read`: the read operation of the Data structure failed
    ///     - `NotFound`: the link was not found in the Data structure
    pub fn data_read(
        &mut self,
        idx: &Idx,
        parent_entity: &str,
        link: Option<&str>,
    ) -> Result<Vec<D>, DataError> {
        let (parent, byte_offset, len_bytes_to_read, _generation) = idx;
        if parent != parent_entity {
            return Err(DataError {
                kind: DataErrorKind::Corrupt,
                message: "Index parent does not match provided parent_entity".into(),
            });
        }

        let mut buf = vec![0u8; *len_bytes_to_read];
        let text = {
            let mut reader = BufReader::new(&mut self.buf);
            reader
                .seek(SeekFrom::Start(*byte_offset as u64))
                .map_err(|e| DataError {
                    kind: DataErrorKind::Corrupt,
                    message: format!("Failed to seek to data segment: {e}"),
                })?;

            reader.read_exact(&mut buf).map_err(|e| DataError {
                kind: DataErrorKind::Read,
                message: format!("Failed to read data segment: {e}"),
            })?;

            String::from_utf8(buf).map_err(|e| DataError {
                kind: DataErrorKind::Corrupt,
                message: format!("Data segment is not valid UTF-8: {e}"),
            })?
        };

        let mut data = parse_data_segment(&text)?;

        if let Some(link) = link {
            data.retain(|(_, l, _)| *l == link);
        }
        Ok(data)
    }
    /// Updates Data entries in the Data structure by appending them to the end of the
    /// structure. This operation should be used when the caller wants to ensure updates are only
    /// made to existing link/value pairs of a particular parent_entity.
    ///
    /// NOTE: the implementation to update data entries is slower than data_upsert as the
    /// parent_entity's links are checked prior to the append to the data structure. USE
    /// data_upsert to skip the checks IF overwriting adding/existing links is not an issue.
    ///
    /// Errors:
    ///     - `Corrupt`: the index structure provided a buf location that doesn't match the
    ///        expected parent_entity
    ///     - `NotFound`: the data entry(ies) specified was not found
    ///     - `Read`: a read operation to the Data structure failed
    ///     - `Write`: a write operation to the Data structure failed
    pub fn data_update(
        &mut self,
        idx: &Idx,
        parent_entity: &str,
        updates: Vec<D>,
    ) -> Result<Idx, DataError> {
        // NOTE: should return new buf start pos + size so Idx can be updated (return updated Idx)
        todo!()
    }
    /// Creates/Upserts Data entries in the Data structure by appending them to the end of the
    /// structure. An Idx is optional as its possible the parent_entity does not exist in the Data
    /// structure yet.
    ///
    /// Returns: An new/updated Idx with the new byte offset, length, and generation for the provided
    /// parent_entity when a successful create/upsert occurs
    ///
    /// Errors:
    ///     - `Corrupt`: the index structure provided a buf location that doesn't match the
    ///        expected parent_entity
    ///     - `Read`: a read operation to the Data structure failed
    ///     - `Write`: a write operation to the Data structure failed
    pub fn data_upsert(
        &mut self,
        idx: Option<&Idx>,
        parent_entity: &str,
        updates: Vec<D>,
    ) -> Result<Idx, DataError> {
        // NOTE: should return new buf start pos + size so Idx can be updated (return updated Idx)
        todo!()
    }
    /// Delete Data entries in the Data structure by appending a tombstone (in case of
    /// parent_entity delete) or appending data without the link (IF provided).
    ///
    /// NOTE: If a idx is provided, it is assumed a link will also be provided (as the 2 are
    /// dependent on each other)
    ///
    /// NOTE: providing a link to delete is slower than deleting the whole parent_entity as the
    /// data structure must be parsed to determine if the provided link must be removed on the
    /// append operation to end of data structure.
    ///
    /// Returns:
    ///     - A optional Idx with the new byte offset, length, and generation when a specific
    ///       link is deleted from the parent_entity successfully.
    ///     - No Idx is returned when the entire parent_entity is removed as both Index and Data
    ///       structures use a tombstone to represent this (no Index is maintained).
    /// Errors:
    ///     - `Corrupt`: the index structure provided a buf location that doesn't match the
    ///        expected parent_entity
    ///     - `NotFound`: the link (if specified) was not found in the Data structure
    ///     - `Parse`: the data segment read from the Data structure failed to parse
    ///     - `Read`: a read operation to the Data structure failed
    ///     - `Write`: a write operation to the Data structure failed
    pub fn data_delete(&mut self, delete: DeleteTarget<'_>) -> Result<Option<Idx>, DataError> {
        let (line_to_write, next_idx): (String, Option<Idx>) = match delete {
            DeleteTarget::Link {
                idx,
                parent_entity,
                link,
            } => {
                let (parent, byte_offset, len_bytes_to_read, generation) = idx;

                if parent != parent_entity {
                    return Err(DataError {
                        kind: DataErrorKind::Corrupt,
                        message: "Index parent does not match provided parent_entity".into(),
                    });
                }

                let mut buf = vec![0u8; *len_bytes_to_read];
                let text = {
                    let mut reader = BufReader::new(&mut self.buf);
                    reader
                        .seek(SeekFrom::Start(*byte_offset as u64))
                        .map_err(|e| DataError {
                            kind: DataErrorKind::Corrupt,
                            message: format!("Failed to seek to data segment: {e}"),
                        })?;

                    reader.read_exact(&mut buf).map_err(|e| DataError {
                        kind: DataErrorKind::Read,
                        message: format!("Failed to read data segment: {e}"),
                    })?;

                    std::str::from_utf8(&buf).map_err(|e| DataError {
                        kind: DataErrorKind::Corrupt,
                        message: format!("Data segment is not valid UTF-8: {e}"),
                    })?
                };

                let mut data = parse_data_segment(&text)?;
                let original_len = data.len();
                data.retain(|(_, l, _)| *l != link);

                if data.len() == original_len {
                    return Err(DataError {
                        kind: DataErrorKind::NotFound,
                        message: format!("Link '{link}' not found under '{parent_entity}'"),
                    });
                }
                let body = write_d_as_string(parent_entity, data);

                (
                    body,
                    Some((
                        parent_entity.into(),
                        0, // placeholder, filled after write
                        0, // placeholder, filled after write
                        generation + 1,
                    )),
                )
            }
            DeleteTarget::Parent { parent_entity } => (write_d_as_string(parent_entity, []), None),
        };

        let (latest_byte_offset, byte_len) = append_line(&mut self.buf, &line_to_write)?;
        if let Some(idx) = next_idx {
            Ok(Some((idx.0, latest_byte_offset, byte_len, idx.3)))
        } else {
            Ok(None)
        }
    }
    /// Compacts the Data structure by retaining only the data from the latest parent_entities
    ///
    /// Errors:
    ///     - TODO
    pub fn data_compact() -> () {
        todo!()
    }
}

/// Appends the provided string slice to the end of the given buffer.
///
/// This function seeks to the end of the underlying stream, writes the
/// UTF-8 bytes of `line`, flushes the writer, and returns the byte offset
/// at which the write began along with the number of bytes written.
///
/// # Type Parameters
///
/// * `RW` - Any type implementing [`Read`], [`Write`], and [`Seek`].
///
/// # Arguments
///
/// * `buf` - A mutable reference to a readable, writable, and seekable stream.
/// * `line` - The string slice to append.
///
/// # Returns
///
/// Returns `Ok((offset, len))` where:
///
/// * `offset` is the byte position in the stream where the write started.
/// * `len` is the number of bytes written (i.e., `line.as_bytes().len()`).
///
/// Returns `Err(DataError)` if seeking, writing, flushing, or determining
/// the stream position fails.
///
/// # Notes
///
/// - The function does not automatically append a newline.
/// - The caller is responsible for ensuring any required line delimiters.
/// - The returned offset can be used for indexing or later random access.
///
/// # Errors
///    - Write: All I/O failures are mapped to `DataErrorKind::Write`.
fn append_line<RW: Read + Write + Seek>(
    buf: &mut RW,
    line: &str,
) -> Result<(usize, usize), DataError> {
    let mut writer = BufWriter::new(buf);

    writer.seek(SeekFrom::End(0)).map_err(|e| DataError {
        kind: DataErrorKind::Write,
        message: format!("Failed to seek to end: {e}"),
    })?;

    let offset = writer.stream_position().map_err(|_| DataError {
        kind: DataErrorKind::Write,
        message: "Unable to determine data offset".into(),
    })? as usize;
    let bytes = line.as_bytes();

    writer.write_all(bytes).map_err(|e| DataError {
        kind: DataErrorKind::Write,
        message: format!("Failed to write data: {e}"),
    })?;

    writer.flush().map_err(|e| DataError {
        kind: DataErrorKind::Write,
        message: format!("Flush failed: {e}"),
    })?;

    Ok((offset, bytes.len()))
}

/// Parses a data segment from buf of bytes into the D format (data type format)
///
/// Errors:
///    - Corrupt: the current data segment provided by Index was not the expected format
///    - Parse: the current data segment is not parsable into a data type format
fn parse_data_segment(text: &str) -> Result<Vec<D>, DataError> {
    let mut lines = text.lines();

    let header = lines.next().ok_or(DataError {
        kind: DataErrorKind::Corrupt,
        message: "Missing data segment header".into(),
    })?;

    let parent_entity = header
        .strip_suffix("->")
        .filter(|s| !s.trim().is_empty())
        .ok_or(DataError {
            kind: DataErrorKind::Parse,
            message: "Invalid header, expected format: parent_entity->".into(),
        })?
        .trim();

    let data: Vec<D> = lines
        .map(|line| parse_data_line(&parent_entity, line))
        .collect::<Result<_, _>>()?;

    if data.is_empty() {
        return Err(DataError {
            kind: DataErrorKind::Parse,
            message: "Data segment contains no link|value entries".into(),
        });
    }
    Ok(data)
}

/// A helper function that parses a data line into type D
///
/// Errors:
///    - Parse: the line segment provided was invalid (missing '|' separator or missing link or
///    value
fn parse_data_line(parent: &str, line: &str) -> Result<D, DataError> {
    let (link, value) = line.split_once('|').ok_or_else(|| DataError {
        kind: DataErrorKind::Parse,
        message: format!("Invalid data line, expected link|value: {line}"),
    })?;

    let link = link.trim();
    let value = value.trim();

    if link.is_empty() || value.is_empty() {
        return Err(DataError {
            kind: DataErrorKind::Parse,
            message: format!("Empty link or value in line: {line}"),
        });
    }

    Ok((parent.into(), link.into(), value.into()))
}

/// A helper function to write type D as a String
///
/// NOTE: Supports types of D where it is empty or contains link/value. If empty, it will return a
/// tombstone
fn write_d_as_string(
    parent: &str,
    d: impl IntoIterator<Item = (String, String, String)>,
) -> String {
    let mut iter = d.into_iter().peekable();
    let mut out = String::new();

    if iter.peek().is_none() {
        out.push('-');
        out.push_str(parent);
        out.push('\n');
    } else {
        out.push_str(parent);
        out.push_str("->\n");
        for (_, l, v) in iter {
            out.push_str(&l);
            out.push('|');
            out.push_str(&v);
            out.push('\n');
        }
    }
    out
}

// Errors
#[derive(Debug, PartialEq)]
pub(super) enum DataErrorKind {
    AlreadyExists,
    Corrupt,
    NotFound,
    Parse,
    Read,
    Write,
}

#[derive(Debug, PartialEq)]
pub(super) struct DataError {
    pub(super) kind: DataErrorKind,
    pub(super) message: String,
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (data error: {})", self.message, self.kind)
    }
}

impl fmt::Display for DataErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataErrorKind::AlreadyExists => write!(f, "Data already exists"),
            DataErrorKind::Corrupt => {
                write!(f, "Data & Index files drifted - Data file corrupted")
            }
            DataErrorKind::NotFound => write!(f, "Data not found"),
            DataErrorKind::Read => write!(f, "Data read failed"),
            DataErrorKind::Parse => write!(f, "Data file structure corrupted"),
            DataErrorKind::Write => write!(f, "Data write failed"),
        }
    }
}

#[cfg(test)]
mod ds_data_tests_private_api {
    use super::*;
    use std::io::{self, Cursor, Read, Seek, SeekFrom};
    struct FailingWriter;

    impl Read for FailingWriter {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Ok(0)
        }
    }

    impl Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write failed"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Seek for FailingWriter {
        fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
            Ok(0)
        }
    }

    #[test]
    fn test_write_d_as_string_tombstone() {
        let expected = "-some_parent\n";
        assert_eq!(write_d_as_string("some_parent", []), expected);
    }
    #[test]
    fn test_write_d_as_string_new_data() {
        let expected = "some_parent->\nsome_key|some_value\n";
        assert_eq!(
            write_d_as_string(
                "some_parent",
                [(
                    "some_parent".to_string(),
                    "some_key".to_string(),
                    "some_value".to_string()
                )]
            ),
            expected
        );
    }
    #[test]
    fn test_parse_data_line_old_format() {
        let input = "  some_key|some_value";
        assert_eq!(
            parse_data_line("some_parent", input),
            Ok((
                "some_parent".to_string(),
                "some_key".to_string(),
                "some_value".to_string()
            ))
        )
    }
    #[test]
    fn test_parse_data_line_new_format() {
        let input = "some_key|some_value";
        assert_eq!(
            parse_data_line("some_parent", input),
            Ok((
                "some_parent".to_string(),
                "some_key".to_string(),
                "some_value".to_string()
            ))
        )
    }
    #[test]
    fn test_parse_data_line_err_missing_separator() {
        let input = "some_keysome_value";
        let err = parse_data_line("some_parent", input);
        assert_eq!(
            err,
            Err(DataError {
                kind: DataErrorKind::Parse,
                message: "Invalid data line, expected link|value: some_keysome_value".to_string()
            })
        )
    }
    #[test]
    fn test_parse_data_line_err_missing_key() {
        let input = "|some_value";
        let err = parse_data_line("some_parent", input);
        assert_eq!(
            err,
            Err(DataError {
                kind: DataErrorKind::Parse,
                message: "Empty link or value in line: |some_value".to_string()
            })
        )
    }
    #[test]
    fn test_parse_data_line_err_missing_value() {
        let input = "some_key|";
        let err = parse_data_line("some_parent", input);
        assert_eq!(
            err,
            Err(DataError {
                kind: DataErrorKind::Parse,
                message: "Empty link or value in line: some_key|".to_string()
            })
        )
    }
    #[test]
    fn test_parse_data_segment_old_format() {
        let input = "searchEngines->\n  google|www.google.com\n  bing|www.bing.com\n";
        let res = parse_data_segment(input);
        assert_eq!(
            res,
            Ok(vec![
                (
                    "searchEngines".to_string(),
                    "google".to_string(),
                    "www.google.com".to_string()
                ),
                (
                    "searchEngines".to_string(),
                    "bing".to_string(),
                    "www.bing.com".to_string()
                )
            ])
        );
    }
    #[test]
    fn test_parse_data_segment_new_format() {
        let input = "searchEngines->\ngoogle|www.google.com\nbing|www.bing.com\n";
        let res = parse_data_segment(input);
        assert_eq!(
            res,
            Ok(vec![
                (
                    "searchEngines".to_string(),
                    "google".to_string(),
                    "www.google.com".to_string()
                ),
                (
                    "searchEngines".to_string(),
                    "bing".to_string(),
                    "www.bing.com".to_string()
                )
            ])
        );
    }
    #[test]
    fn test_parse_data_segment_err_empty() {
        let input = "";
        let res = parse_data_segment(input);
        assert_eq!(
            res,
            Err(DataError {
                kind: DataErrorKind::Corrupt,
                message: "Missing data segment header".to_string()
            })
        );
    }
    #[test]
    fn test_parse_data_segment_err_invalid_header() {
        let input = "some_parent\ngoogle|www.google.com\n";
        let res = parse_data_segment(input);
        assert_eq!(
            res,
            Err(DataError {
                kind: DataErrorKind::Parse,
                message: "Invalid header, expected format: parent_entity->".to_string()
            })
        );
    }
    #[test]
    fn test_parse_data_segment_err_no_data() {
        let input = "some_parent->\n";
        let res = parse_data_segment(input);
        assert_eq!(
            res,
            Err(DataError {
                kind: DataErrorKind::Parse,
                message: "Data segment contains no link|value entries".to_string()
            })
        );
    }
    #[test]
    fn test_append_line_to_empty_buffer() {
        let mut cursor = Cursor::new(Vec::new());

        let (offset, len) = append_line(&mut cursor, "hello").unwrap();
        assert_eq!(offset, 0);
        assert_eq!(len, 5);

        cursor.seek(SeekFrom::Start(0)).unwrap();
        let mut contents = String::new();
        cursor.read_to_string(&mut contents).unwrap();

        assert_eq!(contents, "hello");
    }
    #[test]
    fn test_append_line_to_existing_data() {
        let mut cursor = Cursor::new(b"existing".to_vec());

        let (offset, len) = append_line(&mut cursor, "data").unwrap();

        assert_eq!(offset, 8);
        assert_eq!(len, 4);

        cursor.seek(SeekFrom::Start(0)).unwrap();
        let mut contents = String::new();
        cursor.read_to_string(&mut contents).unwrap();

        assert_eq!(contents, "existingdata");
    }
    #[test]
    fn test_append_line_multiple_times() {
        let mut cursor = Cursor::new(Vec::new());

        let (offset1, len1) = append_line(&mut cursor, "abc").unwrap();
        let (offset2, len2) = append_line(&mut cursor, "def").unwrap();

        assert_eq!(offset1, 0);
        assert_eq!(len1, 3);

        assert_eq!(offset2, 3);
        assert_eq!(len2, 3);

        cursor.seek(SeekFrom::Start(0)).unwrap();
        let mut contents = String::new();
        cursor.read_to_string(&mut contents).unwrap();

        assert_eq!(contents, "abcdef");
    }
    #[test]
    fn test_append_line_empty_string() {
        let mut cursor = Cursor::new(Vec::new());

        let (offset, len) = append_line(&mut cursor, "").unwrap();

        assert_eq!(offset, 0);
        assert_eq!(len, 0);

        cursor.seek(SeekFrom::Start(0)).unwrap();
        let mut contents = String::new();
        cursor.read_to_string(&mut contents).unwrap();

        assert_eq!(contents, "");
    }
    #[test]
    fn append_line_write_failure_returns_error() {
        let mut writer = FailingWriter;

        let err = append_line(&mut writer, "data").unwrap_err();

        assert_eq!(err.kind, DataErrorKind::Write);
    }
}

#[cfg(test)]
mod ds_data_tests_public_api {
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

    // TODO: implement these
    #[test]
    fn test_data_read_() {}
    #[test]
    fn test_data_update_() {}
    #[test]
    fn test_data_upsert_() {}
    #[test]
    fn test_data_delete_() {}
    #[test]
    fn test_data_compact_() {}
}
