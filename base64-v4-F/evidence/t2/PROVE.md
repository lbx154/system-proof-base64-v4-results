# t2 Prove delivery

The unchanged-executable reorganized crate verifies **182 verified, 0 errors at default limits**. Applying the accepted t3 maintenance change also verifies **182/0**, with **zero additional proof edits**. The registered candidate contains that integrated variant. All original E public contracts, supported domain, caller theorem, environment conditions and production/verifier revisions are retained. The result is ready for independent Prove review, not original-goal completion before Audit.

## Proof and integration

`.system-proof/phase` was set to `prove` as the first Prove action, and Engineer Prove context/role guidance loaded. Accepted t1/GROUND.md supplies the unchanged theorem, call graph and TCB; Ground was not repeated. The only Cargo/source file changed against archived E is `src/engine/general_purpose/decode.rs` (source-comparison.txt). Production and the supplied toolchain were not changed.

`canonical_quads_use` has its original definition and now controls unfolding with `opaque`. The existing actual `decode_helper` reveals it to establish the unchanged `decode_complete_quads` precondition. Three checked proof lemmas isolate canonical image expansion, bounds, and the 32-/4-byte chunk preparation. Each preparation lemma derives valid table entries and chunk-byte equality for the concrete slice passed to the existing decode calls. No assumption, external body, resource attribute, solver setting or specification weakening was introduced.

The loops consume these lemma conclusions, retain arithmetic/slice-safety facts and the output prefix/complement frame, and drop image, witness-bound, terminal-index and table invariants now internal to the lemmas. Calls and output-frame composition remain in the actual loop bodies; no copied executable implementation replaces them. Public wrappers and the three source-grounded usage bodies coexist with all proof dependencies in every full run. There are three extra verified proof bodies, explaining 179 -> 182; the guarantee is unchanged.

Construction used one decomposition round: round1.log verifies both loops and all new lemmas but finds one caller assertion requiring the newly opaque predicate's explicit reveal (181/1). Adding that reveal and removing redundant loop invariants gives round2.log (182/0). There were no repeated failing-loop assertion rounds and no raised-limit attempts.

`original-decode.rs` records E; `reorganized-original.rs` records the successful unchanged-executable variant. `proof-reorganization.patch` connects them. `integrated-maintenance.patch` applies the exact executable transformation in accepted t3/executable.patch to the reorganized source. The three hunks were mapped to their new line locations; shared source was not overwritten from t3's isolated tree. `reorganized-maintenance.rs` matches the final registered source. Existing proof calls, including the second loop's equivalent index expression, needed no edit. Additional maintenance proof-repair scope is **0 lines, 0 files, 0 lemmas**, matching E's zero repair cost for this specific change.

The accepted executable equivalence reasoning in t3/COMPARISON.md applies: guards bound both old/new endpoints by input_index_end, all intermediate arithmetic fits usize, selected slices and decoder offsets are equal, and call/write/error order is preserved for arbitrary valid inputs, including partial writes on error. Production is read-only and unchanged. Existing manual E-to-production correspondence and all generic/environment TCB remain required; this is not a machine-checked compiler refinement theorem.

## Reproduction and identities

From the registered verification root:

```bash
source .system-proof/t1/adapter/env.sh
verus-run verify "$PWD" --package base64
bash .system-proof/t2/measure.sh reorganized-maintenance
```

`final-default.log` is a fresh canonical runner success over the integrated actual crate, **182/0**, exit 0. `measure.sh` uses actual src/lib.rs, crate-type lib, edition 2021, std+alloc, and only observational time/expanded/JSON/SMT logging. The original measurement preceded maintenance application; its retained source snapshot is the input to that run. Replaying it requires restoring that snapshot in an isolated verification checkout, not running the original label on the integrated source.

Production revision d9ce0dea622ff80eacb736f95e9488cf1eaa06f4; verification baseline ac5ce76986c9f52f8f5c4adbc8d26ceabcb3c7e1. Supplied Verus 0.2026.09.29.d85f757.dirty (tangruize/verus d85f7578 plus original marker-metasized.patch), Rust 1.98.1, bundled Z3 4.16.0. No frontend or bundled library changes. Both measurements use the registered t1 adapter. Tool and scope/TCB details remain in t1/GROUND.md and INTEGRATION.md.

