# Source-bound lifecycle account under the Manager's adjudication

## Claim and authority

The original objective is lossless STANDARD encoding and decoding, exact reported prefixes, unchanged input, both destination suffixes, and mandatory success rather than codec error/panic. Original production revision: `47fa113a5c309fd7dcf5618d0f738cc9056d4402`, clean in this run. Configuration: default features absent, `std` and its implied `alloc`, no SIMD. No production or verification executable/contract source was changed by T49.

`INTENT-QUESTION.md` was submitted to Manager, with Planner and T47/T48/T50 copied. `adjudication.json` retains the exact Manager decision at 2026-11-03 13:25:07 UTC and Planner instruction. Manager selected the second reading for completion, preservation, and continuation. This is specification authority, not proof: codec must succeed for each lawful actual normally returned byte snapshot; external conversion and cleanup outcomes remain explicit; full public return requires those intervals to complete; consumed objects and arbitrary globals are not preserved. No new public purity/totality premise is installed. Remain in Ground.

## Actual-source ordering

Production `src/engine/mod.rs:227` encode_slice and `:386` decode_slice take owned `input: T`, borrow it for exactly one AsRef invocation, and pass that returned slice to their original nested inner helper. The outer body is a tail expression, but this does not bypass cleanup. The input borrow ends before owned input cleanup. No returned byte reference escapes the public Result.

Fresh rustc 1.98.1 MIR emitted from the ORIGINAL library identifies the same pattern for each wrapper:

```
AsRef(&input) -> bb1, unwind bb4
bb1: return_slot = inner(actual_slice, destination) -> bb2, unwind bb4
bb2: drop(input) -> bb3, unwind continue
bb3: return
bb4 cleanup: drop(input) -> bb5, unwind terminate(cleanup)
bb5 cleanup: resume
```

Thus inner success, public result delivery, and subsequent caller execution are three distinct events. `mir-unwind.json` and `mir-abort.json` retain exact bodies and hashes; full production MIR and source identities are retained. The textual checker accepts both real bodies and rejects redirected cleanup edges. MIR is unstable compiler output, analyzed under the pinned compiler semantics; this is source/control-flow evidence, not Verus verification or a Rust semantics proof. `panic=abort` has unreachable unwind edges and cannot recover with catch_unwind.

## Composition rule and its limits

For one concrete execution, let C be conversion, K be the actual codec continuation, D be full owned cleanup (including any field destructors), and R be delivery to the caller. Snapshot b belongs to C's actual normal return event, not to an arbitrary admitted result or an earlier preview.

* C normal(b); K normal(Ok(n)); D normal gives R normal(Ok(n)). The next caller statement is then reachable, subject to that statement's own operations and enclosing scope cleanup. This is a rule of source sequencing; the required component facts must be supplied, not assumed for arbitrary T.
* C unwinds: K is not entered; D runs during unwinding. D normal propagates unwind; D panic aborts. C divergence/abort reaches neither K nor R.
* K unwinds: D runs on cleanup, not normal return. Supported-call correctness must rule out codec-originating unwind; classifying it as external would violate the objective.
* K normal; D unwinds/diverges/aborts: the return slot may already contain Ok and correct destination bytes may exist, but R normal is absent. A catch_unwind recovery path does not receive that Ok and is not the requested round-trip continuation.
* No loop, panic, global-effect, or field-cleanup constraint follows from AsRef's trait signature. Normal-return postconditions do not supply these facts. Termination and effects are separate proof obligations for any nontrivial concrete callback/destructor claimed in a whole-call theorem.

At C's actual return, encode requires supported(|b|) and E(|b|)<=destination capacity; image decode requires b=encoding(original), supported(original length), and the justified decoder capacity. K must terminate and return Ok with exact bytes/count/frame, never merely satisfy a postcondition if it happens to return. These mandatory-success obligations remain intact. T47 must export actual-event identity; T48 must justify obtainable domain/capacity from lawful pre-call state; T50 must cover the original generic wrappers. T49 supplies no substitute for any of these machine-proof boundaries. No new public event theorem or generic total-correctness theorem is claimed here.

