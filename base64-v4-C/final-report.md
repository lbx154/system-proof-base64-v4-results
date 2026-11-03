+++
schema = "system-proof/final-report-v1"
verdict = "conditional"
source_revision = "47fa113a5c309fd7dcf5618d0f738cc9056d4402"
active_implementation = "Original registered production, transferred from checked in-place behavior-preserving verification adaptations"
requested_top_level_coverage = "complete"
tcb_inventory = "complete"

[[top_level]]
id = "TOP-01"
symbol = "Engine::encode_slice"
explanation = "For every lawful actual observation, STANDARD returns Ok with exact padded bytes and count; the destination suffix is unchanged."
status = "conditional"
production = "../.production/src/engine/mod.rs#L227-L255"
specification = "../src/engine/mod.rs#L256-L272"
implementation = "../src/engine/mod.rs#L273-L400"
caller_status = "verification-only"
caller = "t31/caller.rs#L8-L62"

[[top_level]]
id = "TOP-02"
symbol = "Engine::decode_slice"
explanation = "Decoding the reported encoder prefix returns Ok with the original bytes and length, preserving the destination suffix."
status = "conditional"
production = "../.production/src/engine/mod.rs#L386-L409"
specification = "../src/engine/mod.rs#L541-L559"
implementation = "../src/engine/mod.rs#L560-L632"
caller_status = "verification-only"
caller = "t31/caller.rs#L8-L62"

[[top_level]]
id = "TOP-03"
symbol = "encoded_len"
explanation = "The checked public helper returns the exact padded encoded size when representable, and supplies capacity throughout the supported round-trip range."
status = "verified"
production = "../.production/src/encode.rs#L101-L133"
specification = "../src/round_trip_spec.rs#L6-L30"
implementation = "../src/encode.rs#L129-L158"
caller_status = "verification-only"
caller = "t31/caller.rs#L8-L62"

[[top_level]]
id = "TOP-04"
symbol = "decoded_len_estimate"
explanation = "The checked public estimate is three times the ceiling of encoded length divided by four, sufficient for every selected encoder image."
status = "verified"
production = "../.production/src/decode.rs#L189-L193"
specification = "../src/round_trip_spec.rs#L22-L33"
implementation = "../src/decode.rs#L204-L210"
caller_status = "verification-only"
caller = "t31/caller.rs#L8-L62"

[[tcb]]
id = "TCB-01"
kind = "verifier and environment"
origin = "toolchain"
explanation = "Pinned Verus, rustc, SMT, compiled library artifacts and machine semantics correctly implement the verification and erasure semantics. Normal primitive execution makes progress; invalid Rust, hardware faults and resource exhaustion are excluded."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
evidence = "../t80-handoff/replay/identity.json"

[[tcb]]
id = "TCB-02"
kind = "annotation erasure"
origin = "correspondence"
explanation = "The reviewed twin-entry macro checks the annotated entry and emits the corresponding ordinary original-arity body. Generated erased verification panic stubs are not executable codec proofs. Transfer is restricted to the selected inherited default methods with STANDARD."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "t29/verus/source/builtin_macros/src/attr_rewrite.rs#L1182-L1245"

[[tcb]]
id = "TCB-03"
kind = "inherited library models"
origin = "vstd"
explanation = "Pinned core/std/alloc/vstd models correctly describe slice indexing, subslicing, mutable frames, copying, arithmetic, Option/Result, array conversion and closure forwarding. These library implementations are not proved by this codec replay."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "t29/verus/source/vstd/std_specs/slice.rs#L53-L100"

[[tcb]]
id = "TCB-04"
kind = "external proof interface"
origin = "project"
explanation = "Every actual valid u8 slice has byte length at most isize::MAX, including valid unsafe-created references. This does not bound arbitrary sequences, justify invalid raw pointers, cover ZST sizes or promise allocation existence."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "../src/round_trip_proof.rs#L59-L68"

[[tcb]]
id = "TCB-05"
kind = "external trait and core interfaces"
origin = "core"
explanation = "Arbitrary AsRef observations may change and have external effects. The selected slice implementation returns the same slice, and reference AsRef delegates once. No stable View, purity, normal completion or ghost-token transport is supplied for arbitrary T."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../src/round_trip_proof.rs#L86-L104"

