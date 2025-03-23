use std::env;

/// Parses command-line arguments, skipping the first argument (the program name).
///
/// # Returns
///
/// A vector of strings containing the command-line arguments provided after the program name.
pub fn parse_args() -> Vec<String> {
    env::args().skip(1).collect()
}

pub fn run(args: Vec<String>) -> Result<String, String> {
    match args.len() {
        0 => Ok(display_help()),
        _ => match args[0].as_str() {
            // General:
            "--help" => Ok(display_help()),
            "-v" | "--version" => Ok(display_version()),
            // Utilities:
            "--update" => Ok("TODO: Implement update functionality".to_string()),
            "--tui" => Ok("TODO: Implement TUI functionality".to_string()),
            "-i" | "--init" => Ok("TODO: Implement init functionality".to_string()),
            // TODO: Import needs a src file
            "--import" => parse_args_import(&args[1..]),
            // TODO: Export needs a destination folder
            "--export" => parse_args_export(&args[1..]),
            // Adding, Updating, and Deleting Links:
            "-a" | "--add" => parse_args_add(&args[1..]),
            "-d" | "--delete" => parse_args_delete(&args[1..]),
            "-s" | "--show" => parse_args_show(&args[1..]),
            "-u" | "--upsert" => parse_args_upsert(&args[1..]),
            // Opening links:
            "here" => parse_args_here(&args[1..]),
            _parent_entity => parse_args_parent_entity(&args),
        },
    }
}

fn display_error() -> String {
    "too many arguments, see the Usage section with --help".to_string()
}

fn display_help() -> String {
    format!(
        "{}\n{}\n\n{}",
        display_version(),
        env!("CARGO_PKG_DESCRIPTION"),
        display_commands(),
    )
}

fn display_import_help() -> String {
    // TODO: Export needs a src file
    format!(
        "Tap import command will import a bookmark file from one of the following browsers into Tap:\n{}\n\nExample Usage: {}",
        "Chrome, Edge, Firefox, Opera, Safari, Tap",
        "tap --import [Chrome | Edge | Firefox | Opera | Safari | Tap]"
    )
}

fn display_export_help() -> String {
    // TODO: Export needs a destination folder
    format!(
        "Tap export command will export a bookmark file to one of the following browsers:\n{}\n\nExample Usage: {}",
        "Chrome, Edge, Firefox, Opera, Safari, Tap",
        "tap --export [Chrome | Edge | Firefox | Opera | Safari | Tap]"
    )
}

fn display_version() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

fn display_commands() -> String {
    let mut s = "".to_string();
    s.push_str("Usage:\n");
    s.push_str("  Opening Links:\n");
    s.push_str("   - tap <Parent Entity> [Link Name]                                   Open all Parent Entity's Link. When Link Name is supplied, opens the specific Link\n");
    s.push_str("   - tap here            [Link Name]                                   Uses the current directory as the Parent Entity. Opens all Parent Entity's links. When Link Name is supplied, opens the specific Link\n");
    s.push_str("  Adding, Updating, and Deleting Links:\n");
    s.push_str("   - tap (-a, --add)    <Parent Entity | here> <Link Name> <Value>     Add a new Link to the Parent Entity. It will create the Parent Entity if it doesn't exist\n");
    s.push_str("   - tap (-d, --delete) <Parent Entity | here> [Link Name]             Deletes an existing Link from the Parent Entity. If no Link Name is provided, deletes all Links from the Parent Entity\n");
    s.push_str("   - tap (-s, --show)   <Parent Entity | here> [Link Name]             Shows the value of an existing Link from the Parent Entity. If no Link Name is provided, shows all Link Values from the Parent Entity\n");
    s.push_str("   - tap (-u, --upsert) <Parent Entity | here> <Link Name> <Value>     Upsert an existing Link in the Parent Entity. It will create the Link and Parent Entity if it doesn't exist\n");
    s.push_str("  Utilities:\n");
    s.push_str("   - tap (-i, --init)                                                  Initializes Tap (Shell Auto-Completion, etc.)\n");
    s.push_str("   - tap --import     <Browser | Tap>                                  Imports a bookmark file into Tap. Supports both Tap Files and the following browsers' bookmark manager files: Chrome, Edge, Firefox, Opera, Safari\n");
    s.push_str("   - tap --export     <Browser | Tap>                                  Exports Tap to a bookmark file. Supported Browsers: Chrome, Edge, Firefox, Opera, Safari\n");
    s.push_str("   - tap --tui                                                         Opens a terminal user interface to facilitate adding, updating, and deleting links\n");
    s.push_str(
        "   - tap --update                                                      Updates Tap\n",
    );
    s.push_str(" - tap --help                                                          Display this help message\n");
    s.push_str(" - tap (-v, --version)                                                 Display the version\n");
    s
}

