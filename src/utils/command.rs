use std::{env, fmt};

pub(crate) fn get_current_directory_name() -> Result<String, CommandUtilError> {
    let current_dir = env::current_dir().map_err(|e| CommandUtilError {
        kind: CommandUtilErrorKind::CurrentDirectoryNotFound,
        message: e.to_string(),
    })?;
    let current_dir_name = current_dir
        .file_name()
        .ok_or(CommandUtilError {
            kind: CommandUtilErrorKind::UnableToGetCurrentDirectoryName,
            message: "Unable to derive current directory name - ensure path is a directory and doesn't end with / or ..".to_string(),
        })?;
    let current_dir_name = current_dir_name.to_str().ok_or(CommandUtilError {
        kind: CommandUtilErrorKind::CastError,
        message: "Unable to cast current directory name to string".to_string(),
    })?;
    Ok(current_dir_name.to_string())
}

// Errors
#[derive(Debug, PartialEq)]
pub enum CommandUtilErrorKind {
    CastError,
    CurrentDirectoryNotFound,
    UnableToGetCurrentDirectoryName,
}

#[derive(Debug)]
pub struct CommandUtilError {
    kind: CommandUtilErrorKind,
    message: String,
}

impl fmt::Display for CommandUtilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (command util error: {})", self.message, self.kind)
    }
}

impl fmt::Display for CommandUtilErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandUtilErrorKind::CastError => write!(f, "Cast error"),
            CommandUtilErrorKind::CurrentDirectoryNotFound => {
                write!(f, "Current directory not found")
            }
            CommandUtilErrorKind::UnableToGetCurrentDirectoryName => {
                write!(f, "Unable to get current directory name")
            }
        }
    }
}
