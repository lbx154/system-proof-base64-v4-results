# t1 Ground handoff

## Objective and current result

Preserve E's exact conditional round-trip theorem and selected STANDARD configuration, prepare a reproducible default-budget route, and expose its two unresolved loop obligations for proof reorganization in the next task. No theorem, executable source, Cargo input, E report or supplied Verus changed. Production is clean at d9ce0dea622ff80eacb736f95e9488cf1eaa06f4. The project request is verbatim in `.system-proof/INPUT.md`; `preservation-check.log` confirms it and compares current source against the pre-setup snapshot.

Default actual-crate replay: **177 verified, 2 errors**, both SMT resource exhaustion at `src/engine/general_purpose/decode.rs:284` and `:390`. This exactly reproduces the operator's supplied E default baseline. E's final report states **179/0 with --rlimit 20**; that is historical evidence only, not a fresh run or a selected option. No raised-limit invocation was made in t1. Ground establishes a checked first layer and caller usability, not complete default-budget closure or completion of the campaign.

## Open TOP meaning, domain and usage

Canonical meaning: `src/round_trip_spec.rs:7-85`, especially `supported` (:12), `standard_encoding` (:39), `encode_requires/encode_post` (:46/:50), `decode_requires/decode_post` (:56/:61), Result contracts (:67/:73), and existential/universal decoder contracts (:80/:83). These are open definitions over explicit byte sequences; no uninterpreted encoder/inverter defines the theorem.

Let S=isize::MAX, n=effective byte length, E=4*ceil(n/3), D=3*ceil(E/4). The lawful domain is **n <= 3*floor(S/4)**, equivalent to E<=S. This follows from valid Rust byte-slice extent and padded destination representation, not accepted inputs. `round_trip_proof::capacity_domain` proves the arithmetic equivalence and n<=D<=n+2. Both valid destinations must have length >=E and >=D. D<=E<=S. No allocation existence for these maxima is promised; the theorem is conditional on represented supplied buffers. Empty, arbitrary octets and every block remainder are included.

Successful encode returns exactly Ok(E), writes `standard_encoding(x)`, preserves input and destination suffix after E. Decode receives precisely that prefix, returns Ok(n), and restores x byte-for-byte, preserving its encoded input and destination suffix after n. Both errors are false in the lawful/capacity domain; helper failure or panic is not an allowed substitute.

Actual public contracts and executable wrappers: `src/engine/mod.rs:255-333` (`Engine::encode_slice`) and `:462-496` (`Engine::decode_slice`); these default methods are the selected GeneralPurpose public methods. Their contract uses one actual AsRef conversion's ghost observation. Generic conversion/cleanup completion and enumerated frames are explicit environmental conditions (`src/as_ref_spec.rs`), not guaranteed from AsRef membership. The production destructor counterexample in ATTACK.md shows why cleanup completion is necessary; no historical authorization artifact is needed to preserve the currently requested E theorem. Borrowed slice/reference instances make those premises true. Owned generic use proves success on this invocation's returned bytes and sufficient capacities, not a snapshot of a pre-conversion object or a second AsRef result.

Source-grounded usage intent comes from actual public API docs at `src/engine/mod.rs:233-251`, decode docs at :451-459, public helper docs, existing arbitrary-data/prefix/frame consumers in `src/engine/tests.rs:396-402, :691-708, :1473-1522`, and requested usage. These existing tests/docs motivate the verification-only consumers; no new production application caller is claimed. `src/round_trip_use.rs` supplies borrowed decode, `helper_sized_round_trip` (actual helper sizes -> encode -> reported prefix -> decode -> exact observation) and `owned_generic_round_trip` (one owned encoding input -> borrowed-prefix decode). Owned decoder arbitrary-T coverage is its own public conditional contract; no generic object is assumed to yield that prefix without its caller relation.

Fresh caller command (common cfg, contracts unchanged):

```bash
source .system-proof/t1/adapter/env.sh
verus src/lib.rs --crate-type lib --edition 2021 --cfg 'feature="std"' --cfg 'feature="alloc"' --verify-module round_trip_use
```

`caller-check.log`: **3 verified, 0 errors**, exit 0. No success premise about the codec is trusted by this caller. A public contract is consumed; its two lower loop body obligations remain explicitly open.

## Verifier route, actual code binding and first bridge

Setup and repair are in INTEGRATION.md; code in `adapter/` is part of the review. Source `adapter/env.sh` before any command. The registered E binary is `.system-proof/e-baseline/verus/source/target-verus/release/verus`: Verus 0.2026.09.29.d85f757.dirty, release, tangruize/verus d85f7578 plus supplied marker-metasized.patch; Rust 1.98.1 (48a229cea), bundled Z3 4.16.0. `identities.log` retains versions. The existing frontend patch is a TCB item; it was not rebuilt or altered. Runtime Rust libraries were installed separately.

Selected whole-crate baseline:

```bash
source .system-proof/t1/adapter/env.sh
verus-run setup "$PWD" --in-place --package base64
verus-run verify "$PWD" --package base64
```

