# Production-to-verification behavioral correspondence

Baseline: clean registered production 47fa113a5c309fd7dcf5618d0f738cc9056d4402.
Current candidate: exact selected-inputs.json (accepted T52 installed37), not Git
HEAD alone. Read t40-handoff/CORRESPONDENCE.md C1-C11, TRACE.md, certificates and
source snapshots for the full retained erasure/reconstruction argument. T73's
binding.json refreshes the 12 changed target bodies against accepted T47; exact
37 identity establishes currentness of remaining accepted context. This is a
composed checkable behavioral argument submitted for review, not a machine proof
of Rust contextual equivalence. Production is read-only and untouched.

| Adaptation / actual body | Verus reason | Checkable preservation argument and evidence |
| --- | --- | --- |
| C1 constants, alphabet/config/table constructors and accessors | Models/witnesses needed for STANDARD; const erasure compiler failure | Same literal bytes, assignments, loops, struct fields and initialization order after selected ghost erasure. T33/T40 concrete production anchors. Actual compiler changes are separately trusted; source identity does not prove compiler correctness. |
| C2 encoded_len/decoded_len_estimate and estimate forwarding; error From | Arithmetic/projection and external trait spec interfaces | Executable Option/checked arithmetic branches and fields unchanged. T32/45 body checks; T50/T52 private contract changes are ghost-only. Physical bound is a validity interface, not a new runtime guard. |
| C3 Engine::encode_slice and expanded encode_with_padding | Default-method nested-helper dictionary cycle; do not restore it | Expand literal helper body with self, one input.as_ref(), output bindings evaluated in original argument order; keep nested item. Padding lexical expansion evaluates the same arguments, then the same debug assertions, encoding and conditional padding. Early Err/Ok values and prior writes match; locals are references/usize without user Drop. T40 bridge.py directly reconstructs production tokens; T47 adds one erased event store; current T52 binding preserves those executable tokens. |
| C4 GeneralPurpose::internal_encode | Default/helper dictionary cycle and concrete zero-prefix proof | Execute same zero-capture callback once, with same buffers and zero indices, then same scalar tail. Literal expansion does not skip callback. No SIMD. T38/T40 reconstruction matches retained original helper and actual call. |
| C5 scalar encoder, read_u64, padding | Word/sequence bit arithmetic, slice conversion interface | Original unrolled/group/remainder writes and bounds remain after ghost-loop erasure. read_u64 remains from_be_bytes(s[..8].try_into().unwrap()). add_padding still traverses same modulo count and writes byte61. T38/T29 and T40 certificates. Actual project bodies checked; std conversion values trusted. |
| C6 Engine::decode_slice and Result map | Default/helper cycle; map post projection | Expand actual inner call; one conversion before estimate/internal_decode/map, same metadata projection |dm|dm.decoded_len and errors. Input stays outer-owned. T31/T40 exact reconstruction; T47 erased store and T52 annotations preserve sequence. |
| C7 GeneralPurpose::internal_decode | Concrete helper composition and zero-prefix proof | Bind same estimate/table/settings and literal closure; boundary first, callback once, quad loop, suffix in same order. Administrative Result locals preserve every ? exit and prior writes. T30/T40 retained normalization certificates. |
| C8 decode_complete_quads | Iterator model unavailable | Counted loop enumerates identical input intervals and output offsets, with chunk8 calls at 0,8,16,24 and destinations 0,6,12,18; remaining quad order identical. Multiple-of-four structural domain prevents dropped remainder; actual caller establishes it. Counters bounded, no destructible iterator item/user iterator. T27 schedules/reconstruction plus T40 direct production anchor. |
| C9 decode_chunk_4/8 be32/be64 | Anonymous-const endian external signature integration | Trusted wrapper executes the same to_be_bytes exactly once on same Copy word; same first3/6 copy. No branch or user Drop added. T24 reconstruction/changed-shift controls and current binding. New wrapper frames are declared exclusions. |
| C10 decode_suffix | Unsupported for-loop continue; absent get_mut postcondition | Counter visits same input_index+k; padding branch skips data actions via else instead of continue and advances once. Emission visits same 0..append_count; explicit bounds error equals concrete get_mut/ok_or/? partition and makes no failing write. Same byte/error/offset/prior writes. T34 accepted structural-domain proof, reconstruction, controls and corpus; T30 establishes domain. |
| C11 metadata/projection/caller and erasure | Consumer composition and event capture | Struct construction and map projection unchanged. Ghost slots and snapshots erase; actual client consumes only encoder-reported prefix, never unused capacity. T47 annotation call lowering erases to ordinary original arity. T49/T52 MIR pins conversion, result computation, cleanup and return placement. |

