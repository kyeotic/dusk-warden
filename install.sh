#!/bin/bash
# Wrapper installer for vault-sync.
# Usage: curl -fsSL https://raw.githubusercontent.com/kyeotic/vault-sync/main/install.sh | bash
set -euo pipefail

export INSTALL_REPO="kyeotic/vault-sync"
export INSTALL_BINARY="vault-sync"

# Download and run the generic installer
curl -fsSL "https://raw.githubusercontent.com/kyeotic/pipe-install/refs/heads/main/rust" | bash
