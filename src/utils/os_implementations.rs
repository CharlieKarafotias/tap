use std::{env::consts::OS, fmt, process::Command};

pub fn open_link(link: &str) -> Result<(), OsImplementationError> {
    let mut cmd = match OS {
        "macos" => Command::new("open")
            .arg(link)
            .spawn()
            .map_err(|e| OsImplementationError {
                kind: OsImplementationErrorKind::CommandFailedToStart,
                message: format!("Failed to start command open: {e}"),
            })?,
        "linux" => {
            Command::new("xdg-open")
                .arg(link)
                .spawn()
                .map_err(|e| OsImplementationError {
                    kind: OsImplementationErrorKind::CommandFailedToStart,
                    message: format!("Failed to start command xdg-open: {e}"),
                })?
        }
        // TODO: implement "windows" => (),
        _ => {
            return Err(OsImplementationError {
                kind: OsImplementationErrorKind::OsNotSupported,
                message: format!("Unsupported OS: {}", OS),
            });
        }
    };
    cmd.wait().map_err(|e| OsImplementationError {
        kind: OsImplementationErrorKind::CommandNotRunning,
        message: format!("No exit status from open command: {e}"),
    })?;
    Ok(())
}

// Errors
#[derive(Debug, PartialEq)]
pub enum OsImplementationErrorKind {
    CommandFailedToStart,
    CommandNotRunning,
    OsNotSupported,
}

#[derive(Debug)]
pub struct OsImplementationError {
    kind: OsImplementationErrorKind,
    message: String,
}

impl fmt::Display for OsImplementationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (command util error: {})", self.message, self.kind)
    }
}

impl fmt::Display for OsImplementationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OsImplementationErrorKind::CommandFailedToStart => write!(f, "Command failed to start"),
            OsImplementationErrorKind::CommandNotRunning => write!(f, "Command not running"),
            OsImplementationErrorKind::OsNotSupported => write!(f, "OS not supported"),
        }
    }
}