fn parse_args_here(args: &[String]) -> Result<String, String> {
    if !args.is_empty() && args.len() != 1 {
        todo!("Help for here");
    }
    if args.len() == 1 {
        Ok(format!(
            "TODO: Implement open functionality for here with Link Name {}",
            args[0]
        ))
    } else {
        Ok("TODO: Implement here functionality".to_string())
    }
}

fn parse_args_parent_entity(args: &[String]) -> Result<String, String> {
    if args.len() != 1 && args.len() != 2 {
        return Err(display_error());
    }
    if args.len() == 2 {
        Ok(format!(
            "TODO: Implement open functionality for Parent Entity {} with Link Name {}",
            args[0], args[1]
        ))
    } else {
        Ok(format!(
            "TODO: Implement open functionality for Parent Entity: {}",
            args[0]
        ))
    }
}

fn parse_args_import(args: &[String]) -> Result<String, String> {
    let err = "expected 1 argument - possible types: Chrome, Edge, Firefox, Opera, Safari, Tap"
        .to_string();
    let help = display_import_help();
    if args.len() != 1 {
        return Err(err);
    }
    match args[0].as_str() {
        "Chrome" => Ok("TODO: Implement import functionality from Chrome".to_string()),
        "Edge" => Ok("TODO: Implement import functionality from Edge".to_string()),
        "Firefox" => Ok("TODO: Implement import functionality from Firefox".to_string()),
        "Opera" => Ok("TODO: Implement import functionality from Opera".to_string()),
        "Safari" => Ok("TODO: Implement import functionality from Safari".to_string()),
        "Tap" => Ok("TODO: Implement import functionality from Tap".to_string()),
        "--help" => Ok(help),
        _ => Err(err),
    }
}

fn parse_args_export(args: &[String]) -> Result<String, String> {
    let err = "expected 1 argument - possible types: Chrome, Edge, Firefox, Opera, Safari, Tap"
        .to_string();
    let help = display_export_help();
    if args.len() != 1 {
        return Err(err);
    }
    match args[0].as_str() {
        "Chrome" => Ok("TODO: Implement export functionality to Chrome".to_string()),
        "Edge" => Ok("TODO: Implement export functionality to Edge".to_string()),
        "Firefox" => Ok("TODO: Implement export functionality to Firefox".to_string()),
        "Opera" => Ok("TODO: Implement export functionality to Opera".to_string()),
        "Safari" => Ok("TODO: Implement export functionality to Safari".to_string()),
        "Tap" => Ok("TODO: Implement export functionality to Tap".to_string()),
        "--help" => Ok(help),
        _ => Err(err),
    }
}

fn parse_args_add(args: &[String]) -> Result<String, String> {
    if args.len() != 3 {
        todo!("Help for add");
    }
    match args[0].as_str() {
        "here" => Ok(format!(
            "TODO: Implement add functionality for here with Link Name {} and Value {}",
            args[1], args[2]
        )),
        parent_entity => Ok(format!(
            "TODO: Implement add functionality for Parent Entity {} with Link Name {} and Value {}",
            parent_entity, args[1], args[2]
        )),
    }
}

