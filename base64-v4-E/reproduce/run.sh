#!/usr/bin/env bash
# Rebuilds the exact Verus used by run E and re-checks the proof on the full base64 source.
# Needs: git, curl, unzip, rustup (it installs Rust 1.98.1 if missing). Linux x86_64 or macOS.
# Takes about 15–30 minutes the first time (building Verus), a few minutes after that.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
cd "$here"

VERUS_REPO=https://github.com/tangruize/verus.git
VERUS_COMMIT=d85f75780de687461a30e7771985c4a3e9ea0e58

# 1. Verus at the same commit, plus the one 28-line patch
if [ ! -d verus ]; then
  git clone "$VERUS_REPO" verus
fi
git -C verus fetch -q origin "$VERUS_COMMIT" || true
git -C verus checkout -q "$VERUS_COMMIT"
if git -C verus diff --quiet; then
  git -C verus apply "$here/verus-marker-metasized.patch"
else
  echo "verus/ already has local changes, assuming the patch is applied (delete verus/ to start clean)"
fi

# 2. Build it (Verus pins Rust 1.98.1 in its rust-toolchain.toml; Z3 4.16.0 comes from get-z3.sh)
cd verus/source
rustup toolchain install 1.98.1 --profile minimal \
  --component rustc,rust-std,cargo,rustfmt,rustc-dev,llvm-tools >/dev/null
[ -x z3 ] || ./tools/get-z3.sh
# shellcheck disable=SC1091
source ../tools/activate
vargo build --release
cd "$here"

# 3. Check the proof: the same command and flags run E's audit used
export RUSTUP_TOOLCHAIN=1.98.1
export VERUS_Z3_PATH="$here/verus/source/z3"
"$here/verus/source/target-verus/release/verus" base64-verification/src/lib.rs \
  --crate-type lib --edition 2021 \
  --cfg 'feature="std"' --cfg 'feature="alloc"' \
  --rlimit 20 2>&1 | tee verify.log | tail -3
# Expected last line: verification results:: 179 verified, 0 errors
