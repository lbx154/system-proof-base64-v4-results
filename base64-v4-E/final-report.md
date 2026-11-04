+++
schema = "system-proof/final-report-v1"
verdict = "conditional"
source_revision = "d9ce0dea622ff80eacb736f95e9488cf1eaa06f4"
active_implementation = "Unchanged original production; annotated actual-source verification with reviewed behavior-preserving adaptations and explicit complete-drop forwarding"
requested_top_level_coverage = "complete"
tcb_inventory = "complete"

[[top_level]]
id = "TOP-01"
symbol = "Engine::encode_slice"
explanation = "For STANDARD and sufficient represented output capacity for the actual sole conversion result, returns the exact canonical encoded prefix and count, preserving live input and destination suffix, conditional on caller conversion and complete cleanup completion/effects."
status = "conditional"
production = "../.production/src/engine/mod.rs#L227-L256"
specification = "../src/round_trip_spec.rs#L46-L71"
implementation = "../src/engine/mod.rs#L269-L333"
caller_status = "verification-only"
caller = "../src/round_trip_use.rs#L75-L133"

[[top_level]]
id = "TOP-02"
symbol = "Engine::decode_slice"
explanation = "For STANDARD and exactly an encoder-reported prefix with helper-sized destination, returns every original byte and its exact count, preserving encoded input and destination suffix, conditional on caller conversion and complete cleanup completion/effects."
status = "conditional"
production = "../.production/src/engine/mod.rs#L386-L412"
specification = "../src/round_trip_spec.rs#L56-L87"
implementation = "../src/engine/mod.rs#L474-L496"
caller_status = "verification-only"
caller = "../src/round_trip_use.rs#L75-L133"

[[top_level]]
id = "TOP-03"
symbol = "encoded_len"
explanation = "With padding enabled and supported input length, computes Some(4*ceil(n/3)) without overflow; this is sufficient represented encoding capacity."
status = "verified"
production = "../.production/src/encode.rs#L101-L128"
specification = "../src/encode.rs#L110-L117"
implementation = "../src/encode.rs#L118-L145"
caller_status = "verification-only"
caller = "../src/round_trip_use.rs#L29-L73"

[[top_level]]
id = "TOP-04"
symbol = "decoded_len_estimate"
explanation = "Computes 3*ceil(e/4), providing sufficient represented decoding capacity for every supported encoder image, including terminal tails."
status = "verified"
production = "../.production/src/decode.rs#L189-L193"
specification = "../src/decode.rs#L203-L205"
implementation = "../src/decode.rs#L206-L210"
caller_status = "verification-only"
caller = "../src/round_trip_use.rs#L29-L73"

[[tcb]]
id = "TCB-01"
kind = "T1 Rust representation and local frames"
origin = "core"
explanation = "Valid byte slices have extent at most isize::MAX; sound shared and exclusive reference, provenance and lifetime semantics justify live input immutability and local destination frames."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "../src/as_ref_spec.rs#L35-L40"

[[tcb]]
id = "TCB-02"
kind = "T2 external AsRef interface"
origin = "core"
explanation = "Pinned slice conversion returns itself and reference conversion forwards with the actual receiver-bound lifetime; arbitrary T receives only its declared per-invocation relation, never automatic stability or purity."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../src/as_ref_spec.rs#L5-L32"

[[tcb]]
id = "TCB-03"
kind = "T3 adapted frontend interface matching"
origin = "toolchain"
explanation = "The pinned v2 MetaSized accommodation and manual exact-method checks preserve the real external interfaces despite existing predicate-comparison and ABI/safety checking omissions."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
evidence = "t7/ASSESSMENT.md"

[[tcb]]
id = "TCB-04"
kind = "T4 bundled library models and literal axioms"
origin = "vstd"
explanation = "The pinned bundled vstd/core/alloc models for sequences, slices, arrays, indexing, copying, get_mut, arithmetic, conversions, Option/Result, callbacks and literal ASCII faithfully model runtime behavior."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "t3/frontend/verus/source/vstd/std_specs/slice.rs#L1-L40"

