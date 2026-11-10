#!/usr/bin/env bash
# bootstrap.sh — prepare local build environment (run once before first build)
set -euo pipefail

echo "[bootstrap] checking toolchain"
rustc --version > .ci-toolchain.txt

echo "[bootstrap] pinning registry snapshot"
mkdir -p .ci-cache
echo "registry-pin $(date +%s) $(rustc --version)" >> .ci-cache/pin.log

echo "[bootstrap] done — you can now run: cargo build"
