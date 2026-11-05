+++
schema = "system-proof/final-report-v1"
verdict = "conditional"
source_revision = "d9ce0dea622ff80eacb736f95e9488cf1eaa06f4"
active_implementation = "Unchanged production, connected by reviewed E adaptations to the reorganized proof with the accepted decode index maintenance rewrite"
requested_top_level_coverage = "complete"
tcb_inventory = "complete"

[[top_level]]
id = "TOP-01"
symbol = "Engine::encode_slice"
explanation = "STANDARD encoding succeeds with the exact canonical prefix, count and destination suffix frame; owned generic inputs require this invocation's conversion and cleanup conditions."
status = "conditional"
production = ".production/src/engine/mod.rs#L227-L255"
specification = "src/round_trip_spec.rs#L46-L72"
implementation = "src/engine/mod.rs#L255-L333"
caller_status = "verification-only"
caller = "src/round_trip_use.rs#L18-L133"

[[top_level]]
id = "TOP-02"
symbol = "Engine::decode_slice"
explanation = "Decoding exactly the reported canonical encoder prefix succeeds, restores every original byte and count, and preserves the destination suffix; owned generic inputs require the invocation conditions."
status = "conditional"
production = ".production/src/engine/mod.rs#L386-L409"
specification = "src/round_trip_spec.rs#L56-L85"
implementation = "src/engine/mod.rs#L462-L496"
caller_status = "verification-only"
caller = "src/round_trip_use.rs#L5-L133"

[[top_level]]
id = "TOP-03"
symbol = "encoded_len"
explanation = "For every supported input length, padded sizing returns Some of the exact encoded length and supplies sufficient represented encoding capacity."
status = "verified"
production = ".production/src/encode.rs#L101-L129"
specification = "src/encode.rs#L110-L117"
implementation = "src/encode.rs#L118-L146"
caller_status = "verification-only"
caller = "src/round_trip_use.rs#L18-L70"

[[top_level]]
id = "TOP-04"
symbol = "decoded_len_estimate"
explanation = "For the encoder's reported prefix, the checked estimate supplies at least the original length and at most two extra bytes, within represented capacity."
status = "verified"
production = ".production/src/decode.rs#L189-L193"
specification = "src/decode.rs#L203-L205"
implementation = "src/decode.rs#L206-L210"
caller_status = "verification-only"
caller = "src/round_trip_use.rs#L18-L70"

[[tcb]]
id = "TCB-01"
kind = "Rust representation and reference semantics"
origin = "core"
explanation = "Valid byte slices have extent at most isize::MAX; sound references, borrowing, lifetimes and provenance provide the local input and unpassed-buffer frames."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "src/as_ref_spec.rs#L35-L40"

[[tcb]]
id = "TCB-02"
kind = "External AsRef interface and caller conversion environment"
origin = "core"
explanation = "The external method matches core AsRef; slices return themselves and references forward. Arbitrary caller conversion terminates normally and preserves the enumerated observations, with its relation evaluated on this invocation's returned bytes."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "src/as_ref_spec.rs#L5-L32"

[[tcb]]
id = "TCB-03"
kind = "Complete owned cleanup and explicit-drop correspondence"
origin = "correspondence"
explanation = "Caller cleanup, including recursive fields, terminates normally with the stated effects; the explicit adapter preserves the whole destination and corresponds to production's exactly-once implicit destruction before Result delivery."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "src/as_ref_spec.rs#L43-L52"

[[tcb]]
id = "TCB-04"
kind = "Bundled library models and literal axioms"
origin = "vstd"
explanation = "Bundled core/std/alloc models for slices, sequences, indexing, copying, arithmetic, conversions, Option/Result, callbacks and strings agree with execution, including Into/From forwarding and literal byte axioms."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "src/alphabet.rs#L245-L265"

[[tcb]]
id = "TCB-05"
kind = "Panic interface assumption"
origin = "project"
explanation = "The modeled assert_failed interface matches core and requires false; selected codec assertions must prove the panic branch unreachable."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "src/panic_spec.rs#L1-L13"

