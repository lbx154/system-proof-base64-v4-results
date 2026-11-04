#!/usr/bin/env bash
# Rebuilds the exact Verus used by run E and re-checks the proof on the full base64 source.
# Needs: git, curl, unzip, rustup (it installs Rust 1.98.1 if missing). Linux x86_64 or macOS.
# Takes about 15–30 minutes the first time (building Verus), a few minutes after that.
#
# Note: this follows what the team did. Stock Verus (and its vstd) is built first; then the patch
# is applied and only rust_verify is rebuilt. With the patch in place, vstd itself no longer
# builds (52 "external_trait_specification trait bound mismatch" errors), so the 179/0 result uses
# a vstd built by the unpatched Verus. See README.md.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
cd "$here"

VERUS_REPO=https://github.com/tangruize/verus.git
VERUS_COMMIT=d85f75780de687461a30e7771985c4a3e9ea0e58

# 1. Verus at the same commit, unpatched
[ -d verus ] || git clone "$VERUS_REPO" verus
git -C verus fetch -q origin "$VERUS_COMMIT" || true
git -C verus checkout -q -f "$VERUS_COMMIT"

# 2. Build stock Verus, vstd included (Rust 1.98.1 from rust-toolchain.toml, Z3 4.16.0 from get-z3.sh)
cd verus/source
rustup toolchain install 1.98.1 --profile minimal \
  --component rustc,rust-std,cargo,rustfmt,rustc-dev,llvm-tools >/dev/null
[ -x z3 ] || ./tools/get-z3.sh
# shellcheck disable=SC1091
source ../tools/activate
vargo build --release

# 3. Apply the 28-line patch and rebuild only rust_verify, as the team did
git apply "$here/verus-marker-metasized.patch"
# `activate` shadows cargo with a function that refuses to run; `command` calls the real binary
command cargo build --release -p rust_verify
cd "$here"

# 4. Check the proof: the same command and flags run E's audit used
export RUSTUP_TOOLCHAIN=1.98.1
export VERUS_Z3_PATH="$here/verus/source/z3"
"$here/verus/source/target-verus/release/verus" base64-verification/src/lib.rs \
  --crate-type lib --edition 2021 \
  --cfg 'feature="std"' --cfg 'feature="alloc"' \
  --rlimit 20 2>&1 | tee verify.log | tail -3
# Expected last line: verification results:: 179 verified, 0 errors
