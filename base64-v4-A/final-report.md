+++
schema = "system-proof/final-report-v1"
verdict = "conditional"
source_revision = "5d574f8ad899aa9168aa0d25ad03ef634fc4ae79"
active_implementation = "Original production scalar STANDARD algorithms; frozen t15 verification elaboration; independent t18 Audit acceptance under the declared TCB conditions"
requested_top_level_coverage = "complete"
tcb_inventory = "complete"

[[top_level]]
id = "TOP-01"
symbol = "Engine::encode_slice"
explanation = "On STANDARD, every lawful byte input with representable padded length and sufficient destination succeeds, returns exactly 4*ceil(n/3), writes precisely the defined encoding and preserves the entire destination suffix. The ordinary slice input is unchanged."
status = "verified"
production = "../../production/src/engine/mod.rs#L227-L253"
specification = "../src/roundtrip_spec.rs#L1-L85"
implementation = "t17/fresh/scalar-probe/mod.rs.bridge#L242-L254"
caller_status = "verification-only"
caller = "t17/fresh/scalar-probe/caller.rs.bridge#L1-L33"

[[top_level]]
id = "TOP-02"
symbol = "Engine::decode_slice"
explanation = "On STANDARD, decoding exactly an encoder-produced prefix into a lawful buffer at least the public estimate succeeds, returns the original length, recovers every original byte and preserves the entire destination suffix."
status = "verified"
production = "../../production/src/engine/mod.rs#L386-L409"
specification = "../src/roundtrip_spec.rs#L1-L85"
implementation = "t17/fresh/scalar-probe/mod.rs.bridge#L387-L397"
caller_status = "verification-only"
caller = "t17/fresh/scalar-probe/caller.rs.bridge#L1-L33"

[[top_level]]
id = "TOP-03"
symbol = "encoded_len"
explanation = "With padding=true, returns Some(4*ceil(n/3)) exactly when that result fits usize, otherwise None. The helper is independent of buffer allocation."
status = "verified"
production = "../../production/src/encode.rs#L101-L132"
specification = "../src/roundtrip_spec.rs#L1-L85"
implementation = "t17/fresh/scalar-probe/encode.rs.bridge#L126-L160"
caller_status = "verification-only"
caller = "t17/fresh/scalar-probe/caller.rs.bridge#L1-L33"

[[top_level]]
id = "TOP-04"
symbol = "decoded_len_estimate"
explanation = "Returns exactly 3*ceil(m/4), conservatively sufficient for every selected encoder-produced prefix; its checked constructor arithmetic is representable for all usize input lengths."
status = "verified"
production = "../../production/src/decode.rs#L189-L193"
specification = "../src/roundtrip_spec.rs#L1-L85"
implementation = "t17/fresh/scalar-probe/decode.rs.bridge#L198-L202"
caller_status = "verification-only"
caller = "t17/fresh/scalar-probe/caller.rs.bridge#L1-L33"

[[tcb]]
id = "TCB-01"
kind = "external trait specification and uninterpreted relation"
origin = "core"
explanation = "The one actual AsRef invocation returns a reference permitted by converted; arbitrary T has only normal-return, nondeterministic conversion semantics. No repeated-conversion equality, termination or Drop purity is assumed for arbitrary user types."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "t17/fresh/scalar-probe/mod.rs.bridge#L576-L593"

[[tcb]]
id = "TCB-02"
kind = "external_body slice identity lemma"
origin = "core"
explanation = "For T=&[u8], reference-forwarding and slice identity return the same byte contents, complete without user code or storage writes, and trivial reference Drop preserves the caller observation. Safe borrowing supplies disjoint mutable destinations."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "t17/fresh/scalar-probe/mod.rs.bridge#L584-L593"

[[tcb]]
id = "TCB-03"
kind = "external function specification"
origin = "core"
explanation = "usize::from(bool) is exactly zero for false and one for true; this is a primitive conversion interface, not a proof of core or a codec-success premise."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-04"]
source = "t17/fresh/scalar-probe/gp_decode.rs.bridge#L652-L652"

[[tcb]]
id = "TCB-04"
kind = "assume_specification"
origin = "core"
explanation = "u64::from_be_bytes returns the exact unsigned big-endian weighted sum of its eight bytes on the selected target; core endian conversion, layout and transmute behavior are trusted."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../t9-handoff/byte-order.rs#L1-L6"

[[tcb]]
id = "TCB-05"
kind = "assume_specification"
origin = "core"
explanation = "u32::to_be_bytes returns each of the four big-endian bytes of the input integer, independent of host endianness; core implementation is source-justified rather than checked."
goal_relevance = "direct"
used_by = ["TOP-02"]
source = "../t13-handoff/byte-order.rs#L3-L4"

[[tcb]]
id = "TCB-06"
kind = "assume_specification"
origin = "core"
explanation = "u64::to_be_bytes returns each of the eight big-endian bytes of the input integer, independent of host endianness; core implementation is source-justified rather than checked."
goal_relevance = "direct"
used_by = ["TOP-02"]
source = "../t13-handoff/byte-order.rs#L5-L6"

[[tcb]]
id = "TCB-07"
kind = "opaque error, uninterpreted conversion, broadcast axiom and assume_specification"
origin = "vstd"
explanation = "The fixed-array Copy slice TryFrom succeeds iff its length is N and copies every element; the error payload is opaque. Existing TryInto forwards through this specialized contract to core blanket TryInto, without a general callable axiom."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../t9-handoff/convert.rs.patch#L1-L38"

[[tcb]]
id = "TCB-08"
kind = "external opaque type and uninterpreted state with constructor axiom"
origin = "vstd"
explanation = "Finite Enumerate preserves inner order, returns zero-based offsets, advances its counter without overflow, inherits its inner termination metric and next/exhaustion laws. All selected slice lengths ensure its bounded-counter regime."
goal_relevance = "direct"
used_by = ["TOP-02"]
source = "../t12-handoff/iter.rs.patch#L1-L57"

[[tcb]]
id = "TCB-09"
kind = "external opaque type, uninterpreted state and assumed iterator interfaces"
origin = "vstd"
explanation = "Positive-width ChunksExact yields precisely the ordered floor-length source windows, preserves a fixed remainder through next and exhaustion, and obeys finite iterator head/pop and metric laws. Actual widths are positive 32 and 4."
goal_relevance = "direct"
used_by = ["TOP-02"]
source = "../t12-handoff/slice.rs.patch#L1-L38"

[[tcb]]
id = "TCB-10"
kind = "local compiler signature adaptation"
origin = "toolchain"
explanation = "Closed array constants may match only after both successfully normalize and evaluate to equal target-usize values; element types must still match exactly. Rust normalization and the retained t9 frontend patch must preserve this restriction. See the exact patch and signature regressions."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
evidence = "t17/fresh/registered-identities.json"

[[tcb]]
id = "TCB-11"
kind = "local macro and continue translator adaptation"
origin = "toolchain"
explanation = "Wrapper next advances ghost state once before the body, macro assertions check that fact, and continue executes ordinary entry-invariant and decreases checks on live state. The t14 lowering and its ghost erasure must preserve Rust control flow. Regression negatives support, but do not prove, translator soundness."
goal_relevance = "direct"
used_by = ["TOP-02"]
evidence = "t17/fresh/t14-regress.log"