The runner emits the absolute registered `src/lib.rs`, `--crate-type lib --edition 2021 --cfg 'feature="std"' --cfg 'feature="alloc"'`. This is the direct bundled-metadata equivalent of `--no-default-features --features std` (std entails alloc), not a claim of Cargo verification. There is no SIMD/default feature, proof-specific semantic cfg, solver/resource override, hidden no-lifetime option, or source copy. `.system-proof/verus.toml` pins this route; setup/verify and future installed stage-check audit replay select it through local verus-run on PATH.

`src/lib.rs:328-342` loads specification, caller and proof modules together with actual engine/codec modules. The default methods and descendants are at their actual source locations, annotated with Verus, not copied algorithms substituted into a standalone harness. Existing expanded-helper/scalar rewrites are disclosed correspondence adaptations below.

First target bridge command uses same source/cfg and only focuses obligations:

```bash
verus src/lib.rs --crate-type lib --edition 2021 --cfg 'feature="std"' --cfg 'feature="alloc"' --verify-only-module engine
```

`public-bridge.log`: **5 verified, 0 errors**, exit 0. Public wrappers have checked bodies, not external_body. Their reviewed callee contracts are part of the same full selected context, with the two quads loops as named lower obligations. `baseline-timing.json`/`function-results.json` expose function-mode/success information for every checked function; `baseline-smt/engine__general_purpose__decode.smt2` binds failing queries to real source loop spans. Focused success does not erase full-crate failure or count as final proof closure.

## Call graph and dispositions

| Material path | Current disposition |
| --- | --- |
| Usage -> STANDARD -> GeneralPurpose/alphabet/config/encode and decode tables | Actual initialized constant/config and constructor/table bodies checked; is_standard is established by these, not a caller readiness oracle. |
| Public wrappers -> as_ref_bytes -> AsRef -> observation | Extraction/wrappers checked. Core AsRef slice/reference matching and arbitrary-T invocation completion/relation are declared interface/environment TCB. |
| Helpers -> encoded_len, decoded_len_estimate -> GeneralPurposeEstimate::new/estimate | Actual helper/estimate bodies and domain arithmetic checked; public caller establishes sufficiency from their returned values. |
| encode_slice -> expanded encode_with_padding path -> internal_encode -> scalar zero callback -> encode_scalar_tail -> packing/table writes -> add_padding | Checked adapted actual bodies. Opaque original generic helper is bypassed by source-preserving expansion; its opaque contract supplies no selected proof edge. |
| encode result -> reported prefix -> existential decoder image | Checked usage and round_trip_proof composition lemmas. Exact prefix excludes unused output suffix. |
| decode_slice -> internal_decoded_len_estimate -> internal_decode -> zero scalar callback -> decode_helper -> complete-quads arithmetic | Actual wrappers/helper bodies checked against reviewed contracts; full closure depends on decode_complete_quads below. |
| decode_helper -> decode_complete_quads -> chunk_8/chunk_4 | Chunk bodies and image_window/image_window_byte/canonical_quad facts check. decode_complete_quads has two unresolved default-budget loop queries; its final function-body query checks under loop invariants. This callee is in proof closure, not TCB. |
| decode_helper -> decode_suffix -> terminal/padding/metadata -> DecodeSliceError::from | Actual suffix, arithmetic, frame and conversion bodies check; bundled Into/From forwarding model is TCB. |
| canonical input image -> injectivity -> universal decoder witness -> caller recovered bytes | Defined standard_encoding plus checked canonical tail/quad/injectivity/composition support, consumed by actual decoder and usage; no assumed inversion. |
| Codec-complete Result -> complete_owned_cleanup -> public Result | Explicit external cleanup completion/frame and correspondence to production's implicit exactly-once complete recursive drop: TCB/environment, not verified arbitrary destructors. |
| Sequence/slice/array/indexing/copy/get_mut, arithmetic, conversions, Option/Result, callbacks, string literal axioms | Bundled vstd/core models: interface TCB with actual native execution/borrowing semantics. |
| Allocation, unchecked API, other engines/configs, malformed external strings, streaming, SIMD | Outside requested selected closure. Residual external bodies do not discharge selected target obligations. |

## Adaptations, abstractions and trust

`preserved/e-working.patch` plus six untracked proof/interface files and `preserved/e-source.tar.gz` preserve the complete starting verification source. Compare these with production for exact changes. No t1 source edits. Existing adaptations include helper/padding expansion at actual caller, concrete scalar callback specialization, equivalent index loops for unsupported iterators, explicit big-endian byte packing, equal-valued alphabet/config initializer relocation, arithmetic expression rewrites, ghost snapshots/contracts/invariants and explicit owned cleanup forwarding. Preservation of behavior/error order and complete-drop correspondence is manual TCB, not a compiler refinement theorem. The later task must retain it while reorganizing proof; no product repair is authorized.

TCB inventory for this Ground frontier:

