# CLI App Commands Reference

## cli_app Commands

| Command | Alias | Args | Flags | Purpose |
|---|---|---|---|---|
| `cli_app load` | `l` | — | `-v` | Load and compile files from repository |
| `cli_app save` | `s` | `[pattern]` | `-v`, `-k` | Save data to repository |
| `cli_app query` | `q` | `[pattern...]` | `-v` | Query/find functions by FQN/pattern (supports wildcards) |
| `cli_app clear` | `c` | `[pattern...]` | `-v` | Reset the database, or clear functions by FQN/pattern |
| `cli_app invoke` | `i` | `<function> [params...]` | `-v` | Invoke a specific function by name or FQN |
| `cli_app test` | `t` | — | `-v`, `-k` | Run built-in integration test |
| `cli_app generate` | `g` | `<kodegen_config_key>` | `-v`, `-k`, `-p` | Generate DSL functions given a config key |
| `cli_app watch` | `w` | `<expression>` | `-v` | Interactive step-by-step debugging of expression evaluation |

### Flags

| Flag | Short | Commands | Purpose |
|---|---|---|---|
| `--verbose` | `-v` | all | Enable verbose output |
| `--keep` | `-k` | `save`, `test`, `generate` | Persist functions after command (skip cleanup) |
| `--provision` | `-p` | `generate` only | Provision workspace after generation |

### Examples

```bash
cli_app load -v
cli_app l -v
cli_app save
cli_app save mypattern -k
cli_app query "math/*" -v
cli_app q "*Add*" -v
cli_app clear -v
cli_app clear math/foo math/bar -v
cli_app c "*Add2*" -v
cli_app invoke MyFunc 1 2 3 -v
cli_app i MyFunc 1 2 3
cli_app test -v
cli_app test -v -k
cli_app generate default -v
cli_app g mykey -v -k -p
cli_app watch "sum(n1, sum(n2, n3))" -v
cli_app w "sum(n1, n2)"

# Omit required args to be prompted interactively:
cli_app generate
cli_app invoke
cli_app watch
```

---

## Release Scripts (~/dev-basab/)

| Script | Purpose |
|---|---|
| `rtgadd <version>` | Bump version, commit, tag, push to GitHub, publish to crates.io |
| `rtgdelete <version>` | Delete git tag locally + remotely, yank from crates.io |
| `rtgshow` | Show all git tags, current version, and crates.io entry |
| `rtghpush <message>` | Stage all changes, commit with message, push to GitHub |
| `rtgbupdate <branch>` | Merge current branch into target branch, push, switch back |
| `rtgcopy <branch>` | Create new branch from current, push, return to original |
| `rtguncopy <branch...>` | Delete one or more branches locally and remotely |

### Examples

```bash
rtgadd 0.1.2
rtgadd v0.1.2          # 'v' prefix also accepted
rtgdelete 0.1.1
rtgshow
rtghpush "fix: update load command"
rtgbupdate staging
rtgcopy feature-xyz
rtguncopy feature-xyz
rtguncopy branch1 branch2
```

---

## Cargo Commands

| Command | Purpose |
|---|---|
| `cargo new <name>` | Create new project |
| `cargo init` | Init project in current directory |
| `cargo build` | Compile (debug mode) |
| `cargo build --release` | Compile (optimized) |
| `cargo run` | Compile + run |
| `cargo run -- <args>` | Compile + run with arguments |
| `cargo check` | Fast type/syntax check, no binary |
| `cargo test` | Run all tests |
| `cargo fmt` | Format code |
| `cargo clippy` | Lint (like `go vet`) |
| `cargo clean` | Delete `target/` build artifacts |
| `cargo add <crate>` | Add dependency to `Cargo.toml` |
| `cargo remove <crate>` | Remove dependency |
| `cargo update` | Update dependencies to latest allowed |
| `cargo tree` | Show dependency tree |
| `cargo install <crate>` | Install binary from crates.io |
| `cargo install --path .` | Install binary from local project |
| `cargo install <crate> --version X.Y.Z` | Install specific version |
| `cargo install <crate> --force` | Reinstall / upgrade |
| `cargo uninstall <crate>` | Uninstall binary |
| `cargo publish` | Publish crate to crates.io |
| `cargo yank --version X.Y.Z <crate>` | Hide version from new installs |
| `cargo login <token>` | Authenticate with crates.io |
| `cargo search <keyword>` | Search crates.io |
| `cargo doc --open` | Generate + open local docs |
| `cargo bench` | Run benchmarks |

---

## Project Layout

```
cli_app/
├── Cargo.toml
├── Cargo.lock
├── docs/              ← hand-written documentation
├── examples/          ← runnable examples
├── tests/             ← integration tests
├── benches/           ← benchmarks
└── src/
    ├── main.rs
    ├── prompt.rs
    └── commands/
        ├── mod.rs
        ├── load.rs
        ├── save.rs
        ├── query.rs
        ├── clear.rs
        ├── invoke.rs
        ├── test.rs
        ├── generate.rs
        └── watch.rs
```
