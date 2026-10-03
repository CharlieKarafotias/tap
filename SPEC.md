# tap — Specification

> Living document. `tap` is currently v0.1.0 (pre-release). This spec describes
> the product as implemented today; roadmap items are marked FUTURE.

## 1. Overview

`tap` is a command-line tool that maps short, memorable names to links and
resources. A **parent entity** (e.g. a repository name) holds one or more named
**links** (URLs or local file paths). Instead of digging through browser
bookmarks, you run one command and `tap` opens the right link(s) in your OS
default handler.

Originally designed for software engineers juggling many repositories (build
dashboards, logs, API docs, secret portals), it generalizes to any personal or
team link-mapping need. It also supports local file navigation for offline use.

## 2. Features (implemented)

### 2.1 Open links

- `tap <Parent> [Link]` — open all links of a parent, or one named link.
- `tap here [Link]` — same, but the parent is inferred from the current
  directory's folder name (context-aware; no need to type the repo name).

Links open via the OS default handler (`open` on macOS, `xdg-open` on Linux,
`start` on Windows).

### 2.2 Manage links (CRUD)

- `tap -a|--add <Parent|here> <Link> <Value>` — add a new link.
- `tap -u|--upsert <Parent|here> <Link> <Value>` — create or update a link.
- `tap -d|--delete <Parent|here> [Link]` — delete one link, or all links of a parent.
- `tap -s|--show [Parent|here] [Link]` — list all parents (no args), list a
  parent's links (one arg), or show a link's value (two args).

### 2.3 Import / export

- `tap --import Tap <file>` / `tap --export Tap <dest>` — Tap-native format.
- `tap --export Browser <dest>` — writes a bookmark HTML file importable by browsers.
- `tap --import Browser <file>` — FUTURE (currently a stub).

### 2.4 Shell completions

- `tap -i|--init <auto|zsh>` — print a zsh completion script, or auto-install it
  into `~/.zshrc`. Zsh only.

### 2.5 Misc

- `tap -v|--version`, `tap --help`, per-command `--help`.

### Explicitly out of scope / FUTURE

- `tap --tui` (interactive terminal UI) — stub; panics if invoked.
- `tap --update` (built-in updater) — stub; panics if invoked.
- Browser bookmark import, bash completions, full Windows support.

## 3. CLI Contract

- Dispatch is on the first argument; anything unrecognized is treated as a
  parent entity name (typos in flags silently become entity lookups).
- Reserved words that cannot be parent entity names: `here`, the `|` character,
  and all CLI flags.
- Exit codes: `0` on success; `1` with `ERROR: <message>` on failure.

## 4. Architecture

- `src/main.rs` — thin entry point: parse args → `cli::run` → print result.
- `src/cli.rs` — hand-rolled argument dispatch (no clap); each command implements
  the `Command` trait (`error_message`, `help_message`, `run`).
- `src/commands/` — one module per command (`add`, `delete`, `export`, `help`,
  `here`, `import`, `init`, `parent_entity`, `show`, `tui`, `update`, `upsert`,
  `version`).
- `src/utils/` — `tap_data_store.rs` (storage engine), `command.rs` (cwd helpers),
  `os_implementations.rs` (per-OS link opening), `cli_usage_table.rs` (help rendering).

## 5. Data Model & Storage

- In-memory model: `Vec<(parent_name, Vec<(link_name, value)>)>`.
- On-disk: two files living **next to the tap binary**
  (`std::env::current_exe().parent()`), e.g. `~/.cargo/bin/`:
  - `.tap_data` — custom text format. Parent header `<name>->`, then
    `name|value` lines per link.
  - `.tap_index` — `<parent>|<byte_offset>|<byte_length>` per line, enabling
    seek-based reads without loading the whole file.
- Reads use the index for direct seeks; writes rewrite the data file and refresh
  the index. No file locking — concurrent writes can corrupt the store.
- Reinstalling or relocating the binary orphans the data store (no migration story).

## 6. Non-Goals & Known Limitations

- Zero external dependencies (std-only); keep it that way unless a clear need arises.
- No concurrent-write safety; no data migration story (see §5).
- Docs drift: README/ROADMAP mention SurrealDB, browser import, TUI, updater, and
  Windows support that are not implemented. Code is the source of truth.