## Measurements

Each value is one observation. Resource counts are solver work, not runtime throughput. A canceled baseline query denotes budget exhaustion, not a false theorem.

| Variant | Default whole crate | Process wall s | Max RSS KiB | Native decode_complete_quads SMT s / units |
| --- | --- | --- | --- | --- |
| Preserved E, accepted t1 | 177/2 | 9.71 | 582828 | 3.659348 / 62,110,056 |
| E + maintenance, accepted t3 | 179/0 | 9.65 | 578560 | 3.016409 / 40,430,246 |
| Reorganized, executable unchanged | 182/0 | 9.65 | 579188 | 2.214618 / 42,112,942 |
| Reorganized + maintenance | 182/0 | 9.09 | 578136 | 1.272218 / 19,221,946 |

Native aggregates above include loop and final-body queries, but exclude the new lemmas. Their native totals are **0.135729 s / 905,644 units** before maintenance and **0.132564 s / 905,644 units** after it. Thus loop/function reduction is not achieved by hiding extracted work. Raw function breakdowns remain in each result.json. Parallel native totals and serialized replay below are different measurements and must not be added together.

| Generated query | E original: replay s / units | E + maintenance | Reorganized original | Reorganized + maintenance |
| --- | --- | --- | --- | --- |
| 32-byte loop | 1.911483 / 30,000,000 canceled | 2.289 / 28,210,723 | 1.045876 / 17,839,943 | 0.842809 / 10,962,824 |
| 4-byte loop | 1.672643 / 30,000,000 canceled | 0.454 / 9,100,169 | 0.981368 / 21,119,341 | 0.337279 / 6,443,181 |
| canonical_quads_bounds (new) | — | — | 0.007885 / 26,531 | 0.007850 / 26,531 |
| prepare_decode_32 (new) | — | — | 0.087895 / 814,142 | 0.091733 / 814,142 |
| prepare_decode_4 (new) | — | — | 0.015098 / 64,971 | 0.015184 / 64,971 |
| Final decode_complete_quads body | retained baseline JSON | retained t3 JSON | 0.199247 / 3,153,658 | 0.134508 / 1,815,941 |

All reorganized query responses are unsat. Original E loop spans are decode.rs:284 and :390; reorganized variants are :364 and :431, bound by generated source comments. Lemmas are at :235, :245, :296. Each replay uses the timing skill's unchanged generated SMT/options/order and supplied Z3, with echo timing markers only. `query-replay/baseline-loop-queries.json` contains complete responses; `query-summary.jsonl` contains source/query associations. These are serialized generated-query timings, not native per-loop timings or separate verification results.

The integrated maximum loop work in this observation is 10.96M versus E+maintenance's 28.21M against the 30M default query limit. The unchanged-executable reorganization's second loop uses 21.12M. This supports greater observed resource headroom for the integrated proof, not a stability distribution or general maintenance ranking. Both variants cost zero proof repairs for this change. E's supplied historical published report states 179/0 with --rlimit 20; the fresh results achieve full verification without that option. No new raised-limit measurement was made.

## Review and next step

Interim independent Reviewer accepted the abstraction boundary, unchanged theorem strength, unchanged executable statements before maintenance, source/caller/environment preservation and actual-body binding. Final integrated review is requested against this report, the patches, actual source, final-default.log and both measurements. Accepted t3 evidence is consumed without repeating its experiment. On acceptance, Planner/Manager can decide Prove -> Audit; Audit must independently replay the final actual-code closure and judge the retained TCB/correspondence.

The verus-query-timing skill was extended under the learning guidance with runnable `summarize.py`: select loop plus extracted-lemma functions, print responses/spans/resource costs, and explicitly label serialized sums. Its case note records that maintenance on failing E can pass with zero proof edits, so a baseline failure alone does not demonstrate a repair-cost advantage.
