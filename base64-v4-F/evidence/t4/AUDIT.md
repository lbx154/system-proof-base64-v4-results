# t4 Engineer audit evidence

Phase is audit. No candidate source, contract, proof, verifier or production changes were made.

Fresh replay: source `.system-proof/t1/adapter/env.sh`, then `/usr/bin/time -v verus-run verify "$PWD" --package base64`. `fresh-verification.log` reports 182 verified, 0 errors; `fresh-verification.exit` is 0. Resources: 8.94 s wall, 17.53 s user, 3.41 s system, maximum RSS 595944 KiB. This is native whole-process evidence, not serialized loop timing. The local adapter selects actual registered src/lib.rs, std+alloc with defaults/SIMD disabled, using the supplied unchanged Verus and no resource/solver overrides. Accepted t1/GROUND.md and t2/PROVE.md identify tool versions, common context, actual target/source consumption and correspondence baseline.

`correspondence.txt` confirms production clean at original d9ce0dea622ff80eacb736f95e9488cf1eaa06f4, live verification HEAD d9ce0dea622ff80eacb736f95e9488cf1eaa06f4 (ac5ce76986c9f52f8f5c4adbc8d26ceabcb3c7e1 is historical baseline metadata), and registered decode.rs identical to accepted t2/reorganized-maintenance.rs. E's adaptations and final executable maintenance rewrite remain disclosed manual correspondence TCB; neither is a machine-checked compiler refinement theorem.

## Boundary coverage

`boundary-syntax.txt` retains the full src syntax scan. No codec assume/admit was found. All explicit external bodies are classified:

- src/as_ref_spec.rs byte_slice_valid: Rust-valid byte extent assumption <=isize::MAX; retained representation TCB.
- src/as_ref_spec.rs complete_owned_cleanup: normal completion of complete recursive drop plus destination frame; retained invocation/environment TCB, with exactly-once correspondence to original implicit drop. It runs after codec result and cannot supply codec correctness.
- src/encode.rs encode_with_padding: residual external generic helper bypassed by checked inline expansion in actual encode_slice; no selected call-graph dependency.
- src/engine/general_purpose/mod.rs encode_helper: residual external callback helper bypassed by checked scalar-specialized actual internal_encode; no selected dependency.
- src/engine/mod.rs encode, encode_string, decode, decode_vec, decode_slice_unchecked: allocation/unchecked APIs outside requested closure; no selected dependency.
- src/panic_spec.rs assert_failed: assumed interface with requires false; all selected reachable uses must establish panic unreachable, not assumed successful codec behavior.
- AsRef external trait/spec methods: abstract per-invocation relation, conversion environment and cleanup environment. Slice identity/reference forwarding are reviewed interface models. No arbitrary generic purity/stability/totality follows from AsRef membership. Owned guarantees apply to this invocation's observed conversion and sufficient capacities, under complete conversion/cleanup completion/effects conditions.
- Alphabet string axioms and bundled vstd/core slice, sequence, arithmetic, indexing, copying, get_mut, Option/Result, callback and forwarding models are retained library TCB. Supplied Verus/Rust/macros/erasure/metadata/Z3 and supplied marker-metasized.patch remain toolchain TCB.

Call graph follows accepted t1/GROUND.md with t2 changes: checked usages -> actual STANDARD initialization and public helpers/wrappers -> actual internal encode/scalar tail/padding and internal estimate/decode/helper -> complete-quads loops, chunk_8/chunk_4, suffix and error conversion. Canonical encoding/image/injectivity/composition support is defined and checked; actual loop bodies consume the new bounds/preparation lemmas. All coexist under one common contract context; focused selectors do not alter semantics. The full fresh 182/0 discharges prior loop frontier plus three new lemmas and the three usage bodies. Retained external conditions above are submitted for independent disposition rather than treated as machine-proved facts.

No new reusable method emerged beyond the accepted verus-query-timing skill extension in t2. No substantive repair was performed in Audit.

Reviewer attempted drift-check code-drift over full src scope: exit 127 because the tool is absent. Full git/source comparison is the disclosed fallback; no candidate repair was needed.

## Installed report and stage result

Reviewer independently replayed actual source with the supplied verifier and accepted audit.formal-evidence, audit.code-binding and audit.original-goal. Its complete report was delivered in two letters, retained verbatim as reviewer-report-part1.txt and reviewer-report-part2.txt. Installation joined the split sentence with one space and appended a final newline, as independently compared by Reviewer. Reviewer authorized correction of its two panic_spec.rs line references from L1-L14 to L1-L13. No other report edits were made.

Initial supplemental check found only that off-by-one report link; audit-stage-check-first.log/.exit retain the failure. After the authorized report correction, installed check_stage.py audit with registered production returned ok, exit 0. Its fresh replay reports 182 verified, 0 errors. audit-stage-check.log/.exit and audit-verification.log retain the result; the canonical fresh replay is also .system-proof/audit-verification.log.
