use crate::commands::help::Help;
use crate::commands::{Command, CommandResult};
use std::env;

/// Collects command-line arguments, skipping the first argument (the program name).
///
/// # Returns
///
/// A vector of strings containing the command-line arguments provided after the program name.
pub fn collect_args() -> Vec<String> {
    env::args().skip(1).collect()
}

pub fn run(args: Vec<String>) -> Result<CommandResult, String> {
    match args.len() {
        0 => Help::default().cli_run(&[]),
        _ => match args[0].as_str() {
            // General:
            "--help" => Help::default().cli_run(&args[1..]),
            // "-v" | "--version" => Ok(display_version()),
            // // Utilities:
            // "--update" => Ok("TODO: Implement update functionality".to_string()),
            // "--tui" => Ok("TODO: Implement TUI functionality".to_string()),
            // "-i" | "--init" => Ok("TODO: Implement init functionality".to_string()),
            // "--import" => parse_args_import(&args[1..]),
            // "--export" => parse_args_export(&args[1..]),
            // // Adding, Updating, and Deleting Links:
            // "-a" | "--add" => parse_args_add(&args[1..]),
            // "-d" | "--delete" => parse_args_delete(&args[1..]),
            // "-s" | "--show" => parse_args_show(&args[1..]),
            // "-u" | "--upsert" => parse_args_upsert(&args[1..]),
            // // Opening links:
            // "here" => parse_args_here(&args[1..]),
            // _parent_entity => parse_args_parent_entity(&args),
            _ => Ok(CommandResult::Value("TODO".to_string())),
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

fn display_add_help() -> String {
    format!(
        "Tap --add command will add a new link to the Parent Entity\n\nExample Usage: {}",
        "tap --add search-engines google https://google.com"
    )
}

fn display_delete_help() -> String {
    format!(
        "Tap --delete command will delete either a specific link or all links of a Parent Entity\n\nExample Usage: {}\n{}",
        "Delete all links: tap --delete search-engines",
        "Delete specific link: tap --delete search-engines google"
    )
}

fn display_show_help() -> String {
    format!(
        "Tap --show command will show either a specific link or all links of a Parent Entity\n\nExample Usage: {}\n{}",
        "Show all links: tap --show search-engines",
        "Show specific link: tap --show search-engines google"
    )
}

fn display_upsert_help() -> String {
    format!(
        "Tap --upsert command will add/update a new/existing link to the Parent Entity\n\nExample Usage: {}",
        "tap --upsert search-engines google https://google.com"
    )
}

fn display_import_help() -> String {
    format!(
        "Tap import command will import a bookmark file from one of the following browsers into Tap:\n{}\n\nExample Usage: {}",
        "Chrome, Edge, Firefox, Opera, Safari, Tap",
        "tap --import [Chrome | Edge | Firefox | Opera | Safari | Tap] <bookmark file>"
    )
}

fn display_export_help() -> String {
    format!(
        "Tap export command will export a bookmark file to one of the following browsers:\n{}\n\nExample Usage: {}",
        "Chrome, Edge, Firefox, Opera, Safari, Tap",
        "tap --export [Chrome | Edge | Firefox | Opera | Safari | Tap] <destination folder>"
    )
}

fn display_here_help() -> String {
    format!(
        "Tap here command will use your current working directory as the Parent Entity and will open either all or a specific link\n\nExample Usage: {}\n{}",
        "Open all links: tap here", "Open specific link: tap here google"
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
    let err = "expected 0 or 1 arguments - see 'tap here --help' for more information".to_string();
    let help = display_here_help();
    if !args.is_empty() && args.len() != 1 {
        return Err(err);
    }
    if args.len() == 1 {
        if args[0] == "--help" {
            return Ok(help);
        }
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
    let err = "expected 2 arguments - see 'tap --import --help' for more information.".to_string();
    let help = display_import_help();
    if args.len() == 1 && args[0] == "--help" {
        return Ok(help);
    }
    if args.len() != 2 {
        return Err(err);
    }
    match (args[0].as_str(), args[1].as_str()) {
        ("Chrome", f) => Ok(format!(
            "TODO: Implement import functionality from Chrome: {f}"
        )),
        ("Edge", f) => Ok(format!(
            "TODO: Implement import functionality from Edge: {f}"
        )),
        ("Firefox", f) => Ok(format!(
            "TODO: Implement import functionality from Firefox: {f}"
        )),
        ("Opera", f) => Ok(format!(
            "TODO: Implement import functionality from Opera: {f}"
        )),
        ("Safari", f) => Ok(format!(
            "TODO: Implement import functionality from Safari: {f}"
        )),
        ("Tap", f) => Ok(format!(
            "TODO: Implement import functionality from Tap: {f}"
        )),
        _ => Err(err),
    }
}

fn parse_args_export(args: &[String]) -> Result<String, String> {
    let err = "expected 2 arguments - see 'tap --export --help' for more information.".to_string();
    let help = display_export_help();
    if args.len() == 1 && args[0] == "--help" {
        return Ok(help);
    }
    if args.len() != 2 {
        return Err(err);
    }
    match (args[0].as_str(), args[1].as_str()) {
        ("Chrome", f) => Ok(format!(
            "TODO: Implement export functionality to Chrome: {f}"
        )),
        ("Edge", f) => Ok(format!("TODO: Implement export functionality to Edge: {f}")),
        ("Firefox", f) => Ok(format!(
            "TODO: Implement export functionality to Firefox: {f}"
        )),
        ("Opera", f) => Ok(format!(
            "TODO: Implement export functionality to Opera: {f}"
        )),
        ("Safari", f) => Ok(format!(
            "TODO: Implement export functionality to Safari: {f}"
        )),
        ("Tap", f) => Ok(format!("TODO: Implement export functionality to Tap: {f}")),
        _ => Err(err),
    }
}

fn parse_args_add(args: &[String]) -> Result<String, String> {
    let err = "expected 3 arguments - a Parent Entity, a Link Name, and a Value".to_string();
    let help = display_add_help();
    if args.len() == 1 && args[0].as_str() == "--help" {
        return Ok(help);
    }
    if args.len() != 3 {
        return Err(err);
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
    let err = "expected 1 or 2 arguments - a Parent Entity and optionally a Link Name".to_string();
    let help = display_delete_help();
    if args.len() != 1 && args.len() != 2 {
        return Err(err);
    }
    match args[0].as_str() {
        "--help" => Ok(help),
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
    let err = "expected 1 or 2 arguments - a Parent Entity and optionally a Link Name".to_string();
    let help = display_show_help();
    if args.len() != 1 && args.len() != 2 {
        return Err(err);
    }
    match args[0].as_str() {
        "--help" => Ok(help),
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
    let err = "expected 3 arguments - a Parent Entity, a Link Name, and a Value".to_string();
    let help = display_upsert_help();
    if args.len() == 1 && args[0].as_str() == "--help" {
        return Ok(help);
    }
    if args.len() != 3 {
        return Err(err);
    }
    match args[0].as_str() {
        "--help" => Ok(help),
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
    fn test_current_directory_help() {
        let args = vec!["here", "--help"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = display_here_help();
        let res = run(args).expect("Could not display here help");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_current_directory_error() {
        let args = vec!["here", "some", "error"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected =
            "expected 0 or 1 arguments - see 'tap here --help' for more information".to_string();
        let res = run(args).expect_err("Could not display here error");
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
    fn test_add_error() {
        let args = vec!["--add"].iter().map(|s| s.to_string()).collect();
        let expected =
            "expected 3 arguments - a Parent Entity, a Link Name, and a Value".to_string();
        let res = run(args);
        assert_eq!(res.unwrap_err(), expected);
    }

    #[test]
    fn test_add_help() {
        let args = vec!["--add", "--help"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = display_add_help();
        let res = run(args).expect("Could not display add help");
        assert_eq!(res, expected);
    }

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
    fn test_delete_error() {
        let args = vec!["--delete"].iter().map(|s| s.to_string()).collect();
        let expected =
            "expected 1 or 2 arguments - a Parent Entity and optionally a Link Name".to_string();
        let res = run(args);
        assert_eq!(res.unwrap_err(), expected);
    }

    #[test]
    fn test_delete_help() {
        let args = vec!["--delete", "--help"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = display_delete_help();
        let res = run(args).expect("Could not display delete help");
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
    fn test_show_error() {
        let args = vec!["--show"].iter().map(|s| s.to_string()).collect();
        let expected =
            "expected 1 or 2 arguments - a Parent Entity and optionally a Link Name".to_string();
        let res = run(args);
        assert_eq!(res.unwrap_err(), expected);
    }

    #[test]
    fn test_show_help() {
        let args = vec!["--show", "--help"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = display_show_help();
        let res = run(args).expect("Could not display show help");
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
    fn test_upsert_error() {
        let args = vec!["--upsert"].iter().map(|s| s.to_string()).collect();
        let expected =
            "expected 3 arguments - a Parent Entity, a Link Name, and a Value".to_string();
        let res = run(args);
        assert_eq!(res.unwrap_err(), expected);
    }

    #[test]
    fn test_upsert_help() {
        let args = vec!["--upsert", "--help"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = display_upsert_help();
        let res = run(args).expect("Could not display upsert help");
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
            "expected 2 arguments - see 'tap --import --help' for more information.".to_string();
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
        let args = vec!["--import", "Chrome", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Chrome: ./test.json".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_edge() {
        let args = vec!["--import", "Edge", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Edge: ./test.json".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_firefox() {
        let args = vec!["--import", "Firefox", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Firefox: ./test.json".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_opera() {
        let args = vec!["--import", "Opera", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Opera: ./test.json".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_safari() {
        let args = vec!["--import", "Safari", "./test.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Safari: ./test.json".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_import_tap() {
        let args = vec!["--import", "Tap", "./test.tap"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement import functionality from Tap: ./test.tap".to_string();
        let res = run(args).expect("Could not display import");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_error() {
        let args = vec!["--export"].iter().map(|s| s.to_string()).collect();
        let expected =
            "expected 2 arguments - see 'tap --export --help' for more information.".to_string();
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
        let args = vec!["--export", "Chrome", "./Desktop"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Chrome: ./Desktop".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_edge() {
        let args = vec!["--export", "Edge", "./Desktop"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Edge: ./Desktop".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_firefox() {
        let args = vec!["--export", "Firefox", "./Desktop"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Firefox: ./Desktop".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_opera() {
        let args = vec!["--export", "Opera", "./Desktop"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Opera: ./Desktop".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_safari() {
        let args = vec!["--export", "Safari", "./Desktop"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Safari: ./Desktop".to_string();
        let res = run(args).expect("Could not display export");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_export_tap() {
        let args = vec!["--export", "Tap", "./Desktop"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "TODO: Implement export functionality to Tap: ./Desktop".to_string();
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