- Valid Rust references, extent <=isize::MAX, lifetimes/borrowing and no invalid aliasing.
- External AsRef method matching, slice identity/reference forwarding and this invocation's caller conversion relation/completion/effects. No arbitrary-T purity, stability or totality inferred.
- Caller complete recursive cleanup completion/effects; `complete_owned_cleanup` external frame and exactly-once correspondence to original implicit drop. It frames the whole destination after codec completion and cannot establish codec correctness.
- Supplied Verus/Rust/ghost erasure/macros/metadata/Z3, including existing MetaSized frontend accommodation. E's report notes residual method predicate/ABI/safety matching limitations; t1 does not repair or independently certify verifier soundness.
- Bundled vstd library models, literal string axioms, Into/From and widening models.
- `byte_slice_valid` external proof assumption and panic `assert_failed` interface requiring false (`src/panic_spec.rs`); codec checks must prove panic unreachable.
- Manual production/verification correspondence for all existing behavior-preserving adaptations listed above.

There is no assume/admit closing a codec fact. Generic environment spec methods are abstract and explicitly environmental; standard_encoding and its inverse bridge are defined/checked. Closed predicates such as decoder_image, canonical_quads_use and estimate compatibility have concrete definitions, real source anchors and reveal consumers; they are proof abstractions, not independent unconstrained symbols. Residual external helpers/APIs and historical mixed-vstd Cargo evidence from E's report do not close this direct bundled-metadata run. Original report links into absent E task directories cannot be treated as independently available evidence; fresh t1 source inspection/replays carry this handoff. Full correspondence/TCB acceptance remains for Audit.

## Precise proof frontier and baseline measurements

Both failures are in actual `decode_complete_quads` at :245. Its contract frames all bytes outside the selected output interval and promises Ok plus x bytes in the canonical image case. The first loop (:284) processes 32 input bytes -> 24 output bytes via four checked chunk_8 calls; second (:390) processes residual complete groups of 4 -> 3 using checked chunk_4. Their invariants carry canonical input/witness/table facts, index/alignment/capacity arithmetic, output prefix equality, and an unchanged complement. Each iteration repeats image_window/byte reasoning, slice-window equality and quantified prefix/frame composition in the large loop context. Default-budget failure is at the loop query, not evidence of a concrete false postcondition. No reorganization has yet been attempted in t1.

| Observation | Result |
| --- | --- |
| Canonical default replay | 177/2; wall 9.530 s (`baseline-default.json`) |
| Default diagnostic replay, same source and options plus observational timing/SMT logging | 177/2; wall 9.71 s, user 18.63 s, system 3.72 s, max RSS 582828 KiB (`baseline-timing.resources`) |
| Verus SMT total for decode_complete_quads (both loops plus body) | 3.659348 s; 62,110,056 reported resource units, false success (`baseline-timing.json`) |
| Loop :284 generated-query serialized replay | unknown/canceled; 1.911483 s, 30,000,000 resource-unit delta |
| Loop :390 generated-query serialized replay | unknown/canceled; 1.672643 s, 30,000,000 resource-unit delta |
| Historical E raised-limit report | 179/0 at --rlimit 20, retained report only; no fresh raised-limit evidence |

Verus default rlimit 10 generates a 30,000,000 Z3 limit per query. `measure-baseline.sh` retains the full default run with timing/JSON/SMT logging, no solver-budget change. Verus aggregates function timings; per-loop observations come from replay of its exact generated module SMT in the same query order with only echo markers added. `replay-loop-timing.py` and `baseline-loop-replay.smt2`/stdout/queries/summary retain that serialized measurement. This is explicitly **query replay timing**, not native per-loop wall timing or a second verification claim. The generated default resource settings are unchanged. Timings are one measurement, not a distribution or runtime throughput benchmark. Later comparison should use the same measurement procedure and label query/function/whole-run costs separately.

## Attack decision, unresolved meaning and transition

ATTACK.md assesses analyze-function, analyze-protocol and specdet. The selected lifecycle protocol tool route is blocked by missing executable/package source; a fresh actual-production normal/panicking-Drop attack concretely distinguishes the live alternative. No count/frame alternative remains live after direct source inspection and checked usage; specdet is not selected. Independent Reviewer must assess this decision and interpretation, not accept E's report.

Remaining Prove work is the two default-budget loops and the later maintenance experiment. Factor local facts/loop-step lemmas, trim invariants and control unfolding while preserving the theorem and actual decode behavior. After default whole-crate success, compare a plausible behavior-preserving decode change on reorganized proof and this archived E proof, retaining timing and proof-edit scope. Those changes are deliberately outside t1.

Installed Ground supplemental check: `python3 "$SYSTEM_PROOF_PACK/checks/check_stage.py" ground "$PWD" --production "$PWD/.production"` -> **ok**, exit 0 (`ground-stage-check.log`, `.exit`). It allows declared failing lower obligations and does not decide Ground adequacy. Independent acceptance of all four Ground items and the Manager's transition decision are still required; phase remains ground.
