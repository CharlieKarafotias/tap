# AGENTS.md

Instructions for AI coding agents working in this repository.

## Project

`tap` is a Rust command-line tool that helps you quickly access links and resources
associated with a parent entity (repositories, projects, etc.). It also supports
local file navigation for working offline.

## Commands

Standard Rust workflow (edition 2024):

- `cargo build` — build the binary
- `cargo run -- <args>` — run the CLI locally
- `cargo test` — run the test suite
- `cargo clippy --all-targets -- -D warnings` — lint; keep this clean
- `cargo fmt --check` — check formatting (`cargo fmt` to fix)

Pre-commit hooks (`.pre-commit-config.yaml`) run `fmt`, `cargo-check`, `clippy`,
and `cargo test`. Run the full check suite before pushing.

## Layout

- `src/main.rs` — binary entry point
- `src/cli.rs` — CLI definition
- `src/commands.rs`, `src/commands/` — subcommand implementations
- `src/utils.rs`, `src/utils/` — shared helpers

## Conventions

- Idiomatic Rust: prefer `Result`/`Option` over panics; propagate errors with `?`.
- Keep functions small and single-purpose; prefer new modules over large files.
- `cargo fmt` formatting is required.
- Address all clippy warnings; don't add `#[allow(...)]` without an explanatory comment.
- Add or update tests alongside behavior changes.
- Keep user-facing CLI output and `--help` text terse and consistent.