T50 changed only private helper contracts/ghost proofs, no executable call or public
restriction; T47 changed only erased event stores and annotation call lowering;
T52/T54 changed only contracts/proofs and registered diagnostic consumers. T43/45
FromSpecImpl is checked ghost interface, not a trusted conversion replacement.
These additions need source and macro erasure review, not helper restoration.

## Generic transfer rather than callback coupling

The literal wrapper expansions cannot be universally contextually equivalent to
production with respect to every possible Rust observation: helper frames disappear,
wrapper frames appear, stack addresses/backtraces, panic locations/hooks and timing
can differ. A safe arbitrary callback may inspect those and select different b.
The original task excludes timing and allocation internals; acceptance of other
diagnostic exclusions is still explicit. No claim of equal user-global effects,
stack/resource exhaustion, double-panic behavior or undefined execution is made.

The goal transfer argument uses a stronger uniform codec fact instead of pretending
that callbacks agree. In either execution, retain its own single actually returned
b; after conversion, the selected STANDARD continuation is deterministic from b,
configuration and complete destination. Each expansion preserves those concrete
continuation operations, count/error branches and writes. For every supported b
and sufficient capacity, the checked continuation supplies mandatory Ok/content/
frame. Thus varying which b a callback selects does not change the local guarantee.
The same public caller decodes the corresponding execution's reported encoded
prefix; identity borrowed-slice AsRef supplies that prefix. User cleanup is still
outer-owned and delivery is conditioned as T49 requires. This argument transfers a
per-actual-event property, not equality of b, conversion effects or cleanup outcomes
between builds. It is a semantic induction over C1-C11 under explicit Rust/library
TCB, not an automated simulation theorem; Reviewer acceptance is requested.

For arbitrary callbacks that return noncanonical data to a separate decoder call,
no unconditional recovery is owed by the unchanged goal. For a changing producer
whose preparation is supposed to select the encoder image, a checked entry-to-event
selection argument remains necessary to prove that client's unconditional use.
This is P-CALLER, not a hidden callback-coupling assumption or a compiler gate.

## Trust and limitations

Trust inventory is t52-handoff/TCB.md plus inherited t40-handoff/CORRESPONDENCE.md:
Verus/rustc/SMT and machine widths; reviewed exact T3 deferred-initializer and T29
closed-array-signature patches; regenerated vstd and macro artifacts (six tool
hashes, three macro hashes in replay identity); safe Rust slice/reference/index,
copy/arithmetic/Option/Result/AsRef models; standard array conversion/endian value
interfaces; narrow actual valid u8 slice size<=isize::MAX boundary. No allocation
existence, raw-pointer validity or ZST bound follows. The macro twin-entry mechanism
checks annotation bodies and emits ordinary original-arity bodies; erased verified
entries are panic stubs and are not executable codec proofs. Accepted T47 erasure
TCB transfers to selected inherited default methods with STANDARD, not arbitrary
Engine twin-entry overrides. Core implementations and compiler patches are not
proved by codec success counts. boundary-locations.json gives current locations,
not automatic trust acceptance. Cell get/set models in registered clients remain
opaque; they provide no state retention or generic next-event canonicality.

No project codec body or desired client effect is trusted. Every in-scope edge is
listed in GROUND.md; compiler availability, original-helper restoration and the
abandoned ownership simulation are omitted as goals, not relabeled as assumptions.
Source transformations, erasure trust and per-execution generic transfer require
independent review. Finite RFC/corpus comparisons supplement, never prove, these
relations. Selected borrowed-slice transfer is supported by earlier accepted T40;
unrestricted Rust contextual equivalence is not claimed or required by the new
route, and cannot be inferred from token reconstruction.