[[tcb]]
id = "TCB-06"
kind = "panic interface"
origin = "core"
explanation = "The assertion-failed interface is callable only under false. Verified supported calls must establish that this panic path is unreachable; it supplies no successful panic effect."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../src/round_trip_proof.rs#L106-L119"

[[tcb]]
id = "TCB-07"
kind = "big-endian input conversion"
origin = "std"
explanation = "u64::from_be_bytes returns the stated exact eight-byte big-endian value; inherited slice-to-array, TryInto and unwrap interfaces faithfully forward it. The project read_u64 body itself is checked."
goal_relevance = "direct"
used_by = ["TOP-01"]
source = "../src/read_word_proof.rs#L4-L15"

[[tcb]]
id = "TCB-08"
kind = "big-endian output conversion"
origin = "std"
explanation = "be32 and be64 execute the primitive to_be_bytes operation and return its exact word-to-byte relation. Actual decoder packing and copies are checked; only these primitive interfaces are external bodies."
goal_relevance = "direct"
used_by = ["TOP-02"]
source = "../src/engine/general_purpose/decoder_chunk_proof.rs#L34-L46"

[[tcb]]
id = "TCB-09"
kind = "boolean conversion"
origin = "std"
explanation = "usize::from(bool) returns exactly zero or one. Actual decoded-size arithmetic and forwarding bodies are checked."
goal_relevance = "direct"
used_by = ["TOP-02", "TOP-04"]
source = "../src/engine/general_purpose/decode.rs#L18-L20"

[[tcb]]
id = "TCB-10"
kind = "linear cell library"
origin = "vstd"
explanation = "PCell construction, read and write obey their pinned PointsTo ownership/value contracts, and tracked permissions erase faithfully. No modified LocalInvariant semantics or allocation-to-pool simulation is assumed."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "t29/verus/source/vstd/cell/pcell.rs#L83-L245"

[[tcb]]
id = "TCB-11"
kind = "production continuation correspondence"
origin = "correspondence"
explanation = "Accepted C1-C11 adaptations preserve configuration, arithmetic, scheduling, branches, packing, counts, writes and normal delivery for each execution’s own actual observation. This is a reviewed semantic argument over checked actual bodies, not automated Rust contextual equivalence or equal callback replies across builds."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "../t80-handoff/ADAPTATIONS.md#L1-L86"

[[tcb]]
id = "TCB-12"
kind = "selected caller correspondence"
origin = "correspondence"
explanation = "T74 specializes the closed prepared producer without changing its four observation sequence, field ownership, selected callback, decoder continuation or cleanup. It proves that concrete entry-to-event path, not universal callback transport."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../t74-handoff/EQUIVALENCE.md#L1-L79"

[[tcb]]
id = "TCB-13"
kind = "normal-return lifecycle and separation"
origin = "correspondence"
explanation = "Generic public delivery is conditioned on this invocation’s external conversion and full owned cleanup returning normally. Safe Rust reference separation preserves the immutable consumed bytes during codec execution and both destination frames through those external intervals. Arbitrary globals and consumed owned objects are not preserved."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../t49-handoff/LIFECYCLE.md#L9-L74"

[[tcb]]
id = "TCB-14"
kind = "opaque user-state example interfaces"
origin = "std"
explanation = "The registered Cell example exposes unconstrained reads and ordinary writes without promising next-event canonicality or state precision. Its safe library representation is trusted; finite envelopes are sufficient client facts and do not restrict the public actual-event theorem."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "t39/generic_use.rs#L6-L15"

[[tcb]]
id = "TCB-15"
kind = "unused external convenience APIs"
origin = "project"
explanation = "Engine encode/String, encode_string, decode/Vec, decode_vec and decode_slice_unchecked are source-wide external bodies outside the selected slice round-trip call graph. Their bodies are not accepted as verified and discharge no requested edge."
goal_relevance = "residual"
used_by = []
source = "../src/engine/mod.rs#L175-L661"

[[metric]]
id = "METRIC-01"
name = "Independent direct Audit replay"
value = "172 verified, 0 errors; exit 0; all selected pins stable"
source = "../t80-handoff/independent-review/result.json"

[[metric]]
id = "METRIC-02"
name = "Engineer direct Audit replay"
value = "172 verified, 0 errors; exit 0; full installed src/lib.rs"
source = "../t80-handoff/replay/command.json"