fn parse_args_delete(args: &[String]) -> Result<String, String> {
    if args.len() != 1 && args.len() != 2 {
        todo!("Help for delete");
    }
    match args[0].as_str() {
        "here" => {
            if args.len() == 2 {
                Ok(format!(
                    "TODO: Implement delete functionality for here with Link Name {}",
                    args[1]
                ))
            } else {
                Ok("TODO: Implement delete functionality for here".to_string())
            }
        }
        parent_entity => {
            if args.len() == 2 {
                Ok(format!(
                    "TODO: Implement delete functionality for Parent Entity {} with Link Name {}",
                    parent_entity, args[1]
                ))
            } else {
                Ok(format!(
                    "TODO: Implement delete functionality for Parent Entity: {}",
                    parent_entity
                ))
            }
        }
    }
}

fn parse_args_show(args: &[String]) -> Result<String, String> {
    if args.len() != 1 && args.len() != 2 {
        todo!("Help for show");
    }
    match args[0].as_str() {
        "here" => {
            if args.len() == 2 {
                Ok(format!(
                    "TODO: Implement show functionality for here with Link Name {}",
                    args[1]
                ))
            } else {
                Ok("TODO: Implement show functionality for here".to_string())
            }
        }
        parent_entity => {
            if args.len() == 2 {
                Ok(format!(
                    "TODO: Implement show functionality for Parent Entity {} with Link Name {}",
                    parent_entity, args[1]
                ))
            } else {
                Ok(format!(
                    "TODO: Implement show functionality for Parent Entity: {}",
                    parent_entity
                ))
            }
        }
    }
}

