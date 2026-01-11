use std::{
    env::{consts::OS, var},
    fmt,
    path::PathBuf,
    process::Command,
};

pub fn derive_data_local_dir_by_os(
    qualifier: &str,
    organization: &str,
    application: &str,
) -> Result<PathBuf, OsImplementationError> {
    match OS {
        "linux" => {
            // TODO: improvement to ensure absolute path (what happens if one of these is
            // relative?)
            if let Some(data_home) = var("XDG_DATA_HOME").ok() {
                return Ok(PathBuf::from(format!("{data_home}/{application}")));
            }
            if let Some(home) = var("HOME").ok() {
                return Ok(PathBuf::from(format!("{home}/.local/share/{application}")));
            }
            // TODO: could be derived with sys calls instead of forcing user to set ENV var
            Err(OsImplementationError {
                kind: OsImplementationErrorKind::MissingEnvVar,
                message: "Both environment variable $XDG_DATA_HOME and $HOME were not found"
                    .to_string(),
            })
        }
        "macos" => {
            if let Some(home) = var("HOME").ok() {
                return Ok(PathBuf::from(format!(
                    "{home}/Library/Application Support/{qualifier}.{organization}.{application}"
                )));
            }
            Err(OsImplementationError {
                kind: OsImplementationErrorKind::MissingEnvVar,
                message: "The environment variable $HOME was not found".to_string(),
            })
        }
        "windows" => {
            if let Some(local_app_data) = var("LOCALAPPDATA").ok() {
                return Ok(PathBuf::from(format!(
                    "{local_app_data}\\{organization}\\{application}\\data"
                )));
            }
            Err(OsImplementationError {
                kind: OsImplementationErrorKind::MissingEnvVar,
                message: "The environment variable %LOCALAPPDATA% was not found".to_string(),
            })
        }
        os => Err(OsImplementationError {
            kind: OsImplementationErrorKind::OsNotSupported,
            message: format!("Unsupported OS: {}", os),
        }),
    }
}

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
        "windows" => Command::new("start")
            .arg("\"\"")
            .arg(format!("\"{link}\""))
            .spawn()
            .map_err(|e| OsImplementationError {
                kind: OsImplementationErrorKind::CommandFailedToStart,
                message: format!("Failed to start command start: {e}"),
            })?,
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
    MissingEnvVar,
    OsNotSupported,
}

#[derive(Debug)]
pub struct OsImplementationError {
    kind: OsImplementationErrorKind,
    message: String,
}

impl fmt::Display for OsImplementationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (util error: {})", self.message, self.kind)
    }
}

impl fmt::Display for OsImplementationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OsImplementationErrorKind::CommandFailedToStart => write!(f, "Command failed to start"),
            OsImplementationErrorKind::CommandNotRunning => write!(f, "Command not running"),
            OsImplementationErrorKind::MissingEnvVar => {
                write!(f, "Required environment variable missing")
            }
            OsImplementationErrorKind::OsNotSupported => write!(f, "OS not supported"),
        }
    }
}