[[tcb]]
id = "TCB-05"
kind = "T5 Into/From forwarding"
origin = "vstd"
explanation = "The exact blanket Into-to-From forwarding and numeric widening models match core; arbitrary user From correctness is not assumed."
goal_relevance = "direct"
used_by = ["TOP-02"]
source = "t3/frontend/verus/source/vstd/std_specs/convert.rs#L8-L51"

[[tcb]]
id = "TCB-06"
kind = "T6 panic interface"
origin = "project"
explanation = "AssertKind matches the real panic interface and assert_failed has false precondition; checked codec branches must prove failure unreachable."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../src/panic_spec.rs#L1-L12"

[[tcb]]
id = "TCB-07"
kind = "T7 compiler verifier solver and erasure"
origin = "toolchain"
explanation = "Pinned Rust, adapted Verus, macros, ghost erasure, lifetime checking, imported VIR and Z3 soundly implement the reported verification semantics."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
evidence = "t31/replay.json"

[[tcb]]
id = "TCB-08"
kind = "T8 historical mixed-vstd restriction"
origin = "environment"
explanation = "Historical Cargo evidence depends on compatible mixed-vstd contract meanings and exact core interfaces; it supplies no missing obligation in the final direct bundled-metadata proof."
goal_relevance = "residual"
used_by = []
evidence = "t15/dependency-trees.json"

[[tcb]]
id = "TCB-09"
kind = "T9 production correspondence"
origin = "correspondence"
explanation = "The reviewed helper expansions, byte packing, loop rewrites, initializer relocation and ghost erasure preserve selected original production observations and order; this is manual correspondence, not compiler refinement."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02", "TOP-03", "TOP-04"]
source = "t21/AUDIT.md#L1-L70"

[[tcb]]
id = "TCB-10"
kind = "T10 authorized caller-code premises"
origin = "project"
explanation = "This invocation's caller-owned AsRef conversion and complete owned cleanup terminate normally and preserve the enumerated observed input, buffers and represented capacities; these premises contain no codec correctness fact."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../src/as_ref_spec.rs#L5-L32"

[[tcb]]
id = "TCB-11"
kind = "T11 complete-drop forwarding and frame"
origin = "correspondence"
explanation = "Explicit complete_owned_cleanup corresponds to production's exactly-once implicit destruction including recursive fields, and under the cleanup premise preserves the whole destination from codec completion to result delivery."
goal_relevance = "direct"
used_by = ["TOP-01", "TOP-02"]
source = "../src/as_ref_spec.rs#L43-L52"

[[tcb]]
id = "TCB-12"
kind = "Residual opaque and excluded APIs"
origin = "project"
explanation = "Unselected opaque encoding helpers, allocation and unchecked APIs and excluded SIMD/streaming/test paths have no claimed body proof here and discharge no selected slice-path obligation."
goal_relevance = "residual"
used_by = []
source = "../src/encode.rs#L70-L86"

[[metric]]
id = "METRIC-01"
name = "Fresh actual-target Audit replay"
value = "179 verified, 0 errors, exit 0; independent replay output byte-identical"
source = "t31/actual-target.log"

[[metric]]
id = "METRIC-02"
name = "Pinned replay identity coverage"
value = "33 live inputs; 60 tool and 284 dependency identities; unchanged across replay"
source = "t31/replay.json"

[[metric]]
id = "METRIC-03"
name = "Replay interval"
value = "2026-11-04T10:06:30.627973Z through 2026-11-04T10:06:42.081438Z"
source = "t31/replay.json"

[[metric]]
id = "METRIC-04"
name = "Original production Drop lifecycle controls"
value = "Four cases: encode/decode crossed with normal/panicking Drop; both panicking cases write correctly then prevent caller consumption"
source = "t28/drop-replay.log"

[[metric]]
id = "METRIC-05"
name = "Resource and analysis limitations"
value = "Final direct replay uses rlimit 20; default10 exhaustion retained; native specdet inconclusive; analyze-function unavailable"
source = "t32/AUDIT.md"

[[metric]]
id = "METRIC-06"
name = "Aggregate cost and scale accounting"
value = "Unavailable: accepted evidence supplies no consolidated token/AIC, service-restart, whole-campaign timing, LOC or exhaustive run-outcome ledger"
source = "t31/HANDOFF.md"
+++