fn parse_args_upsert(args: &[String]) -> Result<String, String> {
    if args.len() != 3 {
        todo!("Help for upsert");
    }
    match args[0].as_str() {
        "here" => Ok(format!(
            "TODO: Implement upsert functionality for here with Link Name {} and Value {}",
            args[1], args[2]
        )),
        parent_entity => Ok(format!(
            "TODO: Implement upsert functionality for Parent Entity {} with Link Name {} and Value {}",
            parent_entity, args[1], args[2]
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // General Tests:
    #[test]
    fn test_display_help() {
        let args = vec!["--help"].iter().map(|s| s.to_string()).collect();
        let expected = display_help();
        let res = run(args).expect("Could not display help");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_display_version() {
        let args = vec!["--version"].iter().map(|s| s.to_string()).collect();
        let expected = display_version();
        let res = run(args).expect("Could not display help");
        assert_eq!(res, expected);
    }

    // Opening Links Tests:
    #[test]
    fn test_open_parent_entity_only() {
        let args = vec!["my-repo"].iter().map(|s| s.to_string()).collect();
        let expected = "TODO: Implement open functionality for Parent Entity: my-repo".to_string();
        let res = run(args).expect("Could not display open");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_open_parent_entity_and_link() {
        let args = vec!["my-repo", "my-link"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement open functionality for Parent Entity my-repo with Link Name my-link"
                .to_string();
        let res = run(args).expect("Could not display open");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_open_current_directory_here() {
        let args = vec!["here"].iter().map(|s| s.to_string()).collect();
        let expected = "TODO: Implement here functionality".to_string();
        let res = run(args).expect("Could not display open");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_open_current_directory_here_with_link() {
        let args = vec!["here", "my-link"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement open functionality for here with Link Name my-link".to_string();
        let res = run(args).expect("Could not display open");
        assert_eq!(res, expected);
    }

    // Adding, Updating, and Deleting Links Tests:
    #[test]
    fn test_add_link_short_parent_entity() {
        let args = vec!["-a", "my-repo", "my-link", "https://google.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement add functionality for Parent Entity my-repo with Link Name my-link and Value https://google.com".to_string();
        let res = run(args).expect("Could not display add");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_add_link_long_parent_entity() {
        let args = vec!["--add", "my-repo", "my-link", "https://google.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement add functionality for Parent Entity my-repo with Link Name my-link and Value https://google.com".to_string();
        let res = run(args).expect("Could not display add");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_add_link_short_here() {
        let args = vec!["-a", "here", "my-link", "https://google.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement add functionality for here with Link Name my-link and Value https://google.com".to_string();
        let res = run(args).expect("Could not display add");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_add_link_long_here() {
        let args = vec!["--add", "here", "my-link", "https://google.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement add functionality for here with Link Name my-link and Value https://google.com".to_string();
        let res = run(args).expect("Could not display add");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_link_short_parent_entity_no_link() {
        let args = vec!["-d", "my-repo"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement delete functionality for Parent Entity: my-repo".to_string();
        let res = run(args).expect("Could not display delete");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_link_long_parent_entity_no_link() {
        let args = vec!["--delete", "my-repo"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement delete functionality for Parent Entity: my-repo".to_string();
        let res = run(args).expect("Could not display delete");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_link_short_here_no_link() {
        let args = vec!["-d", "here"].iter().map(|s| s.to_string()).collect();
        let expected = "TODO: Implement delete functionality for here".to_string();
        let res = run(args).expect("Could not display delete");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_link_long_here_no_link() {
        let args = vec!["--delete", "here"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement delete functionality for here".to_string();
        let res = run(args).expect("Could not display delete");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_link_short_parent_entity_with_link() {
        let args = vec!["-d", "my-repo", "my-link"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement delete functionality for Parent Entity my-repo with Link Name my-link"
                .to_string();
        let res = run(args).expect("Could not display delete");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_link_long_parent_entity_with_link() {
        let args = vec!["--delete", "my-repo", "my-link"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement delete functionality for Parent Entity my-repo with Link Name my-link"
                .to_string();
        let res = run(args).expect("Could not display delete");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_link_short_here_with_link() {
        let args = vec!["-d", "here", "my-link"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement delete functionality for here with Link Name my-link".to_string();
        let res = run(args).expect("Could not display delete");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_delete_link_long_here_with_link() {
        let args = vec!["--delete", "here", "my-link"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement delete functionality for here with Link Name my-link".to_string();
        let res = run(args).expect("Could not display delete");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_link_short_parent_entity_no_link() {
        let args = vec!["-s", "my-repo"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement show functionality for Parent Entity: my-repo".to_string();
        let res = run(args).expect("Could not display show");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_link_long_parent_entity_no_link() {
        let args = vec!["--show", "my-repo"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement show functionality for Parent Entity: my-repo".to_string();
        let res = run(args).expect("Could not display show");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_link_short_here_no_link() {
        let args = vec!["-s", "here"].iter().map(|s| s.to_string()).collect();
        let expected = "TODO: Implement show functionality for here".to_string();
        let res = run(args).expect("Could not display show");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_link_long_here_no_link() {
        let args = vec!["--show", "here"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement show functionality for here".to_string();
        let res = run(args).expect("Could not display show");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_link_short_parent_entity_with_link() {
        let args = vec!["-s", "my-repo", "my-link"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement show functionality for Parent Entity my-repo with Link Name my-link"
                .to_string();
        let res = run(args).expect("Could not display show");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_link_long_parent_entity_with_link() {
        let args = vec!["--show", "my-repo", "my-link"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement show functionality for Parent Entity my-repo with Link Name my-link"
                .to_string();
        let res = run(args).expect("Could not display show");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_link_short_here_with_link() {
        let args = vec!["-s", "here", "my-link"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement show functionality for here with Link Name my-link".to_string();
        let res = run(args).expect("Could not display show");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_show_link_long_here_with_link() {
        let args = vec!["--show", "here", "my-link"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "TODO: Implement show functionality for here with Link Name my-link".to_string();
        let res = run(args).expect("Could not display show");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_upsert_link_short_parent_entity() {
        let args = vec!["-u", "my-repo", "my-link", "https://google.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement upsert functionality for Parent Entity my-repo with Link Name my-link and Value https://google.com".to_string();
        let res = run(args).expect("Could not display upsert");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_upsert_link_long_parent_entity() {
        let args = vec!["--upsert", "my-repo", "my-link", "https://google.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement upsert functionality for Parent Entity my-repo with Link Name my-link and Value https://google.com".to_string();
        let res = run(args).expect("Could not display upsert");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_upsert_link_short_here() {
        let args = vec!["-u", "here", "my-link", "https://google.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement upsert functionality for here with Link Name my-link and Value https://google.com".to_string();
        let res = run(args).expect("Could not display upsert");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_upsert_link_long_here() {
        let args = vec!["--upsert", "here", "my-link", "https://google.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement upsert functionality for here with Link Name my-link and Value https://google.com".to_string();
        let res = run(args).expect("Could not display upsert");
        assert_eq!(res, expected);
    }

    // Utilities Tests:
    #[test]
    fn test_init_short() {
        let args = vec!["-i"].iter().map(|s| s.to_string()).collect();
        let expected = "TODO: Implement init functionality".to_string();
        let res = run(args).expect("Could not display init");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_init_long() {
        let args = vec!["--init"].iter().map(|s| s.to_string()).collect();
        let expected = "TODO: Implement init functionality".to_string();
        let res = run(args).expect("Could not display init");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_error() {
        let args = vec!["--import"].iter().map(|s| s.to_string()).collect();
        let expected =
            "expected 1 argument - possible types: Chrome, Edge, Firefox, Opera, Safari, Tap"
                .to_string();
        let res = run(args);
        assert_eq!(res.unwrap_err(), expected);
    }

    #[test]
    fn test_import_help() {
        let args = vec!["--import", "--help"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = display_import_help();
        let res = run(args).expect("Could not display import help");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_chrome() {
        let args = vec!["--import", "Chrome"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Chrome".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_edge() {
        let args = vec!["--import", "Edge"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Edge".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_firefox() {
        let args = vec!["--import", "Firefox"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Firefox".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_opera() {
        let args = vec!["--import", "Opera"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Opera".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_safari() {
        let args = vec!["--import", "Safari"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Safari".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_tap() {
        let args = vec!["--import", "Tap"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Tap".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_error() {
        let args = vec!["--export"].iter().map(|s| s.to_string()).collect();
        let expected =
            "expected 1 argument - possible types: Chrome, Edge, Firefox, Opera, Safari, Tap"
                .to_string();
        let res = run(args);
        assert_eq!(res.unwrap_err(), expected);
    }

    #[test]
    fn test_export_help() {
        let args = vec!["--export", "--help"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = display_export_help();
        let res = run(args).expect("Could not display export help");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_chrome() {
        let args = vec!["--export", "Chrome"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Chrome".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_edge() {
        let args = vec!["--export", "Edge"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Edge".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_firefox() {
        let args = vec!["--export", "Firefox"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Firefox".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_opera() {
        let args = vec!["--export", "Opera"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Opera".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_safari() {
        let args = vec!["--export", "Safari"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Safari".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_tap() {
        let args = vec!["--export", "Tap"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Tap".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_tui() {
        let args = vec!["--tui"].iter().map(|s| s.to_string()).collect();
        let expected = "TODO: Implement TUI functionality".to_string();
        let res = run(args).expect("Could not display tui");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_update() {
        let args = vec!["--update"].iter().map(|s| s.to_string()).collect();
        let expected = "TODO: Implement update functionality".to_string();
        let res = run(args).expect("Could not display update");
        assert_eq!(res, expected);
    }

    // Errors:
    #[test]
    fn test_display_error() {
        let args = vec!["clearly", "an", "unknown", "command"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = display_error();
        let res = run(args);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), expected);
    }
}
