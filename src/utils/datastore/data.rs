use super::index::Idx;
use std::{
    fmt,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
};

/// A Data entry
/// Format: parent_entity, link_name, link_value
pub(super) type D<'a> = (&'a str, &'a str, &'a str);

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
        todo!()
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
///    - Corrupt: the data segment provided is not UTF-8 format
///    - Parse: the current data segment is not parsable into a data type format
fn parse_data_segment<'a>(text: &'a str) -> Result<Vec<D<'a>>, DataError> {
    let mut lines = text.lines();

    let header = lines.next().ok_or(DataError {
        kind: DataErrorKind::Parse,
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

fn parse_data_line<'a>(parent: &'a str, line: &'a str) -> Result<D<'a>, DataError> {
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

    Ok((parent, link, value))
}

/// A helper function to write type D as a String
///
/// NOTE: Supports types of D where it is empty or contains link/value. If empty, it will return a
/// tombstone
fn write_d_as_string<'a>(
    parent: &str,
    d: impl IntoIterator<Item = (&'a str, &'a str, &'a str)>,
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
            out.push_str(l);
            out.push('|');
            out.push_str(v);
            out.push('\n');
        }
    }
    out
}

// Errors
#[derive(Debug, PartialEq)]
enum DataErrorKind {
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