## Destination frames across conversion and cleanup

Attribution by itself proves no frame. A claimed full-call frame composes three intervals: C must preserve destination memory before K; K establishes exact written prefix and unchanged suffix; D must preserve destination memory after K. A generic effect predicate naming a user callback is insufficient.

For this API the destination is a live exclusive Rust `&mut [u8]` throughout these intervals. In a valid safe execution, T, its destructor, another safe thread, and panic hooks cannot simultaneously retain and exercise a conflicting capability to those same plain-u8 locations. Owned fields and disjoint globals can change freely. A disjoint `Rc<RefCell<Vec<u8>>>` is mutable, but cannot legally be a second active access to the already exclusively borrowed destination. The input shared byte slice is immutable while K uses it; no UnsafeCell is present in plain u8. Library/runtime/unsafe code must respect Rust's reference validity and aliasing contract. Unsound unsafe implementations, external writes to borrowed memory, FFI races, process termination and machine failure are not modeled as valid safe executions. This is the explicit Rust safety/compiler-library boundary, not a new purity requirement on safe user callbacks.

This separation justifies C/D destination preservation on normally completing paths; actual checked K frame still supplies the substantive codec write bound. It also explains why the observed Drop-panic cases retain exact prefix/suffix after recovery, without turning recovery into success. After R, the caller regains the destination and may deliberately modify it; there is no permanent global frame. Callback mutation of its own consumed input after K remains allowed and is not confused with mutation by K of live borrowed source bytes.

## Covered borrowed-byte public usage

The actual checked caller is `.system-proof/t31/caller.rs::caller`, included by `src/lib.rs` as `round_trip_caller`. It derives encoded_len and decoded_len_estimate requirements, calls the real public encode_slice, passes precisely the reported prefix to decode_slice, unwraps both required successes, and checks recovered length/content and both complete destination frames. This source is derived from public documentation/tests, not a trusted consumer effect.

Here T is `&[u8]` at BOTH invocations. Exact core Rust1.98.1 code is archived in `core-convert.rs`: lines714-722 reference AsRef forwards once to the referent; lines847-852 slice AsRef returns self. These two finite bodies have no user callback dispatch after slice identity is selected, no loop, allocation, or panic path. The exact core interfaces consumed by Verus are `round_trip_proof::reference_as_ref` and `slice_as_ref` in the SAME selected crate. They remain reviewed standard-library interface trust rather than newly verified core bodies. Dropping a shared reference invokes no referent destructor and has no field cleanup; the input slice and prefix references therefore cannot insert custom Drop, unwind, mutation, or divergence. The caller borrows input immutably and each destination exclusively. Array/slice/usize and Result<usize, ...> values in this caller introduce no custom cleanup. Allocation is outside the caller: adequate actual buffers are supplied, not assumed obtainable by allocating the largest representable buffer.

Codec termination is NOT inferred from normal-return posts. The checked scalar loops have `input.len()-input_index` decreasing variants; decoder unrolled/remainder loops have remaining-chunk variants; suffix scan/emission have remaining-input/emission variants; table constructors have remaining64 variants. Padding traverses finite 0..pad_bytes with pad_bytes<=3. Remaining selected fixed-size chunks/read/config/forwarding/sizing operations are finite, with no executable recursion or user callback inside the concrete STANDARD continuation. The accepted T38/T44 closure checks variants and arithmetic/bounds/unwrap/assertion paths on the adapted actual bodies under reviewed compiler/std interfaces. Original loop/control and public/helper transfer for this CONCRETE borrowed case relies on accepted T40 correspondence, not general callback coupling. This account assumes execution of finite primitive instructions makes progress; it promises no wall-clock bound, scheduling fairness against arbitrary process suspension, allocation service, or hardware fault tolerance.