[[metric]]
id = "METRIC-03"
name = "Standardized Audit replay"
value = "172 verified, 0 errors; exit 0; broad source_stable=false"
source = "../t80-handoff/standardized-record/result.json"

[[metric]]
id = "METRIC-04"
name = "Current-context controls"
value = "Positive 3 verified, 0 errors; five intended faults rejected"
source = "../t80-handoff/diagnostics/commands.json"

[[metric]]
id = "METRIC-05"
name = "Installed API runtime checks"
value = "7 RFC4648 section10 vectors and 771 binary count/recovery/both-suffix cases passed"
source = "../t80-handoff/diagnostics/current-runtime.log"

[[metric]]
id = "METRIC-06"
name = "Selected identity closure"
value = "38 source inputs, six compiled tool/library artifacts and three macro inputs unchanged"
source = "../t80-handoff/independent-review/after.json"

[[metric]]
id = "METRIC-07"
name = "Production preservation"
value = "Revision 47fa113a5c309fd7dcf5618d0f738cc9056d4402; clean; every tracked production byte unchanged"
source = "../t80-handoff/production-identity-after.json"

[[metric]]
id = "METRIC-08"
name = "Executable drift inventory"
value = "Exit 2; incomplete parser observation at src/encode.rs:190; retained independent source inspection"
source = "../t80-handoff/independent-review/code-drift.json"

[[metric]]
id = "METRIC-09"
name = "Historical raw archive availability"
value = "T74/T76 original temporary raw directories absent; zero historical raw copies; no reconstruction"
source = "../t80-handoff/HANDOFF.md"

[[metric]]
id = "METRIC-10"
name = "Campaign cost and aggregate reliability"
value = "Token usage, AIC, aggregate timeouts/restarts and formal LOC unavailable in this Audit package; no estimates synthesized"
source = "../t80-handoff/HANDOFF.md"
+++
# STANDARD round-trip Audit report

## Result at a glance

| Item | Accepted result |
| --- | --- |
| Verdict | Conditional proof accepted for all three Audit items; report installation/hash confirmation and the required Audit stage check remain completion steps. |
| Original goal | Preserve arbitrary binary bytes through real encode_slice, its reported prefix and decode_slice, with exact lengths, unchanged input and both suffix frames. |
| Production revision | 47fa113a5c309fd7dcf5618d0f738cc9056d4402, unchanged and clean. |
| Active implementation | Original production via the operator-authorized behavior-preserving verification adaptation route; no production repair. |
| Named TOP | TOP-01 Engine::encode_slice; TOP-02 Engine::decode_slice; TOP-03 encoded_len; TOP-04 decoded_len_estimate. |
| Checked caller | Verification-only source-grounded t31 caller; generic event clients and the separate concrete T74 specialization also checked. |
| Independent result | Full installed src/lib.rs: 172 verified, 0 errors, exit 0. Counts from separate runs are not added. |
| Bugs and repairs | No confirmed production codec bug in the accepted scope. Specification/event and compiler integration repairs remain explicit. |
| Trust ledger | 14 direct entries and one residual entry; grouped inherited library/semantic trust is explicitly declared. |
| Lifecycle | Generic conversion and complete owned cleanup must return normally; selected codec continuation has checked finite progress and no permitted codec error/panic. |

## Original goal and final claim

The authoritative [objective](effective-objective.txt) requests lossless STANDARD slice encoding/decoding, not a complete RFC or malformed-input audit. The selected configuration disables default features, enables std with its implied alloc, and uses no SIMD. The independently replayed platform is linux_x86_64.

Let b be the immutable byte slice actually returned by one invocation's single AsRef conversion. For 0 <= n = |b| <= 3 floor(isize::MAX/4), sufficient existing destinations and normal external conversion/cleanup, encoding returns Ok with E(n) = 4 ceil(n/3) canonical bytes. Decoding exactly that reported prefix with D(E(n)) capacity, where D(m) = 3 ceil(m/4), returns Ok with n bytes equal to b. Original destination bytes cannot influence those prefixes, and both destination suffixes remain unchanged. Empty, zero, non-UTF-8 and all block/residue boundary inputs are included. Internal errors, assertion/unwrap failure and codec panic are not extra permitted outcomes.

