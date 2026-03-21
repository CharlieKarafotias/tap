mod shell_completions;
mod utils;
mod zsh;

use crate::{
    commands::{Command, CommandInfo, CommandResult},
    utils::{cli_usage_table::DisplayCommandAsRow, datastore::Datastore},
};
use std::io::{Read, Seek, Write};
use utils::{Shell, determine_user_shell};
use zsh::update_zshrc;

pub(crate) struct Init {
    name: String,
    description: String,
    args: [String; 1],
}

impl Default for Init {
    fn default() -> Self {
        Self {
            name: "-i, --init".to_string(),
            description: "Setup Tap and shell completions".to_string(),
            args: ["<auto|zsh>".to_string()],
        }
    }
}

impl CommandInfo for Init {
    fn error_message(&self) -> String {
        "invalid arguments, see the Usage section with tap --init --help".to_string()
    }

    fn help_message(&self) -> String {
        let mut s = String::new();
        s.push_str("Initialize Tap Auto-Completion\n\n");
        s.push_str("Tap currently supports auto-completion for zsh.\n");
        s.push_str("Installation: \n\n");
        s.push_str("  - To automatically setup shell completions, run tap --init auto\n");
        s.push_str("  - For manual setup:\n");
        s.push_str("      1. Make a .zsh directory: mkdir -p ~/.zsh/\n");
        s.push_str(
            "      2. Save the completion file to ~/.zsh/_tap: tap --init zsh > ~/.zsh/_tap\n",
        );
        s.push_str("      3. Add the following line to your .zshrc file: fpath=(~/.zsh/ $fpath)\n");
        s.push_str("      4. Add the following line to your .zshrc file after fpath line: autoload -Uz compinit && compinit\n\n");
        s.push_str("Command Structure: tap --init <auto | zsh>\n\n");
        s.push_str("Example Usage: \n");
        s.push_str("  - Return zsh shell completion: tap --init zsh\n");
        s.push_str(
            "  - Automatically setup shell completion based on current shell: tap --init auto",
        );
        s
    }
}

impl<RW: Read + Write + Seek> Command<RW> for Init {
    fn run<I: Iterator<Item = String>>(
        &self,
        mut args: I,
        _ds: Datastore<RW>,
    ) -> Result<CommandResult, String> {
        let arg1 = args.next();

        match arg1.as_deref() {
            Some("zsh") => Ok(CommandResult::Value(
                shell_completions::ZSH_COMPLETION.to_string(),
            )),
            Some("auto") => {
                match determine_user_shell() {
                    Ok(Shell::Zsh) => update_zshrc().map_err(|e| e.to_string()),
                    Ok(Shell::NotSupported) => Err(
                        "tap does not support your shell, please use zsh for shell completions"
                            .to_string(),
                    ),
                    Err(e) => Err(e.to_string()),
                }?;
                Ok(CommandResult::Value(
                    "Updated shell completions, restart your shell for changes to take effect"
                        .to_string(),
                ))
            }
            Some("--help") => Ok(CommandResult::Value(self.help_message())),
            _ => Err(self.error_message()),
        }
    }
}

impl DisplayCommandAsRow for Init {
    fn args(&self) -> Vec<String> {
        self.args.to_vec()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_init_run_expected_help_arg() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["--help".to_string()].into_iter();
        let cmd = Init::default();
        let expected: Result<CommandResult, String> = Ok(CommandResult::Value(cmd.help_message()));
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_init_run_unexpected_args() {
        let ds = Datastore::new_in_memory(Cursor::new(vec![]), Cursor::new(vec![]));
        let args = vec!["random".to_string()].into_iter();
        let cmd = Init::default();
        let expected: Result<CommandResult, String> = Err(cmd.error_message());
        let res = cmd.run(args, ds);
        assert_eq!(res, expected);
    }
}
