use std::fmt;

#[derive(Debug, PartialEq)]
pub(super) enum Shell {
    Zsh,
    NotSupported,
}

pub(super) fn determine_user_shell() -> Result<Shell, InitError> {
    // TODO: this will not work on windows
    let shell = std::env::var("SHELL").map_err(|e| InitError {
        kind: InitErrorKind::UnableToDetermineUserShell,
        message: e.to_string(),
    })?;

    match shell.as_str() {
        "/bin/zsh" => Ok(Shell::Zsh),
        _ => Ok(Shell::NotSupported),
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum InitErrorKind {
    ReadFailed,
    WriteFailed,
    UnableToDetermineUserShell,
}

#[derive(Debug)]
pub(super) struct InitError {
    kind: InitErrorKind,
    message: String,
}

impl InitError {
    pub(super) fn new(kind: InitErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (init error: {})", self.message, self.kind)
    }
}

impl fmt::Display for InitErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InitErrorKind::ReadFailed => write!(f, "Read failed"),
            InitErrorKind::WriteFailed => write!(f, "Write failed"),
            InitErrorKind::UnableToDetermineUserShell => {
                write!(f, "Unable to determine user shell")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_determine_user_shell_zsh() {
        let mut env_vars = std::collections::HashMap::new();
        env_vars.insert("SHELL", "/bin/zsh");
        assert_eq!(determine_user_shell().unwrap(), Shell::Zsh)
    }

    #[test]
    #[ignore]
    fn test_determine_user_shell_unsupported_shell() {
        let mut env_vars = std::collections::HashMap::new();
        env_vars.insert("SHELL", "/bin/sh");
        assert_eq!(determine_user_shell().unwrap(), Shell::NotSupported)
    }
}
