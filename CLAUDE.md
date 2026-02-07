# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

vault-sync is a Rust CLI tool that syncs Bitwarden Secrets Manager entries to local `.env` files (and can push local files back). It shells out to the `bws` CLI for Bitwarden API access.

## Build & Development Commands

Task runner: `just` (see `justfile`)

| Command                        | Description                               |
| ------------------------------ | ----------------------------------------- |
| `just build`                   | Release build                             |
| `just check`                   | Type-check without building               |
| `just test`                    | Run all tests                             |
| `just lint`                    | Clippy with `-D warnings`                 |
| `just fmt`                     | Format code                               |
| `just fmt-check`               | Check formatting                          |
| `just run -- <args>`           | Run CLI with arguments                    |
| `just release patch --execute` | Release a new version (patch/minor/major) |

Run a single test: `cargo test <test_name>` (e.g., `cargo test check_bws_error_returns_permission_message_on_404`)

## Architecture

Four source modules, all under `src/`:

- **main.rs** — CLI entry point (clap derive), three commands: `sync`, `push`, `update`. Contains core logic for calling `bws` CLI, parallel secret fetching (rayon ThreadPool, max 3 concurrent), and retry with exponential backoff for 429 rate limits.
- **config.rs** — Loads `.vault-sync.toml` config file. Resolves `BWS_ACCESS_TOKEN` from env var or by walking up directory tree for a `.bws` file. Supports `{{ env.VAR }}` template syntax via minijinja.
- **reporter.rs** — Styled terminal output (updated, up-to-date, pushed, retrying, etc.) with ANSI-aware alignment.
- **styles.rs** — Color traits using `owo-colors` with `NO_COLOR` env var support. `AnsiPadding` trait for right-aligned labels.

## Key Patterns

- **Error handling**: `anyhow` with `.context()` for all fallible operations.
- **bws interaction**: Spawns `bws` as a child process (`std::process::Command`), parses JSON stdout. Errors are checked against known patterns (e.g., 404 → permission error message).
- **Config format**: TOML with `[[secrets]]` array entries mapping `id` (Bitwarden secret UUID) to `path` (local file path).
- **Style Gallery**: terminal output goes through the `reporter.rs` class and has tests in `style_gallery`. Anytime new outputs are added they should go in both.

## CI/CD

- **build.yml**: Runs `cargo test` and `cargo clippy` on push to main (src/Cargo changes only).
- **release.yml**: Triggered by `v*` tags. Builds for 3 targets (macOS ARM, macOS Intel, Linux musl), creates GitHub release, updates Homebrew tap.

## Release Process

Uses `cargo-release`. Run `just release patch --execute` to bump version, update CHANGELOG.md, commit, tag, and push. The tag push triggers the release workflow.

CLAUDE should never run release, commit, tag, or push unless explicitly directed to do so.
When asked to update the CHANGELOG add a new section with UNRELEASED as the version, and todays date.
