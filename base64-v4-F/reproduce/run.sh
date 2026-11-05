#!/usr/bin/env bash
# Re-checks F's proof at Verus's default rlimit (no --rlimit flag).
# Uses the same patched Verus as E: E's run.sh builds it (and re-checks E with --rlimit 20 on the way).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
e="$here/../../base64-v4-E/reproduce"
"$e/run.sh"
export RUSTUP_TOOLCHAIN=1.98.1
export VERUS_Z3_PATH="$e/verus/source/z3"
cd "$here"
"$e/verus/source/target-verus/release/verus" base64-verification/src/lib.rs \
  --crate-type lib --edition 2021 \
  --cfg 'feature="std"' --cfg 'feature="alloc"' 2>&1 | tee verify.log | tail -3
# Expected last line: verification results:: 182 verified, 0 errors
