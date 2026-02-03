# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [0.5.0] - 2026-02-02

### Changed

- Project renamed from `dusk-warden` to `vault-sync`.
	- Binary/CLI command is now `vault-sync`.
	- Config file renamed to `.vault-sync.toml`.
	- Release artifacts and install script updated to `vault-sync`.
	- Documentation and GitHub workflow adjusted accordingly.

## [0.4.1] - 2026-02-01

### Fixed

- Improved error message when `push` fails due to missing write permissions on the service account token.

## [0.4.0] - 2026-02-01

### Added

- `push` command to upload local `.env` files to Bitwarden secrets.

## [0.3.0] - 2026-01-31

### Added

- `.bws` file lookup for `BWS_ACCESS_TOKEN`. Searches from the current directory upward to `$HOME`, falling back to the environment variable.

## [0.1.0] - 2026-01-31

### Added

- `sync` command to download Bitwarden secrets and write them to configured `.env` files.
- `update` command for in-place self-update from GitHub releases.
- Install script for quick setup.