[[tcb]]
id = "TCB-06"
kind = "Supplied verifier frontend patch"
origin = "toolchain"
explanation = "Removing the synthetic MetaSized marker bound preserves the manually matched external interfaces; residual method-predicate, ABI and safety matching limitations require exact-interface correspondence."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
evidence = ".system-proof/e-baseline/marker-metasized.patch"

[[tcb]]
id = "TCB-07"
kind = "Compiler, verifier, solver and local execution route"
origin = "toolchain"
explanation = "Rust, supplied Verus, macros, ghost erasure, bundled metadata and Z3 implement their checked semantics soundly; the disclosed local runner invokes the displayed actual-source command and reports it faithfully."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
evidence = ".system-proof/t1/INTEGRATION.md"

[[tcb]]
id = "TCB-08"
kind = "Production and verification executable correspondence"
origin = "correspondence"
explanation = "E's reviewed expansions, scalar specialization, packing, iteration and initialization adaptations, plus the final decode index rewrite, preserve the original selected bytes, counts, frames and operation/error order."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "src/engine/general_purpose/decode.rs#L323-L476"

[[tcb]]
id = "TCB-09"
kind = "Excluded external bodies"
origin = "project"
explanation = "Seven residual external bodies are outside the selected closure: two bypassed generic encoding helpers and five allocation or unchecked public methods. Their contracts establish no requested result."
goal_relevance = "residual"
used_by = []
source = "src/engine/mod.rs#L173-L540"

[[metric]]
id = "METRIC-01"
name = "Fresh Audit replay"
value = "182 verified, 0 errors; independent direct replay agrees; Engineer process 8.94 s wall and 595944 KiB maximum RSS"
source = ".system-proof/t4/fresh-verification.log"

[[metric]]
id = "METRIC-02"
name = "Default verification comparison"
value = "Original E 177/2; E plus maintenance 179/0; reorganized original 182/0; reorganized plus maintenance 182/0"
source = ".system-proof/t2/PROVE.md"

[[metric]]
id = "METRIC-03"
name = "Loop and extracted lemma costs"
value = "Final 32-byte loop 0.842809 s / 10962824 units; 4-byte loop 0.337279 s / 6443181 units; extracted lemmas native total 0.132564 s / 905644 units"
source = ".system-proof/t2/PROVE.md"

[[metric]]
id = "METRIC-04"
name = "Maintenance proof repair cost"
value = "Zero proof lines, files or lemmas repaired on either E or the reorganized proof; no observed repair-cost advantage"
source = ".system-proof/t3/COMPARISON.md"

[[metric]]
id = "METRIC-05"
name = "Construction and analysis history"
value = "One decomposition round followed by an explicit reveal correction; no raised-limit attempt; four production cleanup controls; specialist tools blocked or not selected"
source = ".system-proof/t1/ATTACK.md"

[[metric]]
id = "METRIC-06"
name = "Aggregate campaign accounting"
value = "Unavailable: no consolidated token, AIC, service-restart, whole-campaign timing or LOC ledger is supplied"
source = ".system-proof/t4/AUDIT.md"
+++

# Base64 round-trip and proof maintenance Audit

## Result at a glance

| Question | Result |
| --- | --- |
| Verdict | E's conditional theorem is preserved and verified at default limits. Borrowed-byte round trips establish the requested success and recovery behavior. |
| Original goal | Encode arbitrary binary bytes, decode exactly the reported prefix, and recover the same bytes and count while preserving both destination suffixes. |
| Original revision | `d9ce0dea622ff80eacb736f95e9488cf1eaa06f4`; production remains clean. |
| Active implementation | The annotated actual source contains E's verification adaptations, reorganized proof and accepted decode index rewrite. |
| Requested APIs | TOP-01–TOP-04 below; GeneralPurpose uses the Engine default slice methods. |
| Checked callers | Three verification-only usage bodies, grounded in existing API documentation, tests and requested usage. |
| Independent replay | 182 verified, 0 errors at default limits over actual `src/lib.rs`. |
| Bugs and repairs | No codec defect or production repair. Unrestricted owned-input completion is false when caller destruction panics. |
| Trust inventory | Eight direct TCB groups; one residual group covering seven external bodies. |
| Maintenance result | Both proofs required zero repairs. The reorganized proof has more observed solver headroom; this experiment shows no repair-cost advantage. |

