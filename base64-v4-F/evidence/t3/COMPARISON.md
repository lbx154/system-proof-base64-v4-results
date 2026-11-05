# E maintenance experiment

The selected executable change needs **zero proof edits on preserved E**: the changed crate verifies **179 verified, 0 errors**, default limits, exit 0. No repair round was needed or attempted. This example therefore supplies no evidence that a reorganized proof reduces maintenance repair cost relative to E. The original E failures were pre-existing budget exhaustion, not false statements; the arithmetic refactor itself changes solver behavior enough to close them. The 32-byte query remains near its budget, so this is not evidence that E is robust under other changes.

## Source and reproduction

The scratch tree was extracted from t1/preserved/e-source.tar.gz. Only src/engine/general_purpose/decode.rs differs, exactly by executable.patch. proof-repair.patch is empty. No archive, production, verifier or registered candidate source changed. t2 owns final candidate integration. Original production revision d9ce0dea622ff80eacb736f95e9488cf1eaa06f4; task worktree baseline ac5ce76986c9f52f8f5c4adbc8d26ceabcb3c7e1. All contracts, proof code, Cargo configuration and generic/environmental conditions from accepted t1/GROUND.md are retained.

Run `bash .system-proof/t3/measure.sh` from the directory containing these artifacts. It uses t1's registered env.sh, supplied Verus 0.2026.09.29.d85f757.dirty (tangruize/verus d85f7578 plus supplied marker-metasized.patch), Rust 1.98.1, bundled Z3 4.16.0. It uses the same direct std+alloc/no-default-feature configuration as t1. No limit flags, function limit attributes or solver options were added. The script retains process resources, JSON, stderr, status, generated SMT and same-Z3 serialized query replay separately. Replay filenames beginning baseline are the timing skill's generic output names, not an assertion that this is original E.

## Executable equivalence

The maintenance change computes each loop's input start once, takes start..start+width, and reuses start for the decode offset. This makes the selected block and offset visibly agree, removing repeated index arithmetic.

For the first loop let A=input_index_start, U=unrolled_loop_len, k=chunk_index, w=32. The guard k<U/32 and U a multiple of 32 imply (k+1)*32<=U; A+U=input_unrolled_loop_end<=input_index_end<=input.len(). Consequently k*32, k+1, (k+1)*32, A+k*32, A+(k+1)*32 and (A+k*32)+32 all fit usize. Both old and new slice endpoints are exactly A+k*32 and A+(k+1)*32. UNROLLED_INPUT_CHUNK_SIZE is the constant 32. Moving input_index before the slice introduces no possible arithmetic panic on these reachable iterations.

For the remaining loop let A=input_unrolled_loop_end, R=input_index_end-A, k=chunk_index, w=4. Its guard k<R/4 implies (k+1)*4<=R. Thus the same nonoverflow argument holds with width 4 and endpoint bounded by input_index_end. The new input_index equals the old expression passed to decode_chunk_4. These facts apply for arbitrary input bytes on every valid call to this private implementation, not just canonical encoded data. The function's valid input range and output capacity are its existing preconditions, established by its caller. The full changed Verus run checks the arithmetic/slice safety obligations along with the unchanged invariants.

Equal endpoints select identical bytes in identical order. All chunk/output slices, four decode_chunk_8 calls or one decode_chunk_4 call, decode table, output bases, call order, `?` propagation, iteration counts and writes remain identical. Decode error offsets receive identical start values (and +8/+16/+24 in the first loop). No writes or fallible operations were moved across a decode call. Hence success values, errors, partial writes on error, suffix frames and byte observations agree. Checked and unchecked arithmetic modes agree because all changed intermediate arithmetic is representable on reachable iterations. This is a source-level equivalence argument, not a separately machine-checked compiler refinement theorem; existing E correspondence/TCB conditions remain.

## Measurement

| Observation | Original E (accepted t1) | E + executable.patch |
| --- | --- | --- |
| Actual whole-crate default result | 177/2 | 179/0 |
| Process wall seconds | 9.71 | 9.65 |
| Maximum RSS KiB | 582828 | 578560 |
| 32-byte source loop replay | unknown/canceled; 30,000,000 units; 1.911 s | unsat; 28,210,723 units; 2.289 s |
| 4-byte source loop replay | unknown/canceled; 30,000,000 units; 1.673 s | unsat; 9,100,169 units; 0.454 s |

Loop spans remain decode.rs:284 and :390. Replay times are serialized generated-query observations, not native individual-loop wall time. These single observations do not support a stable speedup claim; the first changed query even takes longer in replay despite consuming fewer resource units. Native function/module aggregates and all raw data remain in the JSON files; no work was extracted into new lemmas. E's supplied historical raised-limit number was 179/0 with --rlimit 20; this experiment matches that count without using that option. No raised-limit run was made here.

Measured repair scope: 0 proof lines, 0 proof files, 0 added lemmas, 0 strengthened assumptions/contracts. executable.patch has three local hunks in the two loops. There are no residual obligations in this changed-E run and no new failures to repair.