# Conditional lossless Base64 round-trip Audit

## Result at a glance

| Item | Accepted result |
|---|---|
| Verdict | Conditional success for INPUT amended only by operator e-decision-1 Option B |
| Human goal | Store arbitrary binary bytes as text and recover exactly those bytes through the real public slice APIs |
| Original revision | d9ce0dea622ff80eacb736f95e9488cf1eaa06f4; production unchanged |
| Active implementation | Original STANDARD scalar implementation, related to annotated actual-source verification through declared correspondence |
| Named TOP | TOP-01 Engine::encode_slice; TOP-02 Engine::decode_slice; TOP-03 encoded_len; TOP-04 decoded_len_estimate |
| Checked callers | Verification-only borrowed helper-sizing caller and owned-encode/borrowed-prefix-decode caller |
| Independent verification | 179 verified, 0 errors, exit 0; lifetime checking enabled |
| Findings and repairs | Unconditional owned-generic whole-call success refuted by both Drop witnesses; no production repair |
| Trust ledger | 10 direct entries; 2 residual entries; grouped entries are not counts of individual axioms |
| Lifecycle observations | One actual conversion, checked codec writes, complete cleanup, then delivered Result |
| Reliability boundary | Direct std+alloc/no-SIMD replay, rlimit 20; no pooled proof count or performance claim |

## Original goal, interpretation and final claim

The [unchanged original request](INPUT.md) asks for arbitrary-byte lossless recovery, exact counts and prefixes, unchanged live inputs and both destination suffixes, and no unjustified error or panic in the supported round-trip. The [operator decision](t29/authorization.json) authorizes precisely Option B: caller-owned conversion and input-object destruction must complete normally and preserve the observed input, buffers and capacities. Complete destruction includes recursive field cleanup. Normal completion excludes divergence, panic, abort and interruption. Arbitrary caller code is not proved total or effect-free.

Subject to these invocation-specific premises and T1–T11 below, the selected actual APIs establish the requested positive behavior. No codec success, inversion or framing proposition is hidden in the caller premises. Audit explicitly accepts audit.formal-evidence, audit.code-binding and audit.original-goal for this amended conditional result. The unamended unconditional promise remains refuted.

For effective input length n, the supported domain is n <= 3*floor(isize::MAX/4). Let E=4*ceil(n/3) and D=3*ceil(E/4). Valid represented encoding and decoding destinations must have lengths at least E and D respectively. This bound follows from byte-slice extent and representable padded output capacity; it is not defined by implementation acceptance. The decoder estimate can exceed n for terminal tails. Existing destination contents are arbitrary. Empty data, zeros, non-UTF-8 bytes and every processing-block remainder are included.

Encoding returns Ok(E), its prefix is the canonical STANDARD encoding, and decoding exactly that reported prefix returns Ok(n) with precisely the original bytes, order and multiplicity. Err is false in the contracts under these conditions. Internal helper errors and codec panics are not additional permitted outcomes. Inputs remain unchanged while live; each destination retains its represented length and bytes after its reported count. Historical byte snapshots survive consumed ownership logically; no access to a destroyed owner is promised. Vec spare capacity, pointer identity and global heap purity are outside the observation.

## Requested TOP coverage

