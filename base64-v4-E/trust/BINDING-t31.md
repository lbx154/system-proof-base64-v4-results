# Final Option B Audit actual-target binding

Fresh registered src/lib.rs replay passed **179 verified, 0 errors, exit 0**. replay.json contains the exact command, timestamps, before/after source and external identities; actual-target.log retains all readable diagnostics. replay.py runs on the registered checkout, never frozen-source. frozen-source preserves exact before inputs. Ordinary lifetime checking remains enabled: no --no-lifetime, --no-verify, --verify-module or other obligation selector occurs. The command explicitly selects std and alloc, excludes simd-unsafe/test, and uses the reviewed --rlimit 20. STANDARD initialization and actual callers coexist with all selected public and scalar bodies in this one context.

## Target-to-entry trace

| Actual target | Shared verifier input and consumer |
|---|---|
| Public Engine::encode_slice / GeneralPurpose default | src/engine/mod.rs:269; consumed by src/round_trip_use.rs:29 and :91 |
| Public Engine::decode_slice / GeneralPurpose default | src/engine/mod.rs:474; consumed by the same two callers and borrowed_decode_use at :10 |
| Conversion and full cleanup | src/engine/mod.rs:641 checked as_ref_bytes; src/as_ref_spec.rs external trait interface and :47 complete_owned_cleanup |
| Public helpers | src/encode.rs:118 encoded_len and src/decode.rs:206 decoded_len_estimate; used at wrapper and caller sites |
| Canonical engine | src/engine/general_purpose/mod.rs:768 checked STANDARD constant, new/config/alphabet/table constructors |
| Scalar encoding | actual internal_encode at general_purpose/mod.rs:86, callback/scalar tail, encode_thirtytwo/eight/read_u64; actual wrapper padding expansion consumes checked add_padding |
| Scalar decoding | actual internal_decode at general_purpose/mod.rs:105 consumes decode_helper at general_purpose/decode.rs:95, complete-quads :245, chunk_8 :633, chunk_4 :763, suffix in decode_suffix.rs:224 and metadata constructor |
| Canonical witness and frames | round_trip_spec native contracts, encode_bridge capacity, round_trip_proof injectivity :101 and universal lifting :136 consumed by actual internal_decode and checked caller composition |

src/lib.rs imports engine at :300, alphabet :303, encode :305, decode :312, round_trip_spec :328, as_ref_spec :331, round_trip_use :334, panic_spec :336, encode_bridge :339 and round_trip_proof :342. general_purpose/mod.rs:15–16 imports actual decoder modules. source-index.txt is a located textual index, including cfg-excluded declarations; it is not a claim that tests, SIMD or naive implementations were verified. The command and source cfg establish selected inclusion.

## Accepted closure and conditions

prior-call-graph.md is the unchanged accepted complete material path; prior-trust.md, prior-T1-T9.md and prior-lifecycle.md together retain T1–T11 and production correspondence. Their hashes remain tied to t29's receipt through upstream-checks.json. No dependency changed, so component/correspondence findings are reused rather than reopened. T10 conditions caller AsRef and complete destructor/field cleanup termination/effects; T11 retains manually reviewed drop forwarding and the external cleanup frame. This replay does not prove arbitrary caller code terminates. Codec success, error exclusion, exact lengths/bytes, injectivity and suffix frames remain actual-body obligations, never T10/T11 assumptions.

The owned generic caller consumes T at encode and decodes the exact reported borrowed prefix. Arbitrary owned decode has a conditional public contract, not an arbitrary owned decoder constructor in that caller. Domain remains arbitrary effective returned bytes with lawful helper-derived represented capacities. Both unconditional public guarantees remain refuted by original-production panic-Drop witnesses retained in accepted t27/t28. The exact Option B authorization is retained as authorization.json; no new premise selection occurs here.

Direct replay imports pinned v2 bundled vstd metadata/macros; manifest-vstd sources are identified separately. Historical mixed-vstd Cargo evidence and focused counts cannot be added to this direct 179/0 result. This task asserts no fresh Cargo success. t33 owns the registered in-place completion replay and final report installation. rlimit20 is unchanged from the reviewed candidate (historical default10 exhaustion remains disclosed). No standards-compliance or performance comparison follows from proof counts.

No substantive source, contract, tool or production change occurred. No new material gap was found in replay/binding; full audit.formal-evidence/code-binding acceptance remains an independent judgment with the cumulative trust inventory. t32 owns original-goal/lifecycle judgment. Any material semantic or trust gap requires Prove rollback, not repair here.
