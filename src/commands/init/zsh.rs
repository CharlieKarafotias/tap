use super::shell_completions::ZSH_COMPLETION;
use super::utils::{InitError, InitErrorKind};
use std::path::PathBuf;
use std::{
    env,
    fs::{self, File, create_dir_all},
    io::Write,
};

/// Returns the path to the user's home directory
fn home_path() -> Result<PathBuf, InitError> {
    Ok(PathBuf::from(env::var("HOME").map_err(|e| {
        InitError::new(
            InitErrorKind::ReadFailed,
            format!("Unable to determine home directory: {e}"),
        )
    })?))
}

/// Returns ~/.zshrc
fn zshrc_path() -> Result<PathBuf, InitError> {
    Ok(home_path()?.join(".zshrc"))
}

/// Returns ~/.zsh/
fn completions_path() -> Result<PathBuf, InitError> {
    Ok(home_path()?.join(".zsh"))
}

/// Creates ~/.zshrc if it doesn't exist.
///
/// # Errors
/// - If the file cannot be created, an InitError of kind WriteFailed will be returned
fn create_zshrc_if_not_exists() -> Result<(), InitError> {
    let zshrc_path = zshrc_path()?;
    let res = File::create_new(zshrc_path);
    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                return Ok(());
            }
            Err(InitError::new(
                InitErrorKind::WriteFailed,
                format!("Unable to create ~/.zshrc: {e}"),
            ))
        }
    }
}

/// Returns true if fpath contains $HOME/.zsh/completions
///
/// # Errors
/// - If an error occurs while determining if completions folder exists, an InitError of kind ReadFailed will be returned
fn fpath_has_zsh_completions() -> Result<bool, InitError> {
    let zshrc_path = zshrc_path()?;
    let zshrc_contents = fs::read_to_string(&zshrc_path).map_err(|e| {
        InitError::new(
            InitErrorKind::ReadFailed,
            format!("Unable to read ~/.zshrc: {e}"),
        )
    })?;
    let completions_path = completions_path()?;
    Ok(zshrc_contents.contains(&format!("fpath=({} $fpath)", completions_path.display())))
}

/// Adds the following to the top of ~/.zshrc (if it doesn't already exist):
/// fpath+=($home/.zsh)
/// autoload -Uz compinit && compinit
///
/// _This function assumes that `create_zshrc_if_not_exists` has already been called_
///
/// # Errors
/// - If the ~/.zshrc file cannot be written, an InitError of kind WriteFailed will be returned
/// - If the ~/.zshrc file cannot be read, an InitError of kind ReadFailed will be returned
fn add_fpath_and_autocompletions_if_not_exists() -> Result<(), InitError> {
    let zshrc_path = zshrc_path()?;
    let completions_path = completions_path()?;
    let fpath = format!("fpath=({} $fpath)", completions_path.display());
    let autoload_compinit = "autoload -Uz compinit\ncompinit";

    let mut f = fs::read_to_string(&zshrc_path)
        .map_err(|e| InitError::new(InitErrorKind::ReadFailed, e.to_string()))?;
    let contains_fpath = fpath_has_zsh_completions()?;
    let contains_autoload_compinit =
        f.contains(autoload_compinit) || f.contains("autoload -Uz compinit && compinit");

    if !contains_autoload_compinit {
        f = format!("{autoload_compinit}\n{f}");
    }

    if !contains_fpath {
        f = format!("{fpath}\n{f}");
    }
    fs::write(zshrc_path, f)
        .map_err(|e| InitError::new(InitErrorKind::WriteFailed, e.to_string()))?;
    Ok(())
}

/// Writes the contents of ZSH_COMPLETION to ~/.zsh/_tap
///
/// # Errors
/// - If the directories for site-functions cannot be created, an InitError of kind WriteFailed will be returned
/// - If the file cannot be written, an InitError of kind WriteFailed will be returned
/// - If the file cannot be made executable, an InitError of kind WriteFailed will be returned
fn add_completions_to_site_functions() -> Result<(), InitError> {
    let p = completions_path()?;
    create_dir_all(&p).map_err(|e| {
        InitError::new(
            InitErrorKind::WriteFailed,
            format!(
                "Could not create directories for path {}: {e} - TIP: run as root user",
                p.display()
            ),
        )
    })?;
    let mut f = File::create(p.join("_tap")).map_err(|e| {
        InitError::new(
            InitErrorKind::ReadFailed,
            format!("Failed to create or open existing _tap completion file: {e}"),
        )
    })?;
    f.write_all(ZSH_COMPLETION.as_ref()).map_err(|e| {
        InitError::new(
            InitErrorKind::WriteFailed,
            format!("Failed to write _tap completion file: {e}"),
        )
    })?;
    Ok(())
}

/// Updates ~/.zshrc to include tap completions
pub(super) fn update_zshrc() -> Result<(), InitError> {
    create_zshrc_if_not_exists()?;
    add_fpath_and_autocompletions_if_not_exists()?;
    add_completions_to_site_functions()?;
    Ok(())
}