[[tcb]]
id = "TCB-12"
kind = "foundational sequence and mathematical model axioms"
origin = "vstd"
explanation = "Imported Seq, arithmetic, extensionality, subrange and quantified mathematical interfaces must denote their documented mathematical objects consistently. Checked support lemmas are proofs relative to this foundation, not proofs of these axioms."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "../t14-handoff/toolchain/source/vstd/seq.rs#L1-L1954"

[[tcb]]
id = "TCB-13"
kind = "array primitive interfaces"
origin = "vstd"
explanation = "Array fill, index, update and View faithfully model fixed Rust arrays and preserve other elements; these interfaces support initialization, scalar read and chunk output."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../t14-handoff/toolchain/source/vstd/std_specs/array.rs#L1-L125"

[[tcb]]
id = "TCB-14"
kind = "slice, borrow, iterator and mutable prophecy interfaces"
origin = "vstd"
explanation = "Slice bounds, immutable views, mutable borrow final views, get_mut, copy_from_slice and slice iteration obey Rust contents and framing semantics. Final-view prophecy and Rust lifetime/alias translation must be faithful."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../t14-handoff/toolchain/source/vstd/std_specs/slice.rs#L1-L644"

[[tcb]]
id = "TCB-15"
kind = "Vec caller interfaces"
origin = "vstd"
explanation = "Vec length, view and as_slice/as_mut_slice agree with initialized lawful contiguous buffers and preserve unchanged elements across borrowed-slice calls. The caller does not allocate or resize."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../t14-handoff/toolchain/source/vstd/std_specs/vec.rs#L1-L488"

[[tcb]]
id = "TCB-16"
kind = "Option/Result, conversion and callable interfaces"
origin = "vstd"
explanation = "Option/Result successful unwrap, ok_or, map closure return contracts, question-mark propagation and trait dispatch follow ordinary Rust behavior; checked success excludes their failure/panic branches on the selected path."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "../t14-handoff/toolchain/source/vstd/std_specs/result.rs#L1-L284"

[[tcb]]
id = "TCB-17"
kind = "numeric, range, string and ordinary core interfaces"
origin = "vstd"
explanation = "Unsigned checked arithmetic and casts, range iteration, constants, string-literal ASCII byte views, equality, Copy/Clone, and boolean primitives match the selected Rust target. The remaining imported foundational interfaces are trusted at their declared contracts; no imported codec fact is assumed."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "../t14-handoff/toolchain/source/vstd/std_specs/num.rs#L1-L536"

[[tcb]]
id = "TCB-18"
kind = "selected extraction and dispatch correspondence"
origin = "correspondence"
explanation = "The CodecCaps/Engine partition, inner hoist, associated-type and UFCS remapping, ghost contracts, exec-constant spec counterparts and exact single-use suffix iterator temporary preserve the original GeneralPurpose defaults, arguments, call order and executable algorithm."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "t4/handoff.md#L109-L172"

[[tcb]]
id = "TCB-19"
kind = "compiler, verifier, SMT and binary identity"
origin = "toolchain"
explanation = "Selected Rust 1.98.1, Verus d85f757 plus pinned t9/t12/t14 patches, vstd, Z3, VC generation, bit-vector reasoning, borrow/lifetime translation and ghost erasure are sound. Retained binary/source/library manifests and filesystem hashes identify the executed stack; no tool rebuild was performed during this Audit."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
evidence = "t17/fresh/scalar-probe/run.json"

[[tcb]]
id = "TCB-20"
kind = "lawful memory and execution environment"
origin = "environment"
explanation = "Inputs and both initialized destination buffers actually exist as lawful live Rust allocations with required lengths and valid nonconflicting borrows; platform usize/isize and unsigned layout match the selected target. Allocation availability, hardware faults and process interruption are outside the theorem."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
evidence = "t17/post-replay-identity.json"

[[tcb]]
id = "TCB-21"
kind = "unrelated external_body encode"
origin = "project"
explanation = "Engine::encode has an inherited external body and no selected round-trip claim depends on it. It is excluded from the caller and goal-relevant call graph, and supplies no success or data premise."
goal_relevance = "residual"
used_by = []
source = "t17/fresh/scalar-probe/mod.rs.bridge#L161-L168"

[[tcb]]
id = "TCB-22"
kind = "unrelated external_body encode_string"
origin = "project"
explanation = "Engine::encode_string has an inherited external body and no selected round-trip claim depends on it. It is excluded from the caller and goal-relevant call graph, and supplies no success or data premise."
goal_relevance = "residual"
used_by = []
source = "t17/fresh/scalar-probe/mod.rs.bridge#L202-L209"

[[tcb]]
id = "TCB-23"
kind = "unrelated external_body decode"
origin = "project"
explanation = "Engine::decode has an inherited external body and no selected round-trip claim depends on it. It is excluded from the caller and goal-relevant call graph, and supplies no success or data premise."
goal_relevance = "residual"
used_by = []
source = "t17/fresh/scalar-probe/mod.rs.bridge#L276-L283"

[[tcb]]
id = "TCB-24"
kind = "unrelated external_body decode_vec"
origin = "project"
explanation = "Engine::decode_vec has an inherited external body and no selected round-trip claim depends on it. It is excluded from the caller and goal-relevant call graph, and supplies no success or data premise."
goal_relevance = "residual"
used_by = []
source = "t17/fresh/scalar-probe/mod.rs.bridge#L335-L342"

[[tcb]]
id = "TCB-25"
kind = "unrelated external_body decode_slice_unchecked"
origin = "project"
explanation = "Engine::decode_slice_unchecked has an inherited external body and no selected round-trip claim depends on it. It is excluded from the caller and goal-relevant call graph, and supplies no success or data premise."
goal_relevance = "residual"
used_by = []
source = "t17/fresh/scalar-probe/mod.rs.bridge#L414-L421"

[[tcb]]
id = "TCB-26"
kind = "residual imported library declarations"
origin = "vstd"
explanation = "Unused vstd external/opaque/uninterpreted/axiomatic declarations outside the primitive families used above have no selected caller edge. The complete textual library marker inventory is retained; library import alone is not a checked core implementation or an additional API claim."
goal_relevance = "residual"
used_by = []
source = "../t14-handoff/toolchain/source/vstd/vstd.rs#L1-L169"

[[metric]]
id = "METRIC-01"
name = "Fresh complete candidate replay"
value = "146 verified, 0 errors; exit 0"
source = "t17/fresh/scalar-probe/verify.log"

[[metric]]
id = "METRIC-02"
name = "Complete candidate elapsed time"
value = "58.640896 seconds; 2026-11-03T08:52:41.994117+00:00 to 2026-11-03T08:53:40.635013+00:00"
source = "t17/fresh/scalar-probe/run.json"

[[metric]]
id = "METRIC-03"
name = "Inherited regression suites"
value = "4 suites passed: t9 signature/conversion, t12 iterator/window, t13 byte order, t14 continue; intended negative failures are required outcomes"
source = "t17/fresh/regression-results.json"

