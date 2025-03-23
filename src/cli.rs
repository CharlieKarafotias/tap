use std::env;

fn parse_args() -> Vec<String> {
    env::args().skip(1).collect()
}

pub fn run() {
    let args = parse_args();
    println!("Parsed args: {:#?}", args);
    // Side effects
    // match by len
    let res = match args.len() {
        0 => display_help(),
        1 => {
            match args[0].as_str() {
                // General:
                "--help" => display_help(),
                "-v" | "--version" => display_version(),
                // Utilities:
                "--update" => "TODO: Implement update functionality".to_string(),
                "-i" | "--init" => "TODO: Implement init functionality".to_string(),
                // Opening links:
                "here" => "TODO: Implement here functionality".to_string(),
                parent_entity => format!(
                    "TODO: Implement open functionality for Parent Entity: {}",
                    parent_entity
                ),
            }
        }
        2 => {
            match args[0].as_str() {
                // Utilities
                "--import" => match args[1].as_str() {
                    "Chrome" => "TODO: Implement import functionality from Chrome".to_string(),
                    "Edge" => "TODO: Implement import functionality from Edge".to_string(),
                    "Firefox" => "TODO: Implement import functionality from Firefox".to_string(),
                    "Opera" => "TODO: Implement import functionality from Opera".to_string(),
                    "Safari" => "TODO: Implement import functionality from Safari".to_string(),
                    "Tap" => "TODO: Implement import functionality from Tap".to_string(),
                    _ => display_error(),
                },
                "--export" => match args[1].as_str() {
                    "Chrome" => "TODO: Implement export functionality to Chrome".to_string(),
                    "Edge" => "TODO: Implement export functionality to Edge".to_string(),
                    "Firefox" => "TODO: Implement export functionality to Firefox".to_string(),
                    "Opera" => "TODO: Implement export functionality to Opera".to_string(),
                    "Safari" => "TODO: Implement export functionality to Safari".to_string(),
                    "Tap" => "TODO: Implement export functionality to Tap".to_string(),
                    _ => display_error(),
                },
                // Adding, Updating, and Deleting links:
                "-d" | "--delete" => match args[1].as_str() {
                    "here" => "TODO: Implement delete functionality for here".to_string(),
                    parent_entity => format!(
                        "TODO: Implement delete functionality for Parent Entity: {}",
                        parent_entity
                    ),
                },
                "-s" | "--show" => match args[1].as_str() {
                    "here" => "TODO: Implement show functionality for here".to_string(),
                    parent_entity => format!(
                        "TODO: Implement show functionality for Parent Entity: {}",
                        parent_entity
                    ),
                },
                // Opening links:
                "here" => format!(
                    "TODO: Implement open functionality for here with Link Name {}",
                    args[1]
                ),
                parent_entity => format!(
                    "TODO: Implement open functionality for Parent Entity {} with Link Name {}",
                    parent_entity, args[1]
                ),
            }
        }
        3 => {
            match args[0].as_str() {
                // Adding, Updating, and Deleting links:
                "-d" | "--delete" => match args[1].as_str() {
                    "here" => format!(
                        "TODO: Implement delete functionality for here with Link Name {}",
                        args[2]
                    ),
                    parent_entity => format!(
                        "TODO: Implement delete functionality for Parent Entity {} with Link Name {}",
                        parent_entity, args[2]
                    ),
                },
                "-s" | "--show" => match args[1].as_str() {
                    "here" => format!(
                        "TODO: Implement show functionality for here with Link Name {}",
                        args[2]
                    ),
                    parent_entity => format!(
                        "TODO: Implement show functionality for Parent Entity {} with Link Name {}",
                        parent_entity, args[2]
                    ),
                },
                _ => display_error(),
            }
        }
        4 => {
            match args[0].as_str() {
                // Adding, Updating, and Deleting links:
                "-a" | "--add" => match args[1].as_str() {
                    "here" => format!(
                        "TODO: Implement add functionality for here with Link Name {} and Value {}",
                        args[2], args[3]
                    ),
                    parent_entity => format!(
                        "TODO: Implement add functionality for Parent Entity {} with Link Name {} and Value {}",
                        parent_entity, args[2], args[3]
                    ),
                },
                "-u" | "--upsert" => match args[1].as_str() {
                    "here" => format!(
                        "TODO: Implement upsert functionality for here with Link Name {} and Value {}",
                        args[2], args[3]
                    ),
                    parent_entity => format!(
                        "TODO: Implement upsert functionality for Parent Entity {} with Link Name {} and Value {}",
                        parent_entity, args[2], args[3]
                    ),
                },
                _ => display_error(),
            }
        }
        _ => display_error(),
    };

    println!("{}", res);
    // Return behavior
}

fn display_error() -> String {
    "ERROR: too many arguments, see the Usage section with --help".to_string()
}

fn display_help() -> String {
    format!(
        "{}\n{}\n\n{}",
        display_version(),
        env!("CARGO_PKG_DESCRIPTION"),
        display_commands(),
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
    s.push_str(
        "   - tap --update                                                      Updates Tap\n",
    );
    s.push_str(" - tap --help                                                          Display this help message\n");
    s.push_str(" - tap (-v, --version)                                                 Display the version\n");
    s
}

// TODO: add tests for all functions and the run function
