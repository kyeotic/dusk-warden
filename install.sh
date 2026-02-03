#!/bin/bash
set -euo pipefail

REPO="kyeotic/vault-sync"

# Detect OS
case "$(uname -s)" in
    Darwin) os="apple-darwin" ;;
    Linux)  os="unknown-linux-musl" ;;
    *)      echo "Unsupported OS: $(uname -s)"; exit 1 ;;
esac

# Detect arch
case "$(uname -m)" in
    x86_64|amd64)  arch="x86_64" ;;
    arm64|aarch64) arch="aarch64" ;;
    *)             echo "Unsupported architecture: $(uname -m)"; exit 1 ;;
esac

target="${arch}-${os}"

# Linux only supports x86_64
if [ "$os" = "unknown-linux-musl" ] && [ "$arch" != "x86_64" ]; then
    echo "Unsupported Linux architecture: $arch (only x86_64 is available)"
    exit 1
fi

echo "Detected target: ${target}"

# Get latest release tag
tag=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)
echo "Latest release: ${tag}"

# Download and extract
url="https://github.com/${REPO}/releases/download/${tag}/vault-sync-${target}.tar.gz"
echo "Downloading ${url}..."

tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

curl -fsSL "$url" -o "${tmpdir}/vault-sync.tar.gz"
tar xzf "${tmpdir}/vault-sync.tar.gz" -C "$tmpdir"

# Install
install -d /usr/local/bin
install "${tmpdir}/vault-sync" /usr/local/bin/vault-sync
echo "Installed vault-sync to /usr/local/bin/vault-sync"
