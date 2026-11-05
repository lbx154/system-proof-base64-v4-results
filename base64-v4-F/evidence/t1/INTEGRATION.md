# Local workflow integration

The installed pack is `/fastdata/v-boxiuli/noah/exp/tsF/home/skill-packs/system_proof`.
The failed installer log advertised by role facts does not exist on this machine. Do not infer its contents.

Demonstrated failures retained here:

- Original pack `setup/init .system-proof/INPUT.md`, with verification root exported, created `.production` at HEAD then stopped at `python: command not found`: `init-first.log`.
- `python3 -m argus_system_proof.input_receipt`: module absent, `missing-module.log` and `.exit`.
- Launcher `SYSTEM_PROOF_AGENT_SOURCE=/fastdata/v-boxiuli/noah/system-proof-agent` does not exist. No `verus-run`, `specdet`, `analyze-protocol` installed on initial PATH.
- Fetching the pack's documented GitHub workflow source failed with noninteractive authentication error: `clone.log`. No remote replacement was installed.
- Supplied Verus initially reported rustup absent; its rust_verify required missing librustc_driver. Installed official Rust 1.98.1 minimal, rustc-dev and llvm-tools locally (`rust-install.log`, `rust-components.log`), never rebuilt Verus or vstd. Bundled Z3 already supplied.

`adapter/init` is a copy of the pack init. `adapter/init.patch` shows its only patch: replace two unavailable Python workflow entry calls with `python3 adapter/workflow.py init`. Worktree construction, receipt input selection and ready message stay in the copied script. The ORIGINAL init was run first; the adapted copy then completed (`init-local.log`).

`adapter/workflow.py` is a replacement, not upstream Python code. It records exact request text, roots and commit, and renders 12 role/phase JSON contexts from installed `policy/live-context.json`, embedding the complete required Ground method. Schema names explicitly say local. It does not claim byte-identical upstream contexts or receipt schema. These artifacts are independently readable and the installed Ground check does not consume an upstream receipt schema. Read `.system-proof/context/engineer.ground.json` and source installed role instructions alongside it.

`adapter/bin/verus-run` is an explicit local direct runner, NOT the missing packaged implementation. Its setup verifies tool availability and writes the reviewed direct preset; verify rejects unexpected configuration, launches ONLY the supplied executable on the actual registered src/lib.rs, retains command/output/status/counts/time and source freshness, and supports the stage check's observe interface. It does not inject `--rlimit`, per-function limits, SMT options, selectors or lifetime exemptions. It does not read ambient Rust flags. `observe` is read-only and reports source/config staleness. Source/config fingerprints are operational freshness data, not semantic acceptance.

The copied adapter differs completely from unavailable upstream runner code; there is no claimed upstream diff. Every line is retained in `adapter/bin/verus-run` for review. The only patch against available pack code is `adapter/init.patch`. No shared installed tool files changed. No source, Cargo files, theorem, decode implementation, E evidence or E toolchain files changed. Production remains clean.

Reproduce from verification:

```bash
source .system-proof/t1/adapter/env.sh
.system-proof/t1/adapter/init .system-proof/INPUT.md
verus-run setup "$PWD" --in-place --package base64
verus-run verify "$PWD" --package base64
python3 "$SYSTEM_PROOF_PACK/checks/check_stage.py" ground "$PWD" --production "$PWD/.production"
```

The expected t1 verify result is exit 1, 177 verified / 2 resource errors. This is a working invocation route with an unfinished proof, not successful whole-crate verification. Later tasks must retain the default command and make the proof pass. Audit uses the unchanged installed stage checker; its replay discovers this adapter through PATH and invokes `verus-run verify`. No claimed audit pass now. All roles must source env.sh before commands because launcher environment remains unrepaired outside this workspace.
