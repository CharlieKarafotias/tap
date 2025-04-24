# Tap - Technical Roadmap

Tap is a Rust-based CLI tool that provides quick access to links and resources associated with a parent entity. The backend will use SurrealDB for data storage. This roadmap outlines the development stages from start to completion.

## Phase 1: Project Initialization

- [x] Set up the project repository
- [x] Initialize Rust project with Cargo
- [x] Set up SurrealDB database
- [x] Define initial data schema for storing entities and links
- [x] Implement basic CLI structure using Clap for argument parsing

## Phase 2: CRUD Operations for Entities and Links

- [x] Implement functionality to **add** new parent entities
- [x] Implement functionality to **upsert** parent entities
- [x] Implement functionality to **delete** parent entities
- [x] Implement functionality to **list** parent entities
- [x] Implement functionality to **add, upsert, delete, and retrieve** associated links
- [x] Write unit tests for CRUD operations

## Phase 3: Context-Aware Command Support

- [x] Implement the `here` keyword for context-aware commands
- [x] Detect current working directory and infer entity name
- [x] Implement command `tap here <key>` to retrieve the associated link
- [ ] Implement validation to ensure links exist before retrieval

## Phase 4: Auto-Completion

- [ ] Implement dynamic auto-completion for parent entities
- [ ] Implement dynamic auto-completion for entity keys
- [ ] Integrate with shell completions (Bash, Zsh, Fish, etc.)
- [ ] Write tests for auto-completion behavior

## Phase 5: Interactive Terminal UI (TUI)

- [ ] Implement basic interactive TUI interface for entity and link management
- [ ] Add support for listing and selecting entities
- [ ] Add support for adding, updating, and deleting links via TUI
- [ ] Write tests for TUI functionality

## Phase 6: Bulk Import and Export Support

- [ ] Implement import functionality from browser bookmark files (Chrome, Edge, Firefox, Opera, Safari)
- [x] Implement bulk import functionality using Tap format
- [x] Implement validation to ensure imported data follows correct schema
- [ ] Implement export functionality to generate browser-compatible bookmark files
- [ ] Write tests for import/export features

## Phase 7: Multi-OS Support

- [x] Implement macOs support
- [x] Implement Linux support
- [x] Implement Windows support
- [x] Test on macOS
- [ ] Test on Linux
- [] Test on Windows

## Phase 8: Built-in Updater

- [ ] Implement `tap update` command to check for and apply updates
- [ ] Ensure proper versioning strategy for updates
- [ ] Write tests for update functionality

## Phase 9: Final Testing and Optimization

- [ ] Conduct full integration testing
- [ ] Optimize database queries and CLI performance
- [ ] Write comprehensive documentation for all commands and features
- [ ] Release initial stable version