| ID and requested symbol | Ordinary-language guarantee | Frozen production | Formal specification | Checked implementation | Checked caller | Status |
|---|---|---|---|---|---|---|
| TOP-01 Engine::encode_slice | Sole effective conversion, exact canonical bytes/count, input and suffix frames under B | [production](../.production/src/engine/mod.rs#L227-L256) | [encode contract](../src/round_trip_spec.rs#L46-L71) | [actual public body](../src/engine/mod.rs#L269-L333) | [owned consumer](../src/round_trip_use.rs#L75-L133) | Conditional |
| TOP-02 Engine::decode_slice | Every encoder image recovers original bytes/count and preserves encoded input/suffix under B | [production](../.production/src/engine/mod.rs#L386-L412) | [decode contract](../src/round_trip_spec.rs#L56-L87) | [actual public body](../src/engine/mod.rs#L474-L496) | [owned consumer](../src/round_trip_use.rs#L75-L133) | Conditional |
| TOP-03 encoded_len | Supported padded length is Some(E), without overflow | [production](../.production/src/encode.rs#L101-L128) | [helper contract](../src/encode.rs#L110-L117) | [actual helper](../src/encode.rs#L118-L145) | [helper-sizing caller](../src/round_trip_use.rs#L29-L73) | Verified under ordinary TCB |
| TOP-04 decoded_len_estimate | Actual helper computes D, sufficient for every selected encoder image | [production](../.production/src/decode.rs#L189-L193) | [helper contract](../src/decode.rs#L203-L205) | [actual helper](../src/decode.rs#L206-L210) | [helper-sizing caller](../src/round_trip_use.rs#L29-L73) | Verified under ordinary TCB |

GeneralPurpose::encode_slice and GeneralPurpose::decode_slice are the selected Engine default methods instantiated with GeneralPurpose and STANDARD, not separately omitted implementation bodies. The [registered source root](../src/lib.rs#L320-L340) loads the supporting interfaces/specification/proof/caller modules into the same verifier context.

## Verified usage and material closure

The [borrowed helper-sizing caller](../src/round_trip_use.rs#L29-L73) computes actual public helper sizes and establishes both public preconditions. The [owned generic caller](../src/round_trip_use.rs#L75-L133) moves arbitrary T into encode_slice, receives its count after cleanup and calls decode_slice with precisely encoded_output[..written]. Its capacity implication is about this invocation's returned effective observation. It neither performs a second sizing conversion nor assumes all possible conversions return the same bytes. Independently sized buffers or a caller-specific conversion relation can establish sufficient effective capacity.

This consumer checks owned generic encode followed by borrowed-prefix decode. It does not construct an arbitrary custom owned decoder value. The actual public decoder separately provides its conditional arbitrary-owned-T guarantee when that invocation's sole conversion returns exactly the encoder-reported prefix and satisfies its own B premises. Establishing that relation for an arbitrary owned decoder object is a caller obligation; the checked borrowed-prefix instantiation supplies it concretely.

The [consolidated call graph](t29/CALL-GRAPH.md), [binding index](t31/BINDING.md) and [independent lifecycle Audit](t32/AUDIT.md) classify the complete selected path:

| Edge | Disposition |
|---|---|
| Callers → STANDARD → GeneralPurpose/alphabet/config/table initialization | Checked bodies; canonical readiness is established, not assumed by callers |
| Public wrappers → one as_ref_bytes conversion | Checked wrapper/extraction; exact external interface and caller completion relation are TCB |
| encoded_len / decoded_len_estimate → capacity arithmetic and estimate constructor | Checked bodies and checked domain lemmas |
| encode_slice → expanded padding/helper → internal_encode → concrete zero scalar callback → scalar tail → 32/8-byte packing/table operations | Checked adapted actual bodies; opaque helper contracts do not close these selected edges |
| Encode result → exact prefix → image witness | Checked consuming caller and composition lemmas |
| decode_slice → internal estimate → internal_decode → concrete callback → decode_helper → complete-quads arithmetic → chunk_8/chunk_4 → suffix → metadata | Checked bodies, exact dispatch/success/inversion/write frames |
| Canonical encoding → injectivity → universal decoder witness → recovered original | Checked proof support consumed by actual decoder and caller; no assumed inversion or acceptance |
| DecodeSliceError::from | Actual body checked; blanket forwarding remains TCB |
| Codec-complete Result → complete_owned_cleanup → delivered Result | Explicit external cleanup frame and manually reviewed exactly-once correspondence, T10/T11 |
| Allocation, unchecked, SIMD, streaming and unrelated clients | Residual excluded surface; no selected obligation discharged by them |

Both destinations are accounted for across four observations: entry, sole conversion return, codec/result preparation completion, and complete cleanup/result delivery. Sound exclusive references frame the unpassed destination. Shared byte references preserve live input and the encoder prefix during decode. Checked codec bodies establish bytes/counts/suffixes before cleanup; the external cleanup contract preserves the whole destination afterward and cannot manufacture codec correctness. Snapshots are taken in the actual wrappers. returned_bytes refers to the logical pre-consumption parameter and historical relation, never an executable post-drop dereference.

## Source identity and production correspondence

Original production remains clean at d9ce0dea622ff80eacb736f95e9488cf1eaa06f4. There is no production/API repair. The [full earlier correspondence diff](t21/production-correspondence.diff), [numbered correspondence assessment](t21/AUDIT.md#L1-L70), [Option B lifecycle mapping](t27/LIFECYCLE.md#L1-L41), [independent interface assessment](t28/INTERFACE-REVIEW.md#L1-L39) and final [source/tool identity record](t31/replay.json) identify the active surfaces.

Verification adaptations expand nested helper/padding calls at the original selected path, specialize the concrete scalar callback, replace unsupported iteration with equivalent index loops, express big-endian byte packing explicitly, relocate equal-valued literals/configuration and add ghost contracts/invariants. They retain the actual scalar algorithm and error/validation order. Proof support erases; no proof-specific cfg substitutes an alternate codec. Manual correspondence is an explicit trust condition, not executable byte identity or a compiler-refinement theorem.

Production prepares the inner Result, implicitly destroys owned T, then delivers the Result. Verification prepares the same Result, moves T once into an explicit adapter that calls core::mem::drop, then delivers it. The moved parameter is not dropped again. The adapter's destination reborrow is unused by executable drop code. Recursive fields are included. No leak, forget, duplicated destruction or early cleanup is introduced. Finite original/erased-runtime controls support this correspondence; arbitrary destruction totality and universal unwind correspondence are not proved.

## Complete TCB ledger

There are 10 direct entries and 2 residual entries. TCB-01 through TCB-11 correspond to the accepted T1–T11 inventory; TCB-08 is historical/residual. TCB-12 groups residual excluded source boundaries. These grouped entries do not count individual library axioms. The [full source boundary inventory](t27/boundaries.json) records 14 constructs plus external trait-spec methods; it is an inventory, not evidence that excluded bodies were checked.

| ID | Origin, kind and relevance | Exact source/evidence and trusted proposition | Validation and consequence |
|---|---|---|---|
| TCB-01 / T1 | core; representation/local frames; direct; TOP-01–TOP-04 | [byte-slice representation boundary](../src/as_ref_spec.rs#L35-L40): valid slices have extent <= isize::MAX; sound Rust provenance, borrowing and lifetimes supply local frames | Lifetime-enabled replay and explicit capacity derivation. Unsound aliasing or invalid references invalidate the claim; no global heap purity follows. |
| TCB-02 / T2 | core; external AsRef; direct; TOP-01–TOP-02 | [external interface and slice/reference specifications](../src/as_ref_spec.rs#L5-L32): slices return themselves; references forward with receiver-bound return lifetimes; arbitrary T supplies its actual per-invocation relation | [Pinned core source](t7/source/rust-core-convert.rs#L219-L224), [reference forwarding](t7/source/rust-core-convert.rs#L714-L722) and [slice implementation](t7/source/rust-core-convert.rs#L847-L852) were manually compared. No repeated-conversion stability or arbitrary-T purity is inferred. |
| TCB-03 / T3 | toolchain; frontend adaptation; direct; TOP-01–TOP-02 | [v2 assessment](t7/ASSESSMENT.md) and [exact patch](t7/source/marker-metasized.patch#L1-L28): drop only the synthetic marker-associated MetaSized bound, retaining real external Self/generic prerequisites | Pinned controls reject arbitrary conversion assertions. Existing method-predicate comparison bypass and ABI/safety omissions require exact-method manual checks, including conversion traits. v1 is rejected; no general external-trait soundness theorem is claimed. |
| TCB-04 / T4 | vstd with core/std/alloc models; library semantics and literal axioms; direct; TOP-01–TOP-04 | [slice models](t3/frontend/verus/source/vstd/std_specs/slice.rs#L1-L40), [alphabet literal axiom use](../src/alphabet.rs#L253-L253) and [second literal axiom use](../src/alphabet.rs#L465-L465): sequences, arrays, slicing/indexing/copy/get_mut, strings/ASCII, arithmetic/bitvectors/conversions, Option/Result and FnOnce models match runtime | Exact bundled metadata/library/source identities are pinned in [replay identities](t31/replay.json) and [dependency trees](t15/dependency-trees.json). The underlying real core/library bodies are not all proved here. Literal axioms are explicit, not trusted codec inversion. |
| TCB-05 / T5 | vstd/core; conversion forwarding; direct; TOP-02 | [Into/From models](t3/frontend/verus/source/vstd/std_specs/convert.rs#L8-L51) and [widening models](t3/frontend/verus/source/vstd/std_specs/convert.rs#L119-L146): exact blanket Into forwards to From and widening preserves values | Exact interfaces were reviewed; actual [DecodeSliceError conversion](../src/decode.rs#L123-L132) is checked. Arbitrary user From correctness is not assumed. |
| TCB-06 / T6 | project; panic interface; direct; TOP-01–TOP-02 | [panic model](../src/panic_spec.rs#L1-L12): AssertKind matches the real interface and assert_failed requires false | Codec assertion branches must establish unreachability. This boundary cannot excuse user conversion/destructor panic or permit codec failure. |
| TCB-07 / T7 | toolchain; compiler/verifier/solver/erasure; direct; TOP-01–TOP-04 | [pinned command and identities](t31/replay.json): Rust translation/typechecking/borrow checking, adapted Verus, macros, ghost erasure, imported VIR and Z3 implement their advertised semantics soundly | Independent actual-source replay and identity checks support reproducibility. No independently checked compiler/solver soundness certificate is supplied. |
| TCB-08 / T8 | environment; mixed-vstd compatibility; residual | [historical dependency identities](t15/dependency-trees.json): historical Cargo source-built/mixed-dispatch models require compatible meanings and exact core interfaces | Final direct replay uses bundled metadata for all selected obligations. Historical Cargo counts cannot fill a direct-proof gap; no final integrated Cargo179/0 result is asserted. |
| TCB-09 / T9 | correspondence; executable adaptations; direct; TOP-01–TOP-04 | [numbered correspondence](t21/AUDIT.md#L1-L70): reviewed helper expansion, packing, loops, literal/config relocation and ghost erasure preserve selected original observations/order | Full retained diff and accepted component reviews support transfer to unchanged production. This remains manual correspondence, not executable equality, compiler refinement or performance equivalence. |
| TCB-10 / T10 | project; authorized caller completion/effects; direct; TOP-01–TOP-02 | [environment predicates](../src/as_ref_spec.rs#L5-L32): this invocation's caller conversion and complete cleanup terminate normally and preserve the enumerated observed input, buffers and represented capacities | [Exact authorization](t29/authorization.json) and [independent lifecycle judgment](t32/AUDIT.md) accept these explicit premises. No codec fact appears in them. Callers violating them have no positive whole-call completion guarantee. |
| TCB-11 / T11 | correspondence/project; complete-drop forwarding/frame; direct; TOP-01–TOP-02 | [cleanup adapter](../src/as_ref_spec.rs#L43-L52): exactly-once complete destruction corresponds to production and preserves the whole destination from codec completion through delivery under the cleanup premise | [Lifecycle mapping](t27/LIFECYCLE.md#L1-L41), [independent assessment](t28/INTERFACE-REVIEW.md#L1-L39) and finite compiled controls support correspondence. Recursive fields are included. Arbitrary destructor totality and universal unwind correctness are not checked body theorems. |
| TCB-12 | project; opaque/excluded paths; residual | [opaque padding helper](../src/encode.rs#L70-L86), [opaque generic helper](../src/engine/general_purpose/mod.rs#L180-L190), allocation APIs [encode](../src/engine/mod.rs#L175-L175), [encode_string](../src/engine/mod.rs#L216-L216), [decode](../src/engine/mod.rs#L352-L352), [decode_vec](../src/engine/mod.rs#L411-L411), and [unchecked decode](../src/engine/mod.rs#L514-L514) remain outside selected closure | Selected checked expansions bypass opaque codec helpers. No selected consumer uses allocation/unchecked contracts. SIMD, streaming, tests and unrelated clients are excluded, not silently certified. |

No assume/admit call, uninterpreted codec theorem or new termination exception closes a material codec edge. External trait predicates describe caller relations and explicit caller completion premises. Closed definitions with checked reveal consumers are not unspecified codec axioms.

## Verification and Audit evidence

The accepted final direct command, run from the registered verification root, is:

```sh
.system-proof/t3/frontend/verus/source/target-verus/release/verus src/lib.rs --crate-type lib --edition 2021 --cfg 'feature="std"' --cfg 'feature="alloc"' --rlimit 20
```

[Exact replay driver](t31/replay.py), [command/tool/dependency identities](t31/replay.json) and [readable output](t31/actual-target.log) bind this whole registered src/lib.rs run to the actual public wrappers, both checked consumers, scalar descendants and shared contracts. It uses no module selector and leaves lifetime checking enabled. std implies alloc; default features and SIMD are excluded. This is a direct replay, not an integrated Cargo proof.

Fresh t31 execution returned 179 verified, 0 errors, exit 0. Independent Reviewer replay returned the same result and byte-identical output. Source and external identities remained unchanged. Accepted t32 provides the final original-goal/trust judgment; its additional replay is not added to t31's count. Earlier module, Cargo and mutation counts likewise are not pooled.

The exact adapted binary and Rust/Z3/library identities are retained in the replay record. The pinned v2 frontend patch is a declared trust entry. rlimit 20 is material: retained default10 comparison exhausted resources. The budget increase changes no contract, but no default-budget success is claimed.

The required final registered completion command after report installation is:

```sh
python3 /fastdata/v-boxiuli/noah/exp/tsE/home/skill-packs/system_proof/checks/check_stage.py audit /fastdata/v-boxiuli/noah/exp/tsE/task/verification --production /fastdata/v-boxiuli/noah/exp/tsE/task/verification/.production
```

The registered completion check passed after installation: 179 verified, 0 errors, exit 0, followed by ok. The [in-place verifier output](audit-verification.log) and [completion-check output](t33/completion-check.log) retain the result. The first Cargo/default-budget attempt failed with 178 verified and 1 resource-exhaustion error at decode.rs:174, exit 101; its [verifier output](t33/completion-first-replay.log), [check output](t33/completion-first-check.log) and [runner result](t33/completion-first-latest.json) remain retained. The registered route configuration changed from [the prior Cargo defaults](t33/verus-before.toml) to [direct actual-source defaults](t33/verus-after.toml), selecting the already accepted pinned command and rlimit 20 without source, contract, production or toolchain changes. The emitted command checks the absolute registered src/lib.rs with the same cfg and lifetime checking. This successful completion replay is direct bundled-metadata evidence; it does not establish an integrated Cargo proof or erase the failed attempt. The check validates format/links and registered replay; it does not replace the semantic Audit judgment. Manager completion still requires Reviewer confirmation of the final installed report bytes.

## Findings, counterexamples and decisions

Original production admits sufficient-capacity owned generic inputs whose conversion returns correct bytes and whose destructor then panics. Both encode_slice and decode_slice perform the correct writes and preserve suffixes, but the caller never receives Result. [Historical original-production witnesses](t27/LIFECYCLE.md#L1-L41) and [fresh four-case replay](t28/drop-replay.log) preserve this finding. The fresh log identifies Rust 1.99.1; historical and erased-runtime evidence separately identify pinned Rust 1.98.1. These diagnostic builds are not silently substituted for the final verifier toolchain.

The concrete data are f → Zg== and Zg== → f. Both normal-Drop controls reach caller consumption; both panic-Drop controls stop after conversion/destruction events despite completed writes. This refutes unconditional whole-call successful recovery. It is not a discovered scalar codec inversion defect, and B does not turn it into an allowed codec error. Arbitrary conversion code may likewise panic or diverge. No production bug fix or API change was made.

Option A, a negative-only disposition, was not selected because it would abandon the positive data-recovery objective that the conditional result preserves. Option C, changing production/API behavior, was not selected because that change was not authorized. The operator explicitly selected B. The [original negative Audit](t22/HANDOFF.md) remains valid for the unconditional claim; only the authorized conditional reading supersedes it for positive completion.

Native specdet was operationally inconclusive, and analyze-function was unavailable through its required Copilot route. Accepted source-bound protocol analysis, original finite probes and substantive mutation controls informed specification/correspondence review; none substitutes for universal actual-body verification. The RFC4648 example is a finite semantic comparison, not a standards audit or benchmark.

## Lifecycle, reliability, scale and cost metrics

| ID and metric name | Value and provenance |
|---|---|
| METRIC-01 Fresh actual-target Audit replay | 179 verified, 0 errors, exit 0; independent replay output byte-identical. [Output](t31/actual-target.log). Counts describe one selected context, not a sum. |
| METRIC-02 Pinned replay identity coverage | 33 live inputs; 60 tool and 284 dependency identities; unchanged across replay. [Identity record](t31/replay.json). |
| METRIC-03 Replay interval | 2026-11-04T10:06:30.627973Z through 2026-11-04T10:06:42.081438Z. [Recorded timestamps](t31/replay.json). This is one replay interval, not campaign duration. |
| METRIC-04 Original production Drop lifecycle controls | Four cases: encode/decode crossed with normal/panicking Drop; both panicking cases write correctly then prevent caller consumption. [Fresh log](t28/drop-replay.log). Finite diagnostics, not universal lifecycle proofs. |
| METRIC-05 Resource and analysis limitations | Final direct replay uses rlimit 20; default10 exhaustion retained; native specdet inconclusive; analyze-function unavailable. [Accepted limitations](t32/AUDIT.md). Protocol/finite/mutation evidence was consumed to distinguish specification and lifecycle hypotheses, without claiming complete specialist coverage. |
| METRIC-06 Aggregate cost and scale accounting | Unavailable: accepted evidence supplies no consolidated token/AIC, service-restart, whole-campaign timing, LOC or exhaustive run-outcome ledger. [Final evidence scope](t31/HANDOFF.md). No aggregate is inferred from task numbering, log samples or independent proof counts. |

## Remaining limitations and unproved surface

The claim concerns STANDARD GeneralPurpose, scalar execution, the selected std+alloc/no-SIMD context, valid represented slices and the explicit effective-length domain. Other alphabets/configurations, external malformed strings, insufficient buffers, unrepresentable lengths, allocation internals, streaming, SIMD, timing and complete standards/interoperability behavior remain outside the request and result.

The public generic guarantees are conditional. Arbitrary caller conversions/destructors are not proved total, pure or stable across invocations. Arbitrary owned decoder values must establish their own exact-prefix conversion relation. The checked owned consumer covers owned encode and borrowed-prefix decode; both consumers are verification-only usage witnesses, not newly discovered production application callers. Historical snapshots do not preserve destroyed storage. Complete-drop forwarding, frontend interface matching, library models and production adaptation correspondence retain the stated trust limits.

Audit acceptance concerns INPUT amended only by e-decision-1 Option B under this complete ledger. It never certifies unconditional owned-generic completion. Only Manager declares campaign completion after exact installation confirmation and the successful registered in-place check.

## Evidence map

The principal reading path is [original request](INPUT.md), [operator amendment](t29/authorization.json), the TOP/caller links above, [call-graph disposition](t29/CALL-GRAPH.md), [final actual-target binding](t31/BINDING.md), [replay command and identities](t31/replay.json), [direct output](t31/actual-target.log), [final semantic/lifecycle Audit](t32/AUDIT.md), [production correspondence diff](t21/production-correspondence.diff), [source boundary inventory](t27/boundaries.json), [lifecycle correspondence](t27/LIFECYCLE.md#L1-L41), [independent interface assessment](t28/INTERFACE-REVIEW.md#L1-L39), [Drop controls](t28/drop-replay.log) and [registered completion replay](audit-verification.log). The nonhidden byte-identical report copy must preserve these relative targets through its accompanying evidence aliases.