[[metric]]
id = "METRIC-04"
name = "Audit implementation repairs"
value = "0; frozen t9\u2013t15 packages and inputs revalidated unchanged; production clean; phase audit"
source = "t17/post-replay-identity.json"

[[metric]]
id = "METRIC-05"
name = "Tool source correspondence"
value = "1464 original and 1464 adapted manifest entries checked; exactly six reviewed source adaptations; original installed source clean and binaries unchanged"
source = "t17/tool-identity.json"

[[metric]]
id = "METRIC-06"
name = "Audit resource setting"
value = "Suffix default; inherited scalar-local rlimit 80; no resource escalation"
source = "../t15-handoff/handoff.md"

[[metric]]
id = "METRIC-07"
name = "Selected snapshot scale"
value = "4236 lines across 9 selected installed source snapshots; includes docs and proof annotations, not formal LOC"
source = "t17/metrics.json"

[[metric]]
id = "METRIC-08"
name = "Campaign cost and reliability totals"
value = "Unavailable: no complete token/AIC, wall-time, restart, rollback or verifier-attempt accounting was supplied to this Audit; historical construction failures are retained but are not summed"
source = "t17/metrics.json"

[[metric]]
id = "METRIC-09"
name = "Specialist attack disposition"
value = "Historical t4 specdet inconclusive (incomplete quantifiers); no completeness acceptance inferred; checked writes uniqueness is consumed in the common context"
source = "t4/handoff.md"
+++

# Result at a glance

| Item | Audit candidate result |
| --- | --- |
| Verdict | Established under the explicit TCB below; independent t18 Reviewer acceptance recorded |
| Original goal | Mandatory successful, lossless STANDARD slice round trip for arbitrary bytes, with exact lengths and preserved input/destination suffixes |
| Production revision | `5d574f8ad899aa9168aa0d25ad03ef634fc4ae79`, clean before and after replay |
| Active implementation | Original scalar algorithms, with verification-only annotations/extraction; no product repair |
| Named TOP | Fully defined `roundtrip_spec::{encoded, writes, capacities, encode_guarantee, decode_guarantee}` |
| Checked caller | `standard_roundtrip`, verification-only source-grounded exact-prefix recipe |
| Fresh independent replay | 146 verified, 0 errors; actual source-path bodies and caller in one context |
| Bugs and repairs | No production counterexample established; no Audit proof/spec/tool repair |
| TCB inventory | 20 direct groups, 6 residual groups; grouping is not a count of individual vstd annotations |
| Lifecycle | Audit remains active; all three protected Audit items accepted by the t18 Reviewer; final report installation requires separate confirmation |

## Original goal, interpretation and claim

The independent t18 Reviewer accepted `audit.formal-evidence`, `audit.code-binding`, and `audit.original-goal`, explicitly accepting TCB-01 through TCB-20 as declared conditions and TCB-21 through TCB-26 as excluded residual surface. The original slice round-trip goal is established under those conditions. The frontmatter verdict remains `conditional` to express those unchanged trust conditions, rather than a pending semantic judgment. The exact acceptance, provenance and subsequent delivery correction are retained in [recorded t18 acceptance](../../t19-audit-delivery/acceptance.md) and [verbatim event records](../../t19-audit-delivery/t18-review-events.jsonl). This report records that independent judgment; report-text approval, installation confirmation and the Manager’s completion decision are separate delivery steps.

The [original request](INPUT.md) selects GeneralPurpose STANDARD, default features disabled, only `std` enabled with its implied `alloc`, and no SIMD. For any lawful immutable byte slice and two lawful initialized destinations satisfying the independent capacity formula, encoding and decoding the exact returned encoded prefix must both return Ok; decoded length and all bytes equal the input. Both complete untouched destination suffixes are preserved for arbitrary prior contents, and the input is unchanged. No internal error or panic substitutes for success. Empty input, zero/non-UTF-8 bytes and every length across scalar/unrolled/quad/suffix boundaries are quantified cases, not test samples.