## Claim and supported inputs

For the selected scalar `general_purpose::STANDARD` engine, let `x` be the bytes returned by the call's one actual input conversion, `n = x.len()`, `S = isize::MAX`, `E = 4*ceil(n/3)` and `D = 3*ceil(E/4)`. The supported range is `0 <= n <= 3*floor(S/4)`. This is equivalent to `E <= S`, the extent limit of a valid padded byte destination, rather than a definition based on which inputs the codec accepts. The [capacity theorem](reproduce/base64-verification/src/round_trip_proof.rs#L5-L17) proves the arithmetic connection and `n <= D <= n+2`; also `D <= E <= S`.

Given valid caller-provided buffers of lengths at least E and D, `encoded_len(n, true)` returns `Some(E)` and `decoded_len_estimate(E)` returns D. Encoding returns `Ok(E)` and writes exactly `standard_encoding(x)`. Decoding only `encoded_output[..E]` returns `Ok(n)` and writes exactly x in its original order and multiplicity. Neither helper failure, codec error nor codec panic is an allowed substitute in this domain. Initial destination contents cannot affect the recovered prefix. Encoding preserves its destination after E; decoding preserves its destination after n, including any estimate slack. Input bytes and the encoded prefix remain unchanged under sound shared-reference semantics.

The [open byte definitions and Result contracts](reproduce/base64-verification/src/round_trip_spec.rs#L7-L85) cover empty input, zero and non-UTF-8 bytes, every remainder and all processing-block boundaries. No allocation existence is promised at the largest representable sizes: the result applies when those valid buffers are supplied.

Borrowed byte slices discharge the conversion and cleanup conditions through the slice/reference instances. Owned generic calls additionally require the actual conversion and complete recursive cleanup to return normally and preserve the specified observations and capacities. Their guarantee concerns this invocation's observed conversion, not the object's earlier contents or a second conversion. An owned decoder argument must itself yield the exact encoded prefix. These conditions constrain caller code; they do not permit codec failure.

## Requested API coverage

| ID and requested API | Guarantee and source binding | Checked use and status |
| --- | --- | --- |
| TOP-01 `Engine::encode_slice` | Exact E-byte canonical prefix, `Ok(E)`, input preservation and suffix frame. [Production](../base64-v4-E/reproduce/base64-production/src/engine/mod.rs#L227-L255), [formal contract](reproduce/base64-verification/src/round_trip_spec.rs#L46-L72), [checked implementation](reproduce/base64-verification/src/engine/mod.rs#L255-L333). | [Borrowed and owned consumers](reproduce/base64-verification/src/round_trip_use.rs#L18-L133). Verified for borrowed bytes; conditional for owned generics. This is also GeneralPurpose's selected public encode method. |
| TOP-02 `Engine::decode_slice` | Exact recovered n bytes and `Ok(n)`, encoded-input preservation and destination frame. [Production](../base64-v4-E/reproduce/base64-production/src/engine/mod.rs#L386-L409), [formal contract](reproduce/base64-verification/src/round_trip_spec.rs#L56-L85), [checked implementation](reproduce/base64-verification/src/engine/mod.rs#L462-L496). | [All three consumers](reproduce/base64-verification/src/round_trip_use.rs#L5-L133). Verified for the borrowed reported prefix; conditional for owned generics. This is GeneralPurpose's selected public decode method. |
| TOP-03 `encoded_len` | Succeeds with exact padded size E throughout the supported domain. [Production](../base64-v4-E/reproduce/base64-production/src/encode.rs#L101-L129), [contract](reproduce/base64-verification/src/encode.rs#L110-L117), [implementation](reproduce/base64-verification/src/encode.rs#L118-L146). | [Helper-sized caller](reproduce/base64-verification/src/round_trip_use.rs#L18-L70) proves the returned value meets the actual encode precondition. Verified. |
| TOP-04 `decoded_len_estimate` | Computes sufficient D with at most two bytes of slack for canonical output. [Production](../base64-v4-E/reproduce/base64-production/src/decode.rs#L189-L193), [contract](reproduce/base64-verification/src/decode.rs#L203-L205), [implementation](reproduce/base64-verification/src/decode.rs#L206-L210). | The same checked helper-sized caller proves decoding capacity and consumes the exact reported prefix. Verified. |

## Usage and complete proof closure

The consumers establish STANDARD readiness through actual construction, obtain representation facts from valid slices, derive helper sizes, call the public wrappers, extract only the reported prefix and prove the final bytes/count/frame effect. They assume sufficient destination sizes, not codec success. The owned consumer returns a ghost observation of its single conversion. Existing public docs and arbitrary-data/prefix/frame tests supply the usage intent; these are verification-only witnesses rather than production application callers.

| Goal-relevant edge | Final disposition |
| --- | --- |
| Consumers → STANDARD/alphabet/config/tables | Checked initialization and constructor bodies establish readiness. |
| Wrappers → `as_ref_bytes` → AsRef | Checked extraction and wrappers; external interface and invocation conditions are TCB-02. |
| Sizing helpers → estimate constructor and capacity arithmetic | Checked bodies and consumed domain lemmas. |
| Encode wrapper → expanded padding helper → scalar internal encode → tail, 32/8-byte packing, tables and padding | Checked adapted actual bodies. The two opaque generic helpers are bypassed. |
| Encoder result → exact prefix → decoder image | Checked caller and composition lemmas, with concrete canonical byte definitions. |
| Decoder → estimate/internal decode → scalar callback → helper → complete-quads loops → chunk_8/chunk_4 | Checked bodies in the same full context. Actual loops consume the new bounds and chunk-preparation lemmas. |
| Decoder helper → terminal arithmetic/suffix/padding/metadata → error conversion | Checked bodies; bundled Into/From forwarding is TCB-04. |
| Canonical encoding → injectivity → universal decoding witness → recovered original | Checked proof support consumed by actual decoder and callers. No assumed inversion or acceptance theorem. |
| Prepared codec Result → complete cleanup → delivered Result | TCB-03 frames the whole destination after codec work and relates explicit to implicit destruction. |
| Allocation, unchecked, SIMD, streaming, other configurations and arbitrary external strings | Outside the requested closure; residual contracts supply no selected proof edge. |

The [crate root](reproduce/base64-verification/src/lib.rs#L323-L342) loads actual engine/codec, specification, proof and usage modules together. Full replay selects no modules or proof modes. The [boundary inventory and Audit account](evidence/t4/AUDIT.md) and [syntax scan](evidence/t4/boundary-syntax.txt) classify every remaining source boundary. There is no codec assume/admit, uninterpreted inversion theorem or added termination exemption. Abstract external trait methods describe the declared caller relation/environment; concrete engine predicates, image definitions and opaque preparation predicates have definitions and checked consumers.

## Source correspondence and trust ledger

Both live Git HEADs identify the original revision above; [correspondence checks](evidence/t4/correspondence.txt) confirm production is clean and the final decode file matches the accepted integrated snapshot. Historical `ac5ce...` metadata does not identify a different live HEAD. No production/API fix was made.

E's adaptations expand nested helper/padding calls, specialize the zero scalar callback, replace unsupported iteration with index loops, express big-endian packing explicitly, relocate equal-valued alphabet/config initializers, and add ghost contracts/snapshots/proofs. These retain the selected scalar algorithm and validation order. No proof-specific cfg hides selected executable behavior. Audit inspected full changed-file scope before individual functions. The prescribed `drift-check code-drift --before .production --after . --comparison correspondence --scope src` could not run because the executable is absent; full Git/source comparison was the fallback.

The [proof-only reorganization patch](evidence/t2/proof-reorganization.patch) adds three checked lemmas, makes the existing canonical predicate opaque and trims loop invariants. The [integrated maintenance patch](evidence/t2/integrated-maintenance.patch) computes each decode block's start once and reuses it in the slice and offset. Loop guards bound every old/new arithmetic intermediate by the valid input interval; endpoints, decoder offsets, writes, call order and error propagation agree, including partial writes on error. [Accepted equivalence reasoning](evidence/t3/COMPARISON.md) supports this manual transfer to production.

Each following trust condition is accepted for the final claim. Counts refer to groups, not individual library axioms.

| ID | Trusted proposition, scope and evidence |
| --- | --- |
| TCB-01 — core, direct, TOP-01–TOP-04 | [Valid-byte-slice boundary](reproduce/base64-verification/src/as_ref_spec.rs#L35-L40): slice extent is at most isize::MAX; valid provenance, borrowing and lifetimes supply local frames. Invalid references or aliasing invalidate the result. |
| TCB-02 — core/caller, direct, TOP-01–TOP-02 | [AsRef interface and instances](reproduce/base64-verification/src/as_ref_spec.rs#L5-L32): exact method matching, slice identity and reference forwarding; arbitrary caller conversion must return normally and preserve the specified observations. The relation concerns the one observed conversion; repeated-conversion stability and arbitrary-T purity are not assumed. |
| TCB-03 — correspondence/caller, direct, TOP-01–TOP-02 | [Complete cleanup adapter](reproduce/base64-verification/src/as_ref_spec.rs#L43-L52): recursive destruction returns normally and preserves the whole destination. Moving T once into `core::mem::drop` corresponds to production's implicit destruction after result preparation and before delivery. No duplicate drop, leak or early cleanup is introduced. This is manual correspondence plus an external frame, not a proof of arbitrary destructors or unwind behavior. |
| TCB-04 — vstd/core/std/alloc, direct, TOP-01–TOP-04 | Bundled slice, sequence, array, indexing, copying, get_mut, arithmetic, bitvector, Option/Result, callback, string and conversion models match execution. [Literal axiom use](reproduce/base64-verification/src/alphabet.rs#L245-L265) and [second use](reproduce/base64-verification/src/alphabet.rs#L460-L470) connect literal bytes; [checked error conversion](reproduce/base64-verification/src/decode.rs#L121-L133) uses bundled Into/From forwarding. These are library/interface assumptions, not trusted codec inversion. |
| TCB-05 — project/core, direct, TOP-01–TOP-02 | [Panic interface](reproduce/base64-verification/src/panic_spec.rs#L1-L13) matches core and requires false. The checked bodies establish codec assertion unreachability; this assumption cannot excuse a codec panic or caller conversion/destructor failure. |
| TCB-06 — toolchain, direct, TOP-01–TOP-02 | The supplied [marker-metasized.patch](../base64-v4-E/trust/verus-t7-marker-metasized.patch) removes the synthetic marker-associated MetaSized bound while retaining the real external prerequisites. Residual method-predicate comparison, ABI and safety matching limitations require manual exact-interface review. No general external-trait soundness theorem is claimed. |
| TCB-07 — toolchain/environment, direct, TOP-01–TOP-04 | Rust translation and borrowing, supplied Verus, macros, ghost erasure, bundled metadata and Z3 implement their advertised semantics. [Tool identities](evidence/t1/identities.log), [local integration disclosure](evidence/t1/INTEGRATION.md) and [runner source](../base64-v4-E/reproduce/run.sh) identify the stack and command route. The runner is a local replacement for unavailable packaged software, not the upstream runner. Independent direct replay checks its semantic result. |
| TCB-08 — correspondence, direct, TOP-01–TOP-04 | E's adaptations and the [final actual decode body](reproduce/base64-verification/src/engine/general_purpose/decode.rs#L323-L476) preserve production's selected observations. [Preserved E diff](evidence/t1/preserved/e-working.patch), [Ground correspondence account](evidence/t1/GROUND.md), reorganization and maintenance patches above support this transfer. This is manual source correspondence, not a machine-checked compiler-refinement or performance-equivalence theorem. |
| TCB-09 — project, residual | Seven external bodies are excluded: [encode_with_padding](reproduce/base64-verification/src/encode.rs#L70-L86), [encode_helper](reproduce/base64-verification/src/engine/general_purpose/mod.rs#L180-L200), and the five allocation/unchecked methods in [Engine](reproduce/base64-verification/src/engine/mod.rs#L173-L540). Selected checked expansions bypass the helpers; selected consumers use none of the other five contracts. These bodies are inventoried, not certified. |

## Verification and maintenance evidence

The selected workflow is:

```sh
source .system-proof/t1/adapter/env.sh
verus-run verify "$PWD" --package base64
```

The local runner invokes the supplied binary on the registered actual `src/lib.rs` with `--crate-type lib --edition 2021 --cfg 'feature="std"' --cfg 'feature="alloc"'`. This direct bundled-metadata configuration represents `--no-default-features --features std`; std entails alloc. Defaults and SIMD are disabled, lifetime checking is enabled, and there is no rlimit flag, per-function limit, solver option or module selector. This is direct verification, not an integrated Cargo proof.

The supplied unchanged stack is Verus `0.2026.09.29.d85f757.dirty`, tangruize/verus d85f7578 plus the disclosed patch; Rust `1.98.1 (48a229cea)`; bundled Z3 `4.16.0`. No verifier, vstd or production modification was made in this campaign. The earlier setup failures and replacement workflow are retained in the integration disclosure above.

Fresh Engineer replay reports 182 verified, 0 errors, exit 0 in [fresh-verification.log](evidence/t4/fresh-verification.log), with [process resources](evidence/t4/fresh-verification.resources). Independent Reviewer execution of the exact underlying supplied-Verus command also returned 182 verified, 0 errors, exit 0; its command/result are recorded in the Reviewer letter. The Reviewer's read-only workspace prevents runner record creation, so the direct invocation supplies the independent replay. Counts describe each full run separately and are never added together.

Original E has 177 verified and two resource-exhaustion errors at default limits. Its historical report states 179/0 with `--rlimit 20`; no new raised-limit run was made. Reorganization adds three checked proof bodies, explaining 179 to 182 without changing the theorem. One construction pass closed the loops but left an opaque-predicate caller assertion at 181/1; an explicit reveal completed 182/0. Audit made no substantive repair.

### METRIC-01 — Fresh Audit replay

182 verified, 0 errors; independent direct replay agrees. The Engineer's native process took 8.94 s wall and 595944 KiB maximum RSS. These are whole-process observations from the linked fresh log/resources, not individual-loop times.

### METRIC-02 — Default verification comparison

| Variant | Verified/errors | Process wall s | Maximum RSS KiB | Native decode_complete_quads SMT s / resource units |
| --- | --- | --- | --- | --- |
| Original E | 177/2 | 9.71 | 582828 | 3.659348 / 62,110,056 |
| E plus maintenance | 179/0 | 9.65 | 578560 | 3.016409 / 40,430,246 |
| Reorganized, executable unchanged | 182/0 | 9.65 | 579188 | 2.214618 / 42,112,942 |
| Reorganized plus maintenance | 182/0 | 9.09 | 578136 | 1.272218 / 19,221,946 |

[Accepted measurements and raw-artifact index](evidence/t2/PROVE.md) and [E maintenance experiment](evidence/t3/COMPARISON.md) supply these single observations. Native function totals include its loops and final-body query, but exclude extracted lemmas.

### METRIC-03 — Loop and extracted lemma costs

| Generated query | Original E: replay s / units | E plus maintenance | Reorganized original | Final maintenance variant |
| --- | --- | --- | --- | --- |
| 32-byte loop | 1.911483 / 30,000,000, canceled | 2.289 / 28,210,723 | 1.045876 / 17,839,943 | 0.842809 / 10,962,824 |
| 4-byte loop | 1.672643 / 30,000,000, canceled | 0.454 / 9,100,169 | 0.981368 / 21,119,341 | 0.337279 / 6,443,181 |
| canonical_quads_bounds | — | — | 0.007885 / 26,531 | 0.007850 / 26,531 |
| prepare_decode_32 | — | — | 0.087895 / 814,142 | 0.091733 / 814,142 |
| prepare_decode_4 | — | — | 0.015098 / 64,971 | 0.015184 / 64,971 |

Original loop spans are decode.rs:284 and :390; reorganized spans are :364 and :431. All reorganized query responses are unsat. Default Verus rlimit 10 generates the unchanged 30,000,000-unit query limit. The measurements replay the generated module SMT in its original query order with the supplied Z3 and echo markers only. They are serialized query replay times, not native individual-loop wall times or separate whole-proof results.

The extracted lemmas' native aggregate costs are 0.135729 s / 905,644 units before maintenance and 0.132564 s / 905,644 units afterward. Thus the function reduction does not hide unreported extracted work. Native parallel measurements and serialized replay times must not be added together. [Measurement provenance](evidence/t2/PROVE.md) links the JSON, source-query associations and replay artifacts.

### METRIC-04 — Maintenance proof repair cost

The index rewrite required zero proof lines, zero proof files and zero added repair lemmas on both E and the reorganized proof. [The preserved-E experiment](evidence/t3/COMPARISON.md) and integrated result agree. E's original failures were pre-existing budget exhaustion; the executable arithmetic rewrite itself changed solver behavior enough to close them. Greater observed headroom in the final proof demonstrates no repair-cost advantage for this experiment. Single timings do not establish a general speedup or robustness distribution.

### METRIC-05 — Construction and analysis history

One decomposition round and one explicit reveal correction produced the default-limit proof; no raised-limit attempt was used. [Attack decisions and controls](evidence/t1/ATTACK.md) retain four original-production normal/panicking-drop cases. Each calls the real library with one conversion. The normal cases deliver successful results; the panic cases finish correct writes and suffix preservation but prevent Result delivery. This refutes unconditional owned-generic whole-call completion, not codec inversion.

Analyze-function's required route was unavailable; the selected analyze-protocol preparation failed because its executable was absent. Neither counts as successful specialist analysis. Specdet was not selected because the byte/count/frame contracts and checked consumers left no live output-uniqueness alternative for it to resolve. The direct lifecycle controls were consumed to justify the caller cleanup condition. [Independent accepted Ground judgment](evidence/t1/REVIEW.md) records that decision.

### METRIC-06 — Aggregate campaign accounting

Unavailable: the [Audit evidence scope](evidence/t4/AUDIT.md) supplies no consolidated token, AIC, service-restart, whole-campaign timing or LOC ledger. No aggregate is inferred from task numbering or selected logs.

## Findings and remaining scope

No scalar codec defect was found, and production remains unchanged. The retained destructor counterexample explains the conditional owned-generic theorem. Keeping that explicit caller condition preserves arbitrary bytes, the full represented domain and mandatory codec success. Unconditional arbitrary-owned-code completion is refuted; changing the production API or narrowing all coverage to borrowed inputs was unnecessary.

The result covers STANDARD GeneralPurpose, scalar execution and the stated std+alloc configuration. It does not certify other alphabets/configurations, arbitrary malformed external strings, insufficient buffers, unrepresentable lengths, allocation internals, streaming, SIMD, interoperability, all RFC rules or timing behavior. Historical mixed-vstd/Cargo evidence supplies no selected proof edge. All positive production conclusions retain the enumerated interface, library, toolchain and manual correspondence conditions.

The independent Reviewer accepts audit.formal-evidence, audit.code-binding and audit.original-goal for this claim. Completion additionally requires exact report installation comparison and the installed registered Audit check. Run:

```sh
source .system-proof/t1/adapter/env.sh
python3 "$SYSTEM_PROOF_PACK/checks/check_stage.py" audit "$PWD" --production "$PWD/.production"
```

That check freshly replays the installed route and retains its verifier output at [audit-verification.log](evidence/audit-verification.log). Its format/link and replay checks supplement the independent judgment. The final handoff records its outcome and the Reviewer's installed-text comparison; this report does not predict either result.

The main evidence path is the [request](evidence/INPUT.md), API/source links above, [Ground scope and call graph](evidence/t1/GROUND.md), [proof reorganization and measurements](evidence/t2/PROVE.md), [maintenance equivalence and zero-repair comparison](evidence/t3/COMPARISON.md), [fresh Audit evidence](evidence/t4/AUDIT.md), [source boundary inventory](evidence/t4/boundary-syntax.txt), [production correspondence checks](evidence/t4/correspondence.txt) and fresh replay output.
