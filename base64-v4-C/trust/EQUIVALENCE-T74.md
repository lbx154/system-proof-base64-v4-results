# Selected prepared execution correspondence (review requested)

The reference is T62 direct-positive.rs and its exact direct-input/src/t55_producer.rs,
retained and hashed in identity.json. The ordinary executable client in runtime.rs
performs that same encode/reported-prefix/new/reference-forwarded-observation/
acquire/prepare/release/generic-decode sequence. The adapted caller is specialized.rs,
now registered at shared src/lib.rs as t74. This is a verification-source adaptation
of that selected closed client, not a new generic decoder contract or a theorem
for unknown callbacks. Independent semantic acceptance is required separately from
its body proof. binding.json is a finite fail-closed template checker, not automated
Rust contextual equivalence.

| Site | Original execution | Adapted execution and justification |
|---|---|---|
| Encoder | STANDARD.encode_slice(raw,encoded), unwrap actual reported count | Same public call, arguments, single identity conversion and unwrap. Inherited accepted T52 public event/codec closure. |
| Prefix | &encoded[..written] | Identical borrow of exactly the returned count. Full capacity is never passed as decoder data. |
| Allocation | Producer::new(invalid,prefix); PCell::new(false), ghost Counter/LocalInvariant construction | Same first/second slice references and PCell::new(false). Return real PointsTo separately; Counter and invariant are erased. There is no substitute cell or assumed initialized flag. |
| Ordinary shared observation | shared=&producer; <&Producer as AsRef>::as_ref(&shared), standard reference forwarding then concrete override | shared=&producer; annotated inherent preview with the literal override read/write/if body. Reference forwarding has no executable effects other than this single delegation. No unknown callback is selected. |
| Acquire | Consume producer, destructure and move first,second,flag; ghost invariant into_inner | Exclusive fields/token stay in the closed local path. These additional field moves have no observable effects: slices and PCell<bool> have no custom Drop, no cell-address reference escapes. |
| Prepare | observe through &mut Owned; if preview==second, observe once more | Same preview bytes, equality branch, second observation if selected. Inherent shared-reference signature is used only under closed ownership with a unique mutable ghost permission. The returned slices refer to original invalid/prefix storage, never to flag storage. |
| Release | Consume Owned, move three runtime fields into Producer; ghost invariant construction | Fields/token remain local. No runtime allocation or mutation is performed by ghost invariant construction. Exact state is retained directly, never recovered by assuming precision of the original invariant. |
| Public invocation | engine.decode_slice(producer,decoded); wrapper self binding; input.as_ref once | Move input=producer; engine=&engine; literal concrete callback application reads the same flag once, writes !old once and selects identical second/first reference. This is the selected generic call's specialization, not a call to public decode_slice with a preconverted or preview slice. |
| Actual event | T52 event store from input_bytes | Same ghost store immediately after actual callback. Proof establishes input_bytes@==prefix@ from its actual read, not from an envelope, preview substitution or next-event assumption. |
| Continuation | T52 wrapper internal_decode(input_bytes,output, internal_decoded_len_estimate(input_bytes.len())).map(dm.decoded_len) | Same engine reference, arguments, evaluation order, actual sizing call, decoder continuation and projection. Existing checked projections() lemma is added only as erased proof. Public unused nested item has no runtime effect. |
| Delivery | Result computed inside wrapper, input owned by wrapper and dropped on normal return before delivery; outer locals then clean up | Result computed before input scope ends; input remains owned through continuation, then scope cleanup before delivery. No custom Drop exists in either Producer/Owned representation or slice/PCell<bool>/bool fields. Ghost permissions, counters/invariants have no runtime cleanup effects. |

Concrete state induction: allocation is false; ordinary callback returns invalid
and writes true. Preparation then reads true, returns prefix, and writes false.
Since invalid is noncanonical and prefix is canonical, their bytes differ; the
runtime equality preview==second is true. The preparation skip reads false,
returns invalid, writes true. The actual decoder callback reads true, returns
prefix and writes false. This is four observations in identical order; preparation
is two observations for this selected entry state. Proof checks this with the real
PointsTo, including the equality branch, with lifetime and linearity enabled.

References: raw, invalid and encoded prefix remain alive and immutable throughout
all callback applications. No alias of the owned flag survives a move; shared is
used only for the first observation. No callback receives/exposes flag references,
there is no unknown call capable of accessing it, no concurrency/reentrancy, and
no executable forwarding path remains unaccounted for. The inherent preview may
have a longer Rust result lifetime than the trait override, but the selected
client uses the same underlying input slice references locally and exports only
Result<usize>; no reference lifetime or alias permission escapes to a user.

The rewrite can alter stack frames/addresses, move locations, debug panic metadata
and compiler layout of zero-sized ghost fields. Those cannot select a different
reply here: the concrete callback only reads its own bool and returns stored
slices. No field address inspection or user callback/Drop occurs. This is scoped
to valid safe Rust and T49 normal-return delivery, not stack/resource exhaustion,
panic hook identity, timing, UB or a universal cross-build callback-equality claim.
PCell cell id is ghost; no self-address-based runtime decision is introduced.

Capacity is obtainable before producer creation: runtime.rs calls the actual
encoded_len(raw.len(),true), then decoded_len_estimate(encoded_capacity), creates
those sufficient output lengths and optional extra suffix, and invokes both clients.
The client requires preexisting sufficient destinations, as the original slice API
does. It consumes the real encoder reported prefix and its proved size. The actual
decoder sizing call receives that prefix length; it does not predict a reply.
No existence of allocation for every possible size is assumed. Input preservation
also follows from the actual immutable raw/invalid borrows and exclusive output
borrows, with normal Rust lifetime checks. The checked contract contains mandatory
Ok/count/recovery via decode_post, actual event equality and encoder write_frame;
decode_post contains the complete decoder suffix frame.

TCB: inherited exact T52/T29/T47 source/tool/macro and standard-library interfaces,
Verus/rustc/SMT and normal safe Rust reference/move/drop semantics; PCell::new/read/
write models and ghost erasure. The new direct-token ghost representation and
finite selected source rewrite require the above independent correspondence
judgment. No new compiler patch, assume, external_body, weakened lifetime check,
LocalInvariant semantics, release/reclaim theorem or accepted arbitrary callback
transport is added. Public wrappers and lower codec bodies are unchanged.

Controls: verifier rejects changed initialization, omitted preparation skip,
extra observation, wrong actual-event branch and stale prepared state. Five finite
binding controls reject wrong cell, duplicate read, wrong ownership/drop and wrong
returned reference. Finite ordinary runs compare the original actual Producer and
real generic public invocation with the adapted path over 771 input/capacity cases,
including empty data, all residues modulo three and extra capacity. These supplement
the source/state/alias argument; they are not a universal equivalence proof.