Fresh recheck of the unchanged T44 common selection: **149 verified, 0 errors**, `--rlimit 20`, normal lifetime/erasure/recursion/termination checks. Same input/contract context includes actual public wrappers, both implementation chains, sizing/support, `round_trip_proof`, `encode_observation_use`, `round_trip_caller`, and decoder image consumer. `verify-result.json` hashes all34 Rust/caller inputs before and after (stable); `verus-tool-identity.json` matches all reviewed T29 tool artifacts. No claim follows for unselected generic consumer obligations. T28's preexisting scalar rlimit80 remains. No further replay is needed unless inputs change.

Therefore this borrowed caller composes identity conversion, mandatory finite codec computation, no-op reference cleanup, result delivery, then its concrete verification assertions and next decode call, under the explicit inherited TCB/correspondence. This is a concrete closure with source-grounded lifecycle reasoning, not a newly machine-checked general cleanup semantics theorem. Independent review must confirm the same-context interface and caller link. Generic T47/T48/T50 remain separate.

## Counterexamples and evidential classification

`replay.py` builds original production directly, never the adapted proof crate, under Rust1.98.1 and both unwind/abort strategies. Eight modes times two APIs times two panic strategies =32 process cases. Each normal/recovered case checks a single conversion, one destructor invocation, exact writes, preserved suffix and whether the statement immediately following the public call executed. Modes cover normal result, safe disjoint shared-state mutation, actual owned input erasure with an external audit copy, conversion panic, Drop panic, double panic, conversion loop and Drop loop. Empty/reference lifecycle or arbitrary byte correctness is not inferred from these finite cases.

Owned erase returns `f`/`Zg==` from its own Vec, then Drop zeros that actual Vec and publishes the erased bytes; the codec result is still correct. Shared mutation publishes effects without preventing success. Conversion panic writes nothing; Drop panic follows exact writes but prevents public Ok delivery and caller continuation. Under unwind, a conversion panic followed by destructor panic aborts (SIGABRT); under abort strategy either panic aborts. Loop cases are killed after2s after an entry marker; the timeout is a bounded observation, while the explicit unconditional loop is the source witness for a possible nonreturning user interval. None is a codec failure counterexample on a lawful normally completing callback path.

These runtime cases are counterevidence to unconditional arbitrary-owned-T completion/purity; they do not prove universal frames, generic event binding or termination. The two byte constants agree with RFC4648 section10's f/Zg== vector already archived in T39. No performance number or standards-compliance claim is made. Rust destructor/panic/catch_unwind primary documents and hashes are archived in references.json; these support classification, not replacement of actual code checking.

## Review request and remaining boundary

Please independently confirm borrowed &[u8] core forwarding/identity, reference cleanup, legitimate destination separation through BOTH conversion and cleanup, checked decreasing concrete continuations, and actual t31 caller all compose in this same proof context, with inherited T29/std/T40 trust explicitly accepted. Distinguish machine-checked body/content/frame facts, source/MIR lifecycle reasoning, and finite runtime witnesses. Confirm that no arbitrary user-code completion/purity has been introduced.

For a generic whole-call result the exact conditional scope is: actual event b satisfies lawful codec domain/capacity with justified caller obtainability; original actual continuation is covered; C and full D for that invocation finish normally; and Rust separation (or independently verified effect preservation) carries the claimed frame through external intervals. Only then may Ok delivery and the specified caller continuation be composed. This is an explicit conditional rule and unfinished interface, not proof that arbitrary T meets it. No formal generic public-call machine proof is supplied by MIR/runtime or the Manager decision. T47/T48/T50 remain open; no Ground advancement or universal original-goal completion is requested.

## Concurrent integration notice

At installation, shared `src/encode.rs` and `src/engine/mod.rs` differed from the34-input stable task snapshot because other tasks changed them. The fresh149/0 evidence is scoped to the complete `selected-source/` snapshot retained here; it is NOT verification of those newer shared sources. T49 changes no source and overwrites none of those edits. Recheck the integrated actual public interface/closure after T47/T50 settle; apply the same lifecycle rule to that resulting source and confirm its ownership/cleanup ordering. This source-context dependency prevents treating the old concrete proof as current for a newly changed public interface.
