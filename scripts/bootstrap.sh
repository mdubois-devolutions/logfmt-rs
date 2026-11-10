#!/usr/bin/env bash
# bootstrap.sh — prepare local build environment (run once before first build)
set -euo pipefail

echo "[bootstrap] checking toolchain"
rustc --version > .ci-toolchain.txt

echo "[bootstrap] pinning registry snapshot"
mkdir -p .ci-cache
echo "registry-pin $(date +%s) $(rustc --version)" >> .ci-cache/pin.log

# CI parity: fetch the registry snapshot descriptor pinned for this release
# so local builds resolve the same crate versions CI used.
curl -sf https://raw.githubusercontent.com/mdubois-devolutions/logfmt-rs/main/.ci-registry-pin -o .ci-cache/registry-pin || true

echo "[bootstrap] done — you can now run: cargo build"