The [fully defined TOP](../src/roundtrip_spec.rs#L1-L85) uses E(n)=4*ceil(n/3) and D(m)=3*ceil(m/4). If U=usize::MAX, the padded helper alone can represent exactly 0<=n<=3*floor(U/4), returning None above this bound. This is a helper arithmetic limit, not a promise that such slices can exist. Real byte slices and each destination have length at most I=isize::MAX. The helper-sized recipe requires n, E(n), D(E(n))<=I, equivalently 0<=n<=3*floor(I/4), and actual destination lengths enc>=E(n), dec>=D(E(n)), each <=I. For the selected 64-bit target the helper maximum is 13,835,058,055,282,163,709 bytes; the realizable-recipe arithmetic maximum is 6,917,529,027,641,081,853 bytes. At that endpoint E(n)=9,223,372,036,854,775,804 and D(E(n))=n. These arithmetic limits do not establish allocation availability; obtaining all buffers simultaneously with lawful initialized storage is an environmental condition. Larger capacities within slice limits are permitted, and suffix framing still holds.

The checked sizing bodies establish helper sufficiency, rather than assuming it. There is no input-content restriction, minimum block length, implementation-success premise, destination-zeroing requirement or error allowance. The caller requires only `capacities`; concrete initialization proves STANDARD/table/config premises. Arbitrary malformed strings, undersized/unrepresentable buffers, other engines/configurations, allocation internals, streaming and timing are outside the request.

## Requested TOP coverage

| ID and requested API | Established guarantee | Production | Formal TOP | Checked implementation | Checked caller | Status |
| --- | --- | --- | --- | --- | --- | --- |
| TOP-01 `Engine::encode_slice` | On STANDARD, every lawful byte input with representable padded length and sufficient destination succeeds, returns exactly 4*ceil(n/3), writes precisely the defined encoding and preserves the entire destination suffix. The ordinary slice input is unchanged. | [source](../../production/src/engine/mod.rs#L227-L253) | [TOP](../src/roundtrip_spec.rs#L1-L85) | [body](t17/fresh/scalar-probe/mod.rs.bridge#L242-L254) | [verification-only](t17/fresh/scalar-probe/caller.rs.bridge#L1-L33) | verified subject to TCB/review |
| TOP-02 `Engine::decode_slice` | On STANDARD, decoding exactly an encoder-produced prefix into a lawful buffer at least the public estimate succeeds, returns the original length, recovers every original byte and preserves the entire destination suffix. | [source](../../production/src/engine/mod.rs#L386-L409) | [TOP](../src/roundtrip_spec.rs#L1-L85) | [body](t17/fresh/scalar-probe/mod.rs.bridge#L387-L397) | [verification-only](t17/fresh/scalar-probe/caller.rs.bridge#L1-L33) | verified subject to TCB/review |
| TOP-03 `encoded_len` | With padding=true, returns Some(4*ceil(n/3)) exactly when that result fits usize, otherwise None. The helper is independent of buffer allocation. | [source](../../production/src/encode.rs#L101-L132) | [TOP](../src/roundtrip_spec.rs#L1-L85) | [body](t17/fresh/scalar-probe/encode.rs.bridge#L126-L160) | [verification-only](t17/fresh/scalar-probe/caller.rs.bridge#L1-L33) | verified subject to TCB/review |
| TOP-04 `decoded_len_estimate` | Returns exactly 3*ceil(m/4), conservatively sufficient for every selected encoder-produced prefix; its checked constructor arithmetic is representable for all usize input lengths. | [source](../../production/src/decode.rs#L189-L193) | [TOP](../src/roundtrip_spec.rs#L1-L85) | [body](t17/fresh/scalar-probe/decode.rs.bridge#L198-L202) | [verification-only](t17/fresh/scalar-probe/caller.rs.bridge#L1-L33) | verified subject to TCB/review |

GeneralPurpose has no public slice overrides. Its selected `GeneralPurpose::encode_slice` and `GeneralPurpose::decode_slice` calls resolve to the checked Engine defaults in TOP-01 and TOP-02; they are not additional omitted requested functions. The two requested sizing helpers are TOP-03 and TOP-04.

## Verified usage and closure

The [checked caller](t17/fresh/scalar-probe/caller.rs.bridge#L1-L33) calls both public helpers, proves output capacity assertions, calls the actual encode_slice, derives the exact reported prefix and passes precisely that slice to actual decode_slice. Its unwraps are justified by mandatory success contracts. Neither buffer is assumed empty or zeroed. Immutable input borrowing and the concrete slice conversion justify unchanged input. Vecs provide a lawful observation path into slices; this caller is verification-only, derived from the production encode_slice prefix doctest and decode sizing documentation, not an existing application claimed as production-verified.

For arbitrary generic T the wrapper invokes `as_ref` once. The conversion relation permits nondeterminism, and the universal wrapper precondition must cover all permitted converted byte slices. Its observation is the one actual conversion, sampled at the inner byte-slice entry. It does not assert arbitrary user T is pure, terminating or free of Drop effects. For the promised ordinary &[u8] caller, core reference forwarding and slice identity discharge these concerns and the universal premises reduce to the same lawful input/capacity condition. This preserves the actual generic signature without broadening the original goal to arbitrary user-code lifecycles.

| Goal-relevant path or data dependency | Audit disposition |
| --- | --- |
| STANDARD -> alphabet literal/PAD/config/GeneralPurpose constructor -> encode/decode table loops | Checked bodies, exact constructor expressions and concrete inverse/invalid-byte facts; no assumed initialization witness |
| encoded_len -> checked arithmetic; decoded_len_estimate -> actual estimate constructor/accessor | Checked bodies; estimate arithmetic/capacity is derived in the same context |
| Engine::encode_slice -> one AsRef -> encode_slice_inner -> encode_with_padding/internal_encode -> encode_helper -> encode_scalar_tail/read_u64 plus add_padding | Checked original wrappers, upper layers, fast/unrolled/ordinary/tail loops and padding; read_u64 is checked using declared primitive interfaces |
| Engine::decode_slice -> one AsRef -> decode_slice_inner -> estimate/internal_decode -> decode_helper/complete_quads_len | Checked original wrappers, upper dispatch, framing and pure-scalar `(0,0)` callback |
| decode_helper -> decode_complete_quads -> chunk8/chunk4 | Checked actual loops and chunk bodies with exact source windows, table inversion, byte reconstruction, length/bounds and full frame; iterator and byte-order primitives are TCB |
| decode_helper -> decode_suffix -> terminal iteration/continue/padding/trailing-bit checks/output writes -> DecodeMetadata::new | Checked actual suffix and metadata aggregate; all canonical produced strings accepted, no error branch allowed; continue translation is TCB |
| terminal/assemble/shifts/trailing/unique, prefix/window/bit lemmas -> actual suffix/chunks/encoder | Checked consumed support, including universal source reconstruction; no support model replaces an executable target |
| actual slice returns -> standard_roundtrip -> exact data/length and both whole-destination frames | Checked caller effect, mandatory success, no hidden caller premise |
| Core/vstd primitives and compiler/SMT | Declared TCB interfaces below; core implementations are source-justified, not machine-checked by this codec run |
| Five external convenience methods and other feature/API paths | Out of scope, no selected claim uses their contracts |

The [detailed call graph](../t15-handoff/call-graph.md) identifies each suffix invariant/call and every inherited boundary. Fresh [Function-Def trace index](t17/fresh/checked-body-traces.txt) contains all these bodies, including wrappers, first layers, helpers, initialization, scalar paths, suffix and caller. The trace paths are relative to the historical replay root; corresponding AIR files are retained alongside the fresh snapshots. These are one full invocation, not a sum of mutually exclusive configurations. Supporting definitions, contracts and proofs coexist in that invocation. No `--verify-only-module` selector occurs in the successful complete command.

## Source identity and correspondence

Production and verification HEAD are both `5d574f8ad899aa9168aa0d25ad03ef634fc4ae79`. [registered identity record](t17/fresh/registered-identities.json) pins inputs, t9–t15 frozen package manifests and t9/t12/t14 binary distributions before copying. [post-replay identity](t17/post-replay-identity.json) revalidates package/input bytes, production cleanliness and registered `audit\n`. The exact selected source hashes are in [fresh invocation metadata](t17/fresh/scalar-probe/run.json).

The extractor installs annotation-bearing bodies at their original src paths only inside the isolated writable checkout and restores them in finally. [fresh scalar/decoder binding](t17/fresh/binding.txt) and [independent wrapper/helper/initializer binding](t17/supplemental-binding.txt) compare production executable tokens with the selected snapshots. The selected [verifier root and module imports](t17/fresh/scalar-probe/lib.rs.bridge#L370-L383) exposes actual module dispatch and consumed proof modules. Retained readable diffs cover [lib.rs](t17/lib.rs.diff), [mod.rs](t17/mod.rs.diff), [general_purpose.rs](t17/general_purpose.rs.diff), [gp_decode.rs](t17/gp_decode.rs.diff), [decode_suffix.rs](t17/decode_suffix.rs.diff), [encode.rs](t17/encode.rs.diff), [decode.rs](t17/decode.rs.diff), [alphabet.rs](t17/alphabet.rs.diff). They contain extensive annotations, not a codec replacement.

The [extraction mapping](t4/handoff.md#L109-L172) splits original lower trait declarations/impl items into CodecCaps and leaves public defaults in Engine, maps lower UFCS/associated types identically, hoists the capture-free inner helpers, and preserves GeneralPurpose default dispatch. This mapping is a correspondence TCB, explicitly limited to this implementation. Ghost erasure removes contracts/assertions/loop invariants and spec-only support, preserves runtime panic/debug checks, and reverses the exact single-use suffix iterator-expression temporary. The proof-specific scalar rlimit affects solver resources only. Exec constants preserve original constructor expressions with checked spec counterparts. There is no proof-specific cfg that switches TOP, View, contracts, implementation, or selected semantics. Defaults and SIMD are absent; std+alloc are the product configuration, not proof modes.

## TCB ledger

There are 20 direct trust groups and 6 residual groups. This counts review categories, not individual imported axioms. The [complete selected-source marker scan](t17/selected-boundaries.txt) and [complete local vstd marker scan](t17/library-boundaries.txt) retain every explicit external/opaque/uninterpreted/assumed/axiomatic/admitted syntax match, including comments. There is no codec correctness external_body, assume or admit on the goal-relevant path; opaque closed spec definitions with bodies are definitions, not unconstrained relations. Generic conversion, core primitives, library models and translation remain explicit trust. Unused library markers are residual; imported foundational families used by the proof are included below. Verified core consumers must not be described as verified core implementations.

The following conditions were explicitly accepted by the independent t18 Reviewer: TCB-01 through TCB-20 as declared conditions, and TCB-21 through TCB-26 as excluded residual surface. Their propositions, evidence and risks are unchanged. A regression count cannot establish an axiom or compiler soundness. Rejection of a material condition requires rollback to Prove, not Audit repair.

### TCB-01 — external trait specification and uninterpreted relation

core; direct; affected TOP-01, TOP-02. The one actual AsRef invocation returns a reference permitted by converted; arbitrary T has only normal-return, nondeterministic conversion semantics. No repeated-conversion equality, termination or Drop purity is assumed for arbitrary user types. [source/evidence](t17/fresh/scalar-probe/mod.rs.bridge#L576-L593).

Acceptance rationale: [ordinary-slice effect analysis](t4/handoff.md#L150-L172) and [retained core forwarding/identity source](t4/convert-mod-std.txt#L710-L855) justify the selected concrete instance. Risk: this reasoning cannot be generalized to arbitrary user conversions.

### TCB-02 — external_body slice identity lemma

core; direct; affected TOP-01, TOP-02. For T=&[u8], reference-forwarding and slice identity return the same byte contents, complete without user code or storage writes, and trivial reference Drop preserves the caller observation. Safe borrowing supplies disjoint mutable destinations. [source/evidence](t17/fresh/scalar-probe/mod.rs.bridge#L584-L593).

Acceptance rationale: [ordinary-slice effect analysis](t4/handoff.md#L150-L172) and [retained core forwarding/identity source](t4/convert-mod-std.txt#L710-L855) justify the selected concrete instance. Risk: this reasoning cannot be generalized to arbitrary user conversions.

### TCB-03 — external function specification

core; direct; affected TOP-01, TOP-02, TOP-04. usize::from(bool) is exactly zero for false and one for true; this is a primitive conversion interface, not a proof of core or a codec-success premise. [source/evidence](t17/fresh/scalar-probe/gp_decode.rs.bridge#L652-L652).

Acceptance rationale: accept the pinned standard verifier/library/correspondence interface for the explicit proposition above, supported by source inspection and the successful actual-body replay. This is a trust condition, not a new codec theorem. Risk: unsound imported models, erasure, source mapping, tool behavior or unlawful environmental inputs can invalidate the result.

### TCB-04 — assume_specification

core; direct; affected TOP-01, TOP-02. u64::from_be_bytes returns the exact unsigned big-endian weighted sum of its eight bytes on the selected target; core endian conversion, layout and transmute behavior are trusted. [source/evidence](../t9-handoff/byte-order.rs#L1-L6).

Acceptance rationale: [conversion/from_be source analysis](../t9-handoff/handoff.md), [to_be source analysis](../t13-handoff/source-justification/README.md), retained [core array conversion](../t9-handoff/source-justification/core-array-mod.rs.txt#L11-L46), [core blanket forwarding](../t9-handoff/source-justification/core-convert-mod.rs.txt#L6-L21) and [core byte representation](../t13-handoff/source-justification/uint_macros.rs.txt#L3950-L4040) and [core from_be_bytes/endian source](../t9-handoff/source-justification/core-num-uint_macros.rs.txt#L1-L53) support the exact interfaces. Positive/negative regressions freshly pass their expected outcomes. Risk: compiler/layout/library mismatch would invalidate correspondence; these are trusted implementations, not checked codec axioms.

### TCB-05 — assume_specification

core; direct; affected TOP-02. u32::to_be_bytes returns each of the four big-endian bytes of the input integer, independent of host endianness; core implementation is source-justified rather than checked. [source/evidence](../t13-handoff/byte-order.rs#L3-L4).

Acceptance rationale: [conversion/from_be source analysis](../t9-handoff/handoff.md), [to_be source analysis](../t13-handoff/source-justification/README.md), retained [core array conversion](../t9-handoff/source-justification/core-array-mod.rs.txt#L11-L46), [core blanket forwarding](../t9-handoff/source-justification/core-convert-mod.rs.txt#L6-L21) and [core byte representation](../t13-handoff/source-justification/uint_macros.rs.txt#L3950-L4040) and [core from_be_bytes/endian source](../t9-handoff/source-justification/core-num-uint_macros.rs.txt#L1-L53) support the exact interfaces. Positive/negative regressions freshly pass their expected outcomes. Risk: compiler/layout/library mismatch would invalidate correspondence; these are trusted implementations, not checked codec axioms.

### TCB-06 — assume_specification

core; direct; affected TOP-02. u64::to_be_bytes returns each of the eight big-endian bytes of the input integer, independent of host endianness; core implementation is source-justified rather than checked. [source/evidence](../t13-handoff/byte-order.rs#L5-L6).

Acceptance rationale: [conversion/from_be source analysis](../t9-handoff/handoff.md), [to_be source analysis](../t13-handoff/source-justification/README.md), retained [core array conversion](../t9-handoff/source-justification/core-array-mod.rs.txt#L11-L46), [core blanket forwarding](../t9-handoff/source-justification/core-convert-mod.rs.txt#L6-L21) and [core byte representation](../t13-handoff/source-justification/uint_macros.rs.txt#L3950-L4040) and [core from_be_bytes/endian source](../t9-handoff/source-justification/core-num-uint_macros.rs.txt#L1-L53) support the exact interfaces. Positive/negative regressions freshly pass their expected outcomes. Risk: compiler/layout/library mismatch would invalidate correspondence; these are trusted implementations, not checked codec axioms.

### TCB-07 — opaque error, uninterpreted conversion, broadcast axiom and assume_specification

vstd; direct; affected TOP-01, TOP-02. The fixed-array Copy slice TryFrom succeeds iff its length is N and copies every element; the error payload is opaque. Existing TryInto forwards through this specialized contract to core blanket TryInto, without a general callable axiom. [source/evidence](../t9-handoff/convert.rs.patch#L1-L38).

Acceptance rationale: [conversion/from_be source analysis](../t9-handoff/handoff.md), [to_be source analysis](../t13-handoff/source-justification/README.md), retained [core array conversion](../t9-handoff/source-justification/core-array-mod.rs.txt#L11-L46), [core blanket forwarding](../t9-handoff/source-justification/core-convert-mod.rs.txt#L6-L21) and [core byte representation](../t13-handoff/source-justification/uint_macros.rs.txt#L3950-L4040) and [core from_be_bytes/endian source](../t9-handoff/source-justification/core-num-uint_macros.rs.txt#L1-L53) support the exact interfaces. Positive/negative regressions freshly pass their expected outcomes. Risk: compiler/layout/library mismatch would invalidate correspondence; these are trusted implementations, not checked codec axioms.

### TCB-08 — external opaque type and uninterpreted state with constructor axiom

vstd; direct; affected TOP-02. Finite Enumerate preserves inner order, returns zero-based offsets, advances its counter without overflow, inherits its inner termination metric and next/exhaustion laws. All selected slice lengths ensure its bounded-counter regime. [source/evidence](../t12-handoff/iter.rs.patch#L1-L57).

Acceptance rationale: [retained core iterator analysis](../t12-handoff/source-justification/README.md) and [fresh window/exhaustion regressions](t17/fresh/t12-regress.log) support exact order, remainder and overflow conditions; selected lawful slices and positive fixed widths obtain the premises. Risk: iterator models must faithfully describe core state and termination.

### TCB-09 — external opaque type, uninterpreted state and assumed iterator interfaces

vstd; direct; affected TOP-02. Positive-width ChunksExact yields precisely the ordered floor-length source windows, preserves a fixed remainder through next and exhaustion, and obeys finite iterator head/pop and metric laws. Actual widths are positive 32 and 4. [source/evidence](../t12-handoff/slice.rs.patch#L1-L38).

Acceptance rationale: [retained core iterator analysis](../t12-handoff/source-justification/README.md) and [fresh window/exhaustion regressions](t17/fresh/t12-regress.log) support exact order, remainder and overflow conditions; selected lawful slices and positive fixed widths obtain the premises. Risk: iterator models must faithfully describe core state and termination.

### TCB-10 — local compiler signature adaptation

toolchain; direct; affected TOP-01, TOP-02. Closed array constants may match only after both successfully normalize and evaluate to equal target-usize values; element types must still match exactly. Rust normalization and the retained t9 frontend patch must preserve this restriction. See the exact patch and signature regressions. [source/evidence](t17/fresh/registered-identities.json).

Acceptance rationale: [exact frontend patch](../t9-handoff/rust_to_vir_func.rs.patch#L1-L32) preserves symbolic structural comparison and rejects failed/unequal closed evaluations; [fresh signature regressions](t17/fresh/t9-regress.log) check accepted signatures and ordinary Rust negative rejections. Risk: negative E0308 cases do not independently exercise every matcher fallback.

### TCB-11 — local macro and continue translator adaptation

toolchain; direct; affected TOP-02. Wrapper next advances ghost state once before the body, macro assertions check that fact, and continue executes ordinary entry-invariant and decreases checks on live state. The t14 lowering and its ghost erasure must preserve Rust control flow. Regression negatives support, but do not prove, translator soundness. [source/evidence](t17/fresh/t14-regress.log).

Acceptance rationale: [macro patch](../t14-handoff/syntax.rs.patch#L1-L30), [backend patch](../t14-handoff/sst_to_air.rs.patch#L1-L19) and [checked wrapper source](../t14-handoff/source-justification/wrapper.rs.txt#L1-L121) show one advancement before body and unchanged continue invariant/decreases checks. Negative state/frame/decreases regressions fail semantically at the expected edges. Risk: translator soundness remains trusted; tests alone are insufficient.

### TCB-12 — foundational sequence and mathematical model axioms

vstd; direct; affected TOP-01, TOP-02, TOP-03, TOP-04. Imported Seq, arithmetic, extensionality, subrange and quantified mathematical interfaces must denote their documented mathematical objects consistently. Checked support lemmas are proofs relative to this foundation, not proofs of these axioms. [source/evidence](../t14-handoff/toolchain/source/vstd/seq.rs#L1-L1954).

Acceptance rationale: accept the pinned standard verifier/library/correspondence interface for the explicit proposition above, supported by source inspection and the successful actual-body replay. This is a trust condition, not a new codec theorem. Risk: unsound imported models, erasure, source mapping, tool behavior or unlawful environmental inputs can invalidate the result.

### TCB-13 — array primitive interfaces

vstd; direct; affected TOP-01, TOP-02. Array fill, index, update and View faithfully model fixed Rust arrays and preserve other elements; these interfaces support initialization, scalar read and chunk output. [source/evidence](../t14-handoff/toolchain/source/vstd/std_specs/array.rs#L1-L125).

Acceptance rationale: accept the pinned standard verifier/library/correspondence interface for the explicit proposition above, supported by source inspection and the successful actual-body replay. This is a trust condition, not a new codec theorem. Risk: unsound imported models, erasure, source mapping, tool behavior or unlawful environmental inputs can invalidate the result.

### TCB-14 — slice, borrow, iterator and mutable prophecy interfaces

vstd; direct; affected TOP-01, TOP-02. Slice bounds, immutable views, mutable borrow final views, get_mut, copy_from_slice and slice iteration obey Rust contents and framing semantics. Final-view prophecy and Rust lifetime/alias translation must be faithful. [source/evidence](../t14-handoff/toolchain/source/vstd/std_specs/slice.rs#L1-L644).

Acceptance rationale: accept the pinned standard verifier/library/correspondence interface for the explicit proposition above, supported by source inspection and the successful actual-body replay. This is a trust condition, not a new codec theorem. Risk: unsound imported models, erasure, source mapping, tool behavior or unlawful environmental inputs can invalidate the result.

### TCB-15 — Vec caller interfaces

vstd; direct; affected TOP-01, TOP-02. Vec length, view and as_slice/as_mut_slice agree with initialized lawful contiguous buffers and preserve unchanged elements across borrowed-slice calls. The caller does not allocate or resize. [source/evidence](../t14-handoff/toolchain/source/vstd/std_specs/vec.rs#L1-L488).

Acceptance rationale: accept the pinned standard verifier/library/correspondence interface for the explicit proposition above, supported by source inspection and the successful actual-body replay. This is a trust condition, not a new codec theorem. Risk: unsound imported models, erasure, source mapping, tool behavior or unlawful environmental inputs can invalidate the result.

### TCB-16 — Option/Result, conversion and callable interfaces

vstd; direct; affected TOP-01, TOP-02, TOP-03, TOP-04. Option/Result successful unwrap, ok_or, map closure return contracts, question-mark propagation and trait dispatch follow ordinary Rust behavior; checked success excludes their failure/panic branches on the selected path. [source/evidence](../t14-handoff/toolchain/source/vstd/std_specs/result.rs#L1-L284).

Acceptance rationale: accept the pinned standard verifier/library/correspondence interface for the explicit proposition above, supported by source inspection and the successful actual-body replay. This is a trust condition, not a new codec theorem. Risk: unsound imported models, erasure, source mapping, tool behavior or unlawful environmental inputs can invalidate the result.

### TCB-17 — numeric, range, string and ordinary core interfaces

vstd; direct; affected TOP-01, TOP-02, TOP-03, TOP-04. Unsigned checked arithmetic and casts, range iteration, constants, string-literal ASCII byte views, equality, Copy/Clone, and boolean primitives match the selected Rust target. The remaining imported foundational interfaces are trusted at their declared contracts; no imported codec fact is assumed. [source/evidence](../t14-handoff/toolchain/source/vstd/std_specs/num.rs#L1-L536).

Acceptance rationale: accept the pinned standard verifier/library/correspondence interface for the explicit proposition above, supported by source inspection and the successful actual-body replay. This is a trust condition, not a new codec theorem. Risk: unsound imported models, erasure, source mapping, tool behavior or unlawful environmental inputs can invalidate the result.

### TCB-18 — selected extraction and dispatch correspondence

correspondence; direct; affected TOP-01, TOP-02, TOP-03, TOP-04. The CodecCaps/Engine partition, inner hoist, associated-type and UFCS remapping, ghost contracts, exec-constant spec counterparts and exact single-use suffix iterator temporary preserve the original GeneralPurpose defaults, arguments, call order and executable algorithm. [source/evidence](t4/handoff.md#L109-L172).

Acceptance rationale: accept the pinned standard verifier/library/correspondence interface for the explicit proposition above, supported by source inspection and the successful actual-body replay. This is a trust condition, not a new codec theorem. Risk: unsound imported models, erasure, source mapping, tool behavior or unlawful environmental inputs can invalidate the result.

### TCB-19 — compiler, verifier, SMT and binary identity

toolchain; direct; affected TOP-01, TOP-02, TOP-03, TOP-04. Selected Rust 1.98.1, Verus d85f757 plus pinned t9/t12/t14 patches, vstd, Z3, VC generation, bit-vector reasoning, borrow/lifetime translation and ghost erasure are sound. Retained binary/source/library manifests and filesystem hashes identify the executed stack; no tool rebuild was performed during this Audit. [source/evidence](t17/fresh/scalar-probe/run.json).

Acceptance rationale: accept the pinned standard verifier/library/correspondence interface for the explicit proposition above, supported by source inspection and the successful actual-body replay. This is a trust condition, not a new codec theorem. Risk: unsound imported models, erasure, source mapping, tool behavior or unlawful environmental inputs can invalidate the result.

### TCB-20 — lawful memory and execution environment

environment; direct; affected TOP-01, TOP-02, TOP-03, TOP-04. Inputs and both initialized destination buffers actually exist as lawful live Rust allocations with required lengths and valid nonconflicting borrows; platform usize/isize and unsigned layout match the selected target. Allocation availability, hardware faults and process interruption are outside the theorem. [source/evidence](t17/post-replay-identity.json).

Acceptance rationale: accept the pinned standard verifier/library/correspondence interface for the explicit proposition above, supported by source inspection and the successful actual-body replay. This is a trust condition, not a new codec theorem. Risk: unsound imported models, erasure, source mapping, tool behavior or unlawful environmental inputs can invalidate the result.

### TCB-21 — unrelated external_body encode

project; residual; affected none. Engine::encode has an inherited external body and no selected round-trip claim depends on it. It is excluded from the caller and goal-relevant call graph, and supplies no success or data premise. [source/evidence](t17/fresh/scalar-probe/mod.rs.bridge#L161-L168).

Acceptance rationale: no selected call, assumption or postcondition is obtained from this API/declaration. Residual means unproved surface, not successful verification of that implementation.

### TCB-22 — unrelated external_body encode_string

project; residual; affected none. Engine::encode_string has an inherited external body and no selected round-trip claim depends on it. It is excluded from the caller and goal-relevant call graph, and supplies no success or data premise. [source/evidence](t17/fresh/scalar-probe/mod.rs.bridge#L202-L209).

Acceptance rationale: no selected call, assumption or postcondition is obtained from this API/declaration. Residual means unproved surface, not successful verification of that implementation.

### TCB-23 — unrelated external_body decode

project; residual; affected none. Engine::decode has an inherited external body and no selected round-trip claim depends on it. It is excluded from the caller and goal-relevant call graph, and supplies no success or data premise. [source/evidence](t17/fresh/scalar-probe/mod.rs.bridge#L276-L283).

Acceptance rationale: no selected call, assumption or postcondition is obtained from this API/declaration. Residual means unproved surface, not successful verification of that implementation.

### TCB-24 — unrelated external_body decode_vec

project; residual; affected none. Engine::decode_vec has an inherited external body and no selected round-trip claim depends on it. It is excluded from the caller and goal-relevant call graph, and supplies no success or data premise. [source/evidence](t17/fresh/scalar-probe/mod.rs.bridge#L335-L342).

Acceptance rationale: no selected call, assumption or postcondition is obtained from this API/declaration. Residual means unproved surface, not successful verification of that implementation.

### TCB-25 — unrelated external_body decode_slice_unchecked

project; residual; affected none. Engine::decode_slice_unchecked has an inherited external body and no selected round-trip claim depends on it. It is excluded from the caller and goal-relevant call graph, and supplies no success or data premise. [source/evidence](t17/fresh/scalar-probe/mod.rs.bridge#L414-L421).

Acceptance rationale: no selected call, assumption or postcondition is obtained from this API/declaration. Residual means unproved surface, not successful verification of that implementation.

### TCB-26 — residual imported library declarations

vstd; residual; affected none. Unused vstd external/opaque/uninterpreted/axiomatic declarations outside the primitive families used above have no selected caller edge. The complete textual library marker inventory is retained; library import alone is not a checked core implementation or an additional API claim. [source/evidence](../t14-handoff/toolchain/source/vstd/vstd.rs#L1-L169).

Acceptance rationale: no selected call, assumption or postcondition is obtained from this API/declaration. Residual means unproved surface, not successful verification of that implementation.

Additional foundational source coverage: [vstd std_specs/option.rs](../t14-handoff/toolchain/source/vstd/std_specs/option.rs#L1-L412), [vstd std_specs/convert.rs](../t14-handoff/toolchain/source/vstd/std_specs/convert.rs#L1-L211), [vstd std_specs/iter.rs](../t14-handoff/toolchain/source/vstd/std_specs/iter.rs#L1-L979), [vstd std_specs/range.rs](../t14-handoff/toolchain/source/vstd/std_specs/range.rs#L1-L587), [vstd string.rs](../t14-handoff/toolchain/source/vstd/string.rs#L1-L965), [vstd std_specs/ops.rs](../t14-handoff/toolchain/source/vstd/std_specs/ops.rs#L1-L804), [vstd std_specs/core.rs](../t14-handoff/toolchain/source/vstd/std_specs/core.rs#L1-L203), [vstd arithmetic/div_mod.rs](../t14-handoff/toolchain/source/vstd/arithmetic/div_mod.rs#L1-L1667). These provide the source locations for the grouped ordinary-core/model interfaces; the full marker scan preserves individual syntax sites. The selected bool primitive is supported by [core bool conversion source](t4/convert-num-std.txt#L1-L727).

## Verification and Audit evidence

From the workspace root, the durable audit launcher is:

```bash
python3 verification/.system-proof/t17/replay-audit.py
```

It requires a new `t17-audit-replay` sibling destination. Its [source](t17/replay-audit.py) runs the frozen t15 replay in memory with only two operational adaptations: it checks the registered phase is Audit, and labels the isolated copy Prove for historical replay/binding assertions. No frozen package is edited, and no registered phase is written. [launcher identity](t17/launcher-identity.json) and [outer readable output](t17/replay.log) make this explicit. The historical printed “remain in Prove” refers only to the copy and carries no campaign authority. For a new review run, pass `--destination /new/writable/path`; the unchanged source/package/tool hash validation remains mandatory.

The launcher invokes these serial commands in the copy:

```bash
python3 t15-handoff/regress.py
python3 t15-handoff/probe.py
python3 t15-handoff/check-binding.py --production /fastdata/v-boxiuli/noah/exp/ts/production
```

Exact commands, timestamps, expected markers and readable output are retained in [fresh driver records](t17/fresh/replay-results.json). The complete proof command is:

```text
<t14 distribution>/verus src/lib.rs --cap-lints allow --crate-type lib --edition 2021
  --cfg 'feature="std"' --cfg 'feature="alloc"' --trace --log vir --log air
  --log-dir t15-handoff/scalar-probe/ir
```

The executable is Verus `0.2026.09.29.d85f757.t14`, release linux_x86_64, Rust `1.98.1-x86_64-unknown-linux-gnu`. The original Verus source is d85f75780de687461a30e7771985c4a3e9ea0e58, with reviewed t9 normalization/conversion, t12 iterator/library and t14 macro/backend continue patches. [tool source manifest](../t14-handoff/tool-source-sha256.json) and [binary/library/solver manifest](../t14-handoff/adapted-binary-sha256.json) identify actual adapted inputs; synthetic build Git metadata is not the source identity. The fresh invocation records distribution hashes, and the pre-copy driver checks all retained distributions. No installed tools were edited or rebuilt during Audit. Fresh [tool correspondence checks](t17/tool-identity.json) verify all 1464 original and adapted source-manifest entries, exactly the six reviewed changed files, installed binary/library hashes, adapted distribution hashes and clean original installed source. The build-to-binary relationship still relies explicitly on retained reviewed build evidence, not a version string.

Fresh scope is the full extracted target crate, 146 obligations, 0 errors, exit 0, including source-grounded caller and every selected codec body. [readable verifier output](t17/fresh/scalar-probe/verify.log) and [VIR root](t17/fresh/scalar-probe/ir/crate.vir) expose input/body correspondence; the [AIR query index](t17/fresh/checked-body-traces.txt) locates actual Function-Def queries. Individual repeated Function-Def entries include split queries and do not equal the verified count. No separate run counts are added.

Fresh inherited suites use their historical pinned distributions, while the complete candidate uses the t14 distribution. They cover t9 signature/conversion, t12 enumeration/windows/remainder/exhaustion, t13 positive/inverse byte order and false byte claims, and t14 continue/control-flow/state/frame/decreases. Required false claims failing are regression successes, not unresolved candidate failures. Readable logs are [t9](t17/fresh/t9-regress.log), [t12](t17/fresh/t12-regress.log), [t13](t17/fresh/t13-regress.log), [t14](t17/fresh/t14-regress.log). Historical construction resource failures and the rejected prophetic termination measure are retained in [t15 failure account](../t15-handoff/handoff.md); the final full replay has no timeout/failure. Default suffix budget and scalar-local 80 are unchanged.

## Findings, bugs and repairs

No production data-loss counterexample or required product repair was established. The active claim concerns the original registered algorithms. Prior codec proof gaps are closed by checked bodies; the former core-interface and continue-lowering limitations are handled by explicitly trusted, source-supported local adaptations rather than by trusting codec correctness. Audit introduced only evidence/report scripts and artifacts. One evidence collector initially assumed every historical task had inputs.json; the collector now handles absent optional manifests as the frozen replay already does. This had no effect on proof execution or candidate meaning.

Historical specdet underconstraint analysis was inconclusive owing to quantifiers, not an accepted completeness proof. Its original artifacts remain linked in the t4 handoff. The defined byte relation, source-grounded caller, mandatory-success/frame guarantees and checked whole-output uniqueness provide reviewable semantic evidence; a specialist tool outcome does not replace original-goal judgment.

## Lifecycle, reliability, scale and cost metrics

| ID | Metric | Value | Evidence |
| --- | --- | --- | --- |
| METRIC-01 | Fresh complete candidate replay | 146 verified, 0 errors; exit 0 | [evidence](t17/fresh/scalar-probe/verify.log) |
| METRIC-02 | Complete candidate elapsed time | 58.640896 seconds; 2026-11-03T08:52:41.994117+00:00 to 2026-11-03T08:53:40.635013+00:00 | [evidence](t17/fresh/scalar-probe/run.json) |
| METRIC-03 | Inherited regression suites | 4 suites passed: t9 signature/conversion, t12 iterator/window, t13 byte order, t14 continue; intended negative failures are required outcomes | [evidence](t17/fresh/regression-results.json) |
| METRIC-04 | Audit implementation repairs | 0; frozen t9–t15 packages and inputs revalidated unchanged; production clean; phase audit | [evidence](t17/post-replay-identity.json) |
| METRIC-05 | Tool source correspondence | 1464 original and 1464 adapted manifest entries checked; exactly six reviewed source adaptations; original installed source clean and binaries unchanged | [evidence](t17/tool-identity.json) |
| METRIC-06 | Audit resource setting | Suffix default; inherited scalar-local rlimit 80; no resource escalation | [evidence](../t15-handoff/handoff.md) |
| METRIC-07 | Selected snapshot scale | 4236 lines across 9 selected installed source snapshots; includes docs and proof annotations, not formal LOC | [evidence](t17/metrics.json) |
| METRIC-08 | Campaign cost and reliability totals | Unavailable: no complete token/AIC, wall-time, restart, rollback or verifier-attempt accounting was supplied to this Audit; historical construction failures are retained but are not summed | [evidence](t17/metrics.json) |
| METRIC-09 | Specialist attack disposition | Historical t4 specdet inconclusive (incomplete quantifiers); no completeness acceptance inferred; checked writes uniqueness is consumed in the common context | [evidence](t4/handoff.md) |

## Remaining limitations and independent decision

The result depends on explicit primitive/library/compiler/correspondence/environment trust, not on assumed codec success. Core integer/array/iterator implementations are justified by retained source, not fully verified Rust core bodies. There is no allocation-existence theorem, all-platform compiler theorem, arbitrary generic-conversion lifecycle theorem, other-configuration claim, malformed-string guarantee, or proof of the five convenience APIs. These limits preserve the original byte-slice round-trip objective and its lawful capacity domain.

The independent t18 Reviewer accepted the three protected Audit items and all listed TCB groups with the classifications above, finding no rollback or substantive repair necessary. That letter reports a fresh independent replay of 146 verified, 0 errors, four passing inherited regression suites, source/support hashes matching t17, and inspection supporting correspondence. The unavailable /tmp evidence package is not substituted for durable evidence or used as an installation source. The t17 evidence links and source identities below remain unchanged. The later t18 delivery correction left approved-finalization installation unconfirmed. This t19 report records semantic acceptance; completion requires approval of this exact report text, confirmed installation, passing required checks and independent Reviewer/Manager judgment. Phase remains Audit. If a material semantic, proof, trust or binding gap is found, return to Prove; perform no substantive repair in Audit.

## Evidence map

Start with the TOP matrix, [caller](t17/fresh/scalar-probe/caller.rs.bridge#L1-L33), [fresh identity/command](t17/fresh/scalar-probe/run.json), [fresh verification](t17/fresh/scalar-probe/verify.log), [actual-body binding](t17/fresh/binding.txt), [wrapper/initialization binding](t17/supplemental-binding.txt), [closure inventory](../t15-handoff/call-graph.md), [selected boundaries](t17/selected-boundaries.txt), [library boundaries](t17/library-boundaries.txt), [metric provenance](t17/metrics.json). Durable installed snapshots and AIR/VIR live under `.system-proof/t17/fresh/`; replay scripts and static packages remain in the registered verification tree. No evidence depends solely on /tmp.