The bound follows from valid byte-slice representation and padded destination capacity, not whichever values the codec accepts. Its 64-bit maximum is 6,917,529,027,641,081,853 bytes. The formula gives 1,610,612,733 for 32-bit representation; this Audit did not independently replay a 32-bit target. Maximum allocation existence is not promised. Public encoded_len has broader integer Option semantics; integer representability alone does not imply a physically obtainable destination.

The public [open meaning](../src/round_trip_spec.rs#L6-L98) is reply-uniform within each invocation. An earlier preview, timeless admitted-reply relation or arbitrary other callback reply is not substituted for b. Changing callbacks may choose different b in production and adapted builds; the reviewed transfer establishes the same per-event guarantee for each lawful b separately. T74 does not narrow that generic theorem. Safe input preservation concerns the consumed bytes during the codec interval, not preservation of a consumed owned object's representation after arbitrary user Drop.

## Requested TOP coverage

| ID / requested symbol | Guarantee and status | Production | Formal public meaning | Checked implementation | Checked caller |
| --- | --- | --- | --- | --- | --- |
| TOP-01 `Engine::encode_slice` | For every lawful actual observation, STANDARD returns Ok with exact padded bytes and count; the destination suffix is unchanged. Status: conditional under the declared conditions. | [original](../.production/src/engine/mod.rs#L227-L255) | [contract](../src/engine/mod.rs#L256-L272) | [actual body](../src/engine/mod.rs#L273-L400) | [verification-only t31](t31/caller.rs#L8-L62) |
| TOP-02 `Engine::decode_slice` | Decoding the reported encoder prefix returns Ok with the original bytes and length, preserving the destination suffix. Status: conditional under the declared conditions. | [original](../.production/src/engine/mod.rs#L386-L409) | [contract](../src/engine/mod.rs#L541-L559) | [actual body](../src/engine/mod.rs#L560-L632) | [verification-only t31](t31/caller.rs#L8-L62) |
| TOP-03 `encoded_len` | The checked public helper returns the exact padded encoded size when representable, and supplies capacity throughout the supported round-trip range. Status: verified under the declared conditions. | [original](../.production/src/encode.rs#L101-L133) | [contract](../src/round_trip_spec.rs#L6-L30) | [actual body](../src/encode.rs#L129-L158) | [verification-only t31](t31/caller.rs#L8-L62) |
| TOP-04 `decoded_len_estimate` | The checked public estimate is three times the ceiling of encoded length divided by four, sufficient for every selected encoder image. Status: verified under the declared conditions. | [original](../.production/src/decode.rs#L189-L193) | [contract](../src/round_trip_spec.rs#L22-L33) | [actual body](../src/decode.rs#L204-L210) | [verification-only t31](t31/caller.rs#L8-L62) |

GeneralPurpose::encode_slice and GeneralPurpose::decode_slice inherit these selected Engine default methods; they are not separate override bodies. STANDARD's actual constructors, alphabet, encoding/inverse tables and padding/settings establish the concrete engine predicates consumed by those contracts.

## Verified usage and proof closure

The [common verifier root](../src/lib.rs#L291-L362) registers the actual targets, supporting proof modules and all selected callers together. Product cfg values select std/alloc; no proof-mode switch changes contracts or resources. The full run has no verify-only selector. Support proofs are consumed by actual body assertions and contracts, rather than replacing the codec.

| Material edge | Audit disposition |
| --- | --- |
| Caller -> public sizing -> physical capacities | Checked sizing bodies and arithmetic lemmas; valid-byte-slice bound is TCB-04. Caller derives sufficient lengths from actual sizing values and supplied existing slices. |
| STANDARD -> alphabet/config/table constructors | Checked concrete project bodies/witnesses establish symbol61, canonical encoding table, inverse decoding table and decoder readiness. |
| Generic public encode -> one conversion -> lexical wrapper/padding continuation | Checked actual annotated wrapper; captured event follows the conversion immediately. TCB-02/05/11 cover erasure, library observation and original-source transfer. |
| Encoder continuation -> internal_encode -> scalar groups/tail -> read_u64/add_padding | Checked project bodies establish exact count, byte content and full frame; TCB-07 covers only standard conversion. Zero-prefix callback executes once and changes no bytes. |
| Returned written count -> &encoded[..written] -> public decode | Checked t31 caller uses only the encoder's prefix, derives canonical image and decoded capacity, unwraps mandatory success and asserts byte equality/input preservation/both frames. |
| Public estimate -> GeneralPurposeEstimate::new -> forwarding | Checked actual sizing chain; TCB-09 covers bool conversion only. |
| Public decode -> actual event -> internal_decode -> boundary selection | Checked actual wrappers/forwarding and canonical-image boundary facts. Capacity is not assumed to excuse an error. |
| Decoder -> complete quads -> chunk4/chunk8 | Checked loop composition, byte-group recovery and complete destination frames; TCB-08 covers primitive endian output only. |
| Decoder -> suffix -> terminal bits/padding -> metadata/map/From | Checked actual adapted suffix plus consumed canonical-terminal/bit facts; metadata projection and actual From<DecodeError> body preserve variants/payloads. No remaining P-T25 or P-READ-U64 effect assumption. Library Result/closure forwarding is TCB-03. |
| Generic changing/variable-prefix clients | Checked event consumers derive obtainable bounds from actual source lengths/envelopes. Their sufficient envelopes do not become public restrictions. Opaque state interfaces provide no prediction of the next event. |
| Prepared producer -> actual canonical event -> decoder | Checked [T74 direct specialization](t74/specialized.rs#L6-L82), with real PointsTo and four observations; reviewed concrete correspondence is TCB-12. No universal Release/Reclaim or arbitrary callback-transport theorem is asserted. |
| Owned conversion/cleanup -> delivery and frames | TCB-13 normal-return and safe-reference separation composition; borrowed t31 inputs and concrete T74 fields have no custom Drop. |
| Convenience/streaming/SIMD/other engines | Out of scope; no unverified body from these paths discharges a selected edge. |

The detailed [call graph and boundary disposition](../t80-handoff/CALL-GRAPH.md) and [located inventory](../t80-handoff/replay/boundaries.json) are reviewed indices, not verification oracles. No active project assume/admit, broadcast axiom declaration or uninterpreted TOP function occurs in the selected sources. Opaque/closed specifications have definitions; empty proof bodies are verifier-checked obligations, not admissions. Imported axiomatic/library contracts remain TCB-03, including the array conversion axiom used by checked read_u64.

## Source identity and correspondence

[Independent before](../t80-handoff/independent-review/before.json) and [after](../t80-handoff/independent-review/after.json) identify all 38 selected inputs, six compiled verifier/library artifacts and three macro inputs. The source registration includes seven path-based caller files; all actual target/support source and relevant imported vstd/builtin artifacts are covered. Product-excluded tests/SIMD source is also pinned. Remaining Rust/core/std runtime semantics and toolchain behavior are declared trust, not silently unhashed project dependencies.

The [production identity](../t80-handoff/production-identity.json) and [after check](../t80-handoff/production-identity-after.json) preserve every tracked production byte and the fixed revision. T80 made evidence changes only. The [raw source diff](../t80-handoff/source.diff), [C1-C11 argument](../t80-handoff/ADAPTATIONS.md#L1-L86), accepted target lineage and actual inspected bodies classify executable expansions/counting-loop/endian/suffix rewrites as operator-authorized Verus adaptations. No new product algorithm or repair is claimed.

The independent [code-drift result](../t80-handoff/independent-review/code-drift.json) returned exit2 because src/encode.rs:190 could not be parsed by its restricted projection. It is incomplete, not a clean correspondence certificate. Direct inspection compared the unchanged padding range/write/return to production. The other located differences were assessed against the explicit adaptations and stable accepted source lineage. Token equality and finite comparisons support, but do not replace, the reviewed semantic continuation argument.

## TCB ledger

All 14 direct entries are accepted explicitly for this scoped result; TCB-15 is the residual source-wide entry. This ledger counts declared groups, not every imported vstd theorem individually. A defect in a direct trusted proposition may invalidate the conclusion. Codec success counts do not prove the compiler or foundational library models.

| ID | Origin / kind | Trusted proposition, use and risk | Exact source/evidence |
| --- | --- | --- | --- |
| TCB-01 | toolchain; verifier and environment; direct | Pinned Verus, rustc, SMT, compiled library artifacts and machine semantics correctly implement the verification and erasure semantics. Normal primitive execution makes progress; invalid Rust, hardware faults and resource exhaustion are excluded. Affects: TOP-01, TOP-02, TOP-03, TOP-04. | [boundary](../t80-handoff/replay/identity.json) |
| TCB-02 | correspondence; annotation erasure; direct | The reviewed twin-entry macro checks the annotated entry and emits the corresponding ordinary original-arity body. Generated erased verification panic stubs are not executable codec proofs. Transfer is restricted to the selected inherited default methods with STANDARD. Affects: TOP-01, TOP-02. | [boundary](t29/verus/source/builtin_macros/src/attr_rewrite.rs#L1182-L1245) |
| TCB-03 | vstd; inherited library models; direct | Pinned core/std/alloc/vstd models correctly describe slice indexing, subslicing, mutable frames, copying, arithmetic, Option/Result, array conversion and closure forwarding. These library implementations are not proved by this codec replay. Affects: TOP-01, TOP-02, TOP-03, TOP-04. | [boundary](t29/verus/source/vstd/std_specs/slice.rs#L53-L100) |
| TCB-04 | project; external proof interface; direct | Every actual valid u8 slice has byte length at most isize::MAX, including valid unsafe-created references. This does not bound arbitrary sequences, justify invalid raw pointers, cover ZST sizes or promise allocation existence. Affects: TOP-01, TOP-02, TOP-03, TOP-04. | [boundary](../src/round_trip_proof.rs#L59-L68) |
| TCB-05 | core; external trait and core interfaces; direct | Arbitrary AsRef observations may change and have external effects. The selected slice implementation returns the same slice, and reference AsRef delegates once. No stable View, purity, normal completion or ghost-token transport is supplied for arbitrary T. Affects: TOP-01, TOP-02. | [boundary](../src/round_trip_proof.rs#L86-L104) |
| TCB-06 | core; panic interface; direct | The assertion-failed interface is callable only under false. Verified supported calls must establish that this panic path is unreachable; it supplies no successful panic effect. Affects: TOP-01, TOP-02. | [boundary](../src/round_trip_proof.rs#L106-L119) |
| TCB-07 | std; big-endian input conversion; direct | u64::from_be_bytes returns the stated exact eight-byte big-endian value; inherited slice-to-array, TryInto and unwrap interfaces faithfully forward it. The project read_u64 body itself is checked. Affects: TOP-01. | [boundary](../src/read_word_proof.rs#L4-L15) |
| TCB-08 | std; big-endian output conversion; direct | be32 and be64 execute the primitive to_be_bytes operation and return its exact word-to-byte relation. Actual decoder packing and copies are checked; only these primitive interfaces are external bodies. Affects: TOP-02. | [boundary](../src/engine/general_purpose/decoder_chunk_proof.rs#L34-L46) |
| TCB-09 | std; boolean conversion; direct | usize::from(bool) returns exactly zero or one. Actual decoded-size arithmetic and forwarding bodies are checked. Affects: TOP-02, TOP-04. | [boundary](../src/engine/general_purpose/decode.rs#L18-L20) |
| TCB-10 | vstd; linear cell library; direct | PCell construction, read and write obey their pinned PointsTo ownership/value contracts, and tracked permissions erase faithfully. No modified LocalInvariant semantics or allocation-to-pool simulation is assumed. Affects: TOP-01, TOP-02. | [boundary](t29/verus/source/vstd/cell/pcell.rs#L83-L245) |
| TCB-11 | correspondence; production continuation correspondence; direct | Accepted C1-C11 adaptations preserve configuration, arithmetic, scheduling, branches, packing, counts, writes and normal delivery for each execution’s own actual observation. This is a reviewed semantic argument over checked actual bodies, not automated Rust contextual equivalence or equal callback replies across builds. Affects: TOP-01, TOP-02, TOP-03, TOP-04. | [boundary](../t80-handoff/ADAPTATIONS.md#L1-L86) |
| TCB-12 | correspondence; selected caller correspondence; direct | T74 specializes the closed prepared producer without changing its four observation sequence, field ownership, selected callback, decoder continuation or cleanup. It proves that concrete entry-to-event path, not universal callback transport. Affects: TOP-01, TOP-02. | [boundary](../t74-handoff/EQUIVALENCE.md#L1-L79) |
| TCB-13 | correspondence; normal-return lifecycle and separation; direct | Generic public delivery is conditioned on this invocation’s external conversion and full owned cleanup returning normally. Safe Rust reference separation preserves the immutable consumed bytes during codec execution and both destination frames through those external intervals. Arbitrary globals and consumed owned objects are not preserved. Affects: TOP-01, TOP-02. | [boundary](../t49-handoff/LIFECYCLE.md#L9-L74) |
| TCB-14 | std; opaque user-state example interfaces; direct | The registered Cell example exposes unconstrained reads and ordinary writes without promising next-event canonicality or state precision. Its safe library representation is trusted; finite envelopes are sufficient client facts and do not restrict the public actual-event theorem. Affects: TOP-01, TOP-02. | [boundary](t39/generic_use.rs#L6-L15) |
| TCB-15 | project; unused external convenience APIs; residual | Engine encode/String, encode_string, decode/Vec, decode_vec and decode_slice_unchecked are source-wide external bodies outside the selected slice round-trip call graph. Their bodies are not accepted as verified and discharge no requested edge. Affects: no requested TOP. | [boundary](../src/engine/mod.rs#L175-L661) |

The reviewed T3 initializer-erasure patch and T29 closed-array signature repair are part of TCB-01, with the exact adapted artifacts pinned; the [T3 patch](../t3-handoff/compiler.patch), [T29 combined compiler patch](../t29-handoff/compiler-combined.patch) and [vstd delta](../t29-handoff/vstd-delta.patch) retain the trusted changes. They are not codec proofs. T47 macro source pins are part of TCB-02. Standard library models include references/moves/destruction, array TryFrom/TryInto and Option/Result forwarding; the [conversion interfaces](t29/verus/source/vstd/std_specs/convert.rs#L1-L115) and [copy interface](t29/verus/source/vstd/std_specs/slice.rs#L567-L573) are representative exact consumed boundaries. No arbitrary Engine override or modified invariant simulation is accepted.

## Verification and Audit evidence

The independent [command](../t80-handoff/independent-review/command.json) ran in the actual shared verification root:

```text
.system-proof/t29/verus/source/target-verus/release/verus --edition=2021 --crate-type lib --cfg 'feature="std"' --cfg 'feature="alloc"' --rlimit 20 -V spinoff-all --triggers-mode silent src/lib.rs
```

The [independent full output](../t80-handoff/independent-review/full.log) and [result](../t80-handoff/independent-review/result.json) report 172 verified/0 errors, exit0 with stable identities. The [Engineer direct output](../t80-handoff/replay/full.log) is a distinct full run. Both check the installed actual candidate and the same contract/resource context.

The separate [standardized command](../t80-handoff/standardized-command.json), `verus-run verify /fastdata/v-boxiuli/noah/exp/tsC/verification --package base64`, retains [output](../t80-handoff/standardized-record/output.log), [manifest](../t80-handoff/standardized-record/manifest.json) and [result](../t80-handoff/standardized-record/result.json). It selects the same source/std/alloc/rlimit20/pinned route with normal checks, omitting only spinoff-all. Its broad Git fingerprint changed as evidence/untracked files changed; source_stable=false and [unstable-source observation](../t80-handoff/standardized-observation.json) remain verbatim. Exact selected source/library/macro closure is independently stable. Therefore the completed172/0 result supports this pinned candidate without asserting broad repository stability. Supplemental [runner validation behavior](../t80-handoff/runner-validation-excerpt.txt) is not Audit acceptance authority.

The exact independent original artifact bytes were transferred through the Reviewer journal letter and [T81 provenance](../t81-handoff/provenance.json). All seven installed members matched the Reviewer originals byte for byte, including the six-file manifest. Historical missing raw directories and the overwritten supplemental failure log remain disclosed rather than reconstructed. The original failed runtime import command and surviving compiler output remain in [commands](../t80-handoff/diagnostics/commands.json) and [driver output](../t80-handoff/diagnostics-driver.log). Successful installed API runtime checks are not prepared-client comparisons.

## Findings, bugs and repairs

No confirmed in-scope production codec counterexample was found. The prior timeless generic-domain defect was repaired by invocation-specific event capture; current contracts do not assume purity or a canonical next observation. Unsupported compiler constant/trait/endian integration and iterator/continue modeling prompted explicitly reviewed compiler/source adaptations. Abandoned dictionary/ownership simulation investigations supply no theorem and are not hidden prerequisites.

Finite RFC/vector/runtime checks corroborate exact bytes/counts/frames but prove neither universal equivalence nor standards compliance. Fresh controls reject wrong initialization, missing preparation skip, wrong event selection, extra conversion and stale prepared state. Those controls test the selected T74 source path; they do not prove universal callback transport.

## Lifecycle, reliability, scale and cost

| ID / metric name | Evidence-sourced value | Source |
| --- | --- | --- |
| METRIC-01: Independent direct Audit replay | 172 verified, 0 errors; exit 0; all selected pins stable | [record](../t80-handoff/independent-review/result.json) |
| METRIC-02: Engineer direct Audit replay | 172 verified, 0 errors; exit 0; full installed src/lib.rs | [record](../t80-handoff/replay/command.json) |
| METRIC-03: Standardized Audit replay | 172 verified, 0 errors; exit 0; broad source_stable=false | [record](../t80-handoff/standardized-record/result.json) |
| METRIC-04: Current-context controls | Positive 3 verified, 0 errors; five intended faults rejected | [record](../t80-handoff/diagnostics/commands.json) |
| METRIC-05: Installed API runtime checks | 7 RFC4648 section10 vectors and 771 binary count/recovery/both-suffix cases passed | [record](../t80-handoff/diagnostics/current-runtime.log) |
| METRIC-06: Selected identity closure | 38 source inputs, six compiled tool/library artifacts and three macro inputs unchanged | [record](../t80-handoff/independent-review/after.json) |
| METRIC-07: Production preservation | Revision 47fa113a5c309fd7dcf5618d0f738cc9056d4402; clean; every tracked production byte unchanged | [record](../t80-handoff/production-identity-after.json) |
| METRIC-08: Executable drift inventory | Exit 2; incomplete parser observation at src/encode.rs:190; retained independent source inspection | [record](../t80-handoff/independent-review/code-drift.json) |
| METRIC-09: Historical raw archive availability | T74/T76 original temporary raw directories absent; zero historical raw copies; no reconstruction | [record](../t80-handoff/HANDOFF.md) |
| METRIC-10: Campaign cost and aggregate reliability | Token usage, AIC, aggregate timeouts/restarts and formal LOC unavailable in this Audit package; no estimates synthesized | [record](../t80-handoff/HANDOFF.md) |

Separate runs and focused scopes are never added to create a larger proof count. No exact campaign-wide cost or reliability summary was supplied. Ground source-bound specification attacks informed the actual-event and lawful-domain decisions; T80's consumed specialist evidence is the partial drift inventory plus independent source inspection, not a newly run generic state simulation.

## Remaining limitations and completion protocol

The proof is conditional on the explicitly accepted TCB and T49 normal-return scope. It does not establish arbitrary user AsRef/Drop termination or purity, preservation of user-global effects or consumed owned objects, cross-build callback reply equality, Rust contextual equivalence for stack/backtrace/timing observations, resource-exhaustion behavior, unsafe-invalid references, maximal allocation availability, arbitrary Engine overrides, malformed external encoded strings, insufficient buffers, unrepresentable lengths, streaming, SIMD or alternate alphabets/settings. These exclusions do not permit a supported codec error or panic.

All three protected Audit items are accepted for the selected claim. Campaign completion additionally requires exact installation of this Reviewer-authored report, independent installed hash confirmation, and successful `check_stage audit` with registered production, including its fresh preserved verifier replay. This report does not assert that those later completion actions have already passed.

## Evidence map

The [requested objective](effective-objective.txt), [public TOP](../src/round_trip_spec.rs#L6-L98), [checked usage](t31/caller.rs#L8-L62), [generic actual-event use](t47/event_use.rs#L6-L102), [T74 source](t74/specialized.rs#L6-L82), [call graph](../t80-handoff/CALL-GRAPH.md), [boundary inventory](../t80-handoff/replay/boundaries.json), [source diff](../t80-handoff/source.diff), [independent output](../t80-handoff/independent-review/full.log), [transfer provenance](../t81-handoff/provenance.json) and [package handoff](../t80-handoff/HANDOFF.md) are the retained audit index. Older READINESS/Ground documents are historical indices; their prior phase/P-CALLER status is not the final T80 disposition.
