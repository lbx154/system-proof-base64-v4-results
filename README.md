# base64-v4 审阅包：A、C、E 组

> 网页版（全部材料全文、可折叠）：https://lbx154.github.io/system-proof-base64-v4-results/

任务：system-proof-benchmarks 的 `base64-v4`。目标是证明 base64 crate 用 STANDARD 引擎时，`encode_slice` 再 `decode_slice` 能对任意字节原样往返。生产代码的版本固定为 `47fa113a5c309fd7dcf5618d0f738cc9056d4402`。

两组用的都是 TypeScript 版 Argus（[lbx154/Argus](https://github.com/lbx154/Argus)）加 `system_proof` 技能包。技能包不在本仓库，这里只放两组的成果。后端是 codex，模型是 gpt-6.1-sol。

## 一眼看结论

| | A 组 | C 组 |
|---|---|---|
| Argus 版本 | feat/ts-core（无 worktree，串行） | feat/ts-core + task worktree（#425）+ 边做边协作（#432） |
| 用时 / 花费 / 任务数 | 3h06 / $18.69 / 19 | 5h55 / $104.12 / 82 |
| 并行 | 基本串行 | 同时开着的任务最多 13 个 |
| 插件的原地审计重放（`verus-run verify <verification> --preset standard`） | **不通过**：在原地只验证了 1 项（`inplace.log`） | **通过**：172 verified, 0 errors（`audit-verification.log`），我在另一个 worktree 上独立重跑也是 172/0（`independent-replay.log`） |
| 证明的对象 | 抽出来的副本（打过补丁的 Verus 上 146/0；原版 Verus 编译不过） | 验证 checkout 里真实的 crate 源码；生产代码一个字节没改 |
| 是否改了 Verus | 是（TCB-10/11：编译器签名、宏、continue 的翻译） | 是（T3、T29 两个编译器补丁，加一处 vstd 增补，见下文） |

A 组的结果过不了插件自己的审计。C 组拿到了在真实代码上、原地通过审计的完整功能证明，但前提是那两个 Verus 补丁可信。

## 请优先审的三件事（都在 C 组）

1. **两个 Verus 编译器补丁，风险最大**
   - `base64-v4-C/trust/verus-T3-compiler.patch`：+34/−20 行，改 `rust_verify/src/verifier.rs` 和 `rustc_hir_analysis/src/lib.rs`。作用是把常量初始化的求值推迟到 THIR 擦除初始化之后，而且仍然检查所有未用到的常量。目的是绕开 Verus 对常量处理的一个限制。
   - `base64-v4-C/trust/verus-T29-compiler-combined.patch`：+73/−22 行，改 `rust_to_vir_func.rs`、`verifier.rs` 等文件，修的是“closed-array signature”的问题。这个补丁改的是 Rust 函数翻译成 VIR 的部分，最可能影响可靠性，请重点看。
2. **vstd 增补**：`trust/vstd-T29-delta.patch`，+22 行。只加了一条 broadcast axiom，内容是 `<[T; N]>::try_from(&[T])` 成功当且仅当 `len == N`，成功时内容相同。我核对过，和 Rust 标准库的行为一致。
3. **验证副本里的 11 处行为等价改写（C1–C11）**
   - `trust/ADAPTATIONS-T80.md`、`trust/EQUIVALENCE-T74.md`，完整差异在 `proof/verification-vs-production.patch`（验证 checkout 的 `src` 对比生产的 `src`，+1442/−69，大部分是证明注解）。
   - 改写包括把 trait 默认方法里形成字典循环的调用就地展开（Verus 处理不了这种循环）。这条路线是操作员在 11-03 15:10 明确批准的（`operator/route.txt`）。
   - 改写和生产代码的对应关系是 Reviewer 审查论证的（报告里的 TCB-11/12），不是机器证明的。

## C 组证明了什么

最终报告在 `base64-v4-C/final-report.md`，由 Reviewer 撰写，SHA-256 `91b7864f…0ad2c6`。

- 规约是独立写的（`proof/round_trip_spec.rs`），按 RFC 4648 STANDARD 逐字节定义编码：字母表、每 3 字节变 4 个字符、用 `=` 补齐。
- 证明了真实的 `encode_slice` 输出逐字节等于这个编码，返回的长度是 `4⌈n/3⌉`。用报告的那段前缀去 `decode_slice`，能拿回原字节，返回 `n`。输入不变，两个目标缓冲区后面的部分不变。在支持范围内不会出现 codec 错误或 panic。
- 检查用的调用方在 `proof/caller-t31.rs`，写法照抄库自带的 doctest：只取编码返回的前缀去解码，最后断言 `decoded[..n] == input`。
- 证明里没有 `assume` 也没有 `admit`。8 处 `external_body` 里：
  - 5 处是这条调用路径用不到的便利接口（`encode`/`decode` 的 String/Vec 版等，TCB-15）；
  - 其余 3 处是 `be32`/`be64`（TCB-08）和“合法切片长度不超过 isize::MAX”（TCB-04）。
- 前提条件：
  - 目标缓冲区足够大；
  - `n ≤ 3⌊isize::MAX/4⌋`；
  - 用户的 `AsRef`/`Drop` 正常返回（TCB-13，`trust/LIFECYCLE-T49.md`）；
  - 只在 x86_64 上重放过。
- 不覆盖：SIMD、流式接口、其他字母表和设置、畸形输入。
- 佐证：RFC 4648 第 10 节的 7 个测试向量，加 771 个二进制往返运行检查，全部通过。

## A 组

- `base64-v4-A/final-report.md`：A 组自己的报告，声称有条件地证明了往返。
- `inplace.log`：插件审计重放在原地的结果，1 verified。也就是说，它的证明并不在验证 checkout 的真实源码上。
- `recheck-patched.log`：在它打过补丁的 Verus 上复核抽出来的副本，146/0。
- `recheck-stock.log`：用原版 Verus 复核，编译不过，缺少它自己加进 vstd 的 `slice_array_result_contract`、`enumerate_postcondition`。
- `drift.txt`：副本和生产代码的差异，其中有改过的错误类型（`EncodeSliceError::OutputSliceTooSmall`）。
- A 组跑的时候，技能包还没有恢复审计重放检查，所以 A 自己没发现这个问题。这个漏洞已经在技能包里修好。

## 过程上的事实（C 组）

- 时间线：09:59 开始，ground 到 15:20，prove 用了 16 分钟，audit 用了 18 分钟，15:54 完成。
- ground 阶段的大部分时间花在攻 Verus 编译器内部，想在不改源码的前提下处理 trait 字典循环。15:10 操作员批准了改写路线，8 分钟后就过了 ground。
- 操作员一共介入 3 次，原文都在 `operator/` 里：
  - `continue.txt`：Manager 自己判了“尝试失败”、想结束时，要求它继续；
  - `wide.txt`：要求尽量多开并行；
  - `route.txt`：批准改写路线。
- 运行期间换了 5 次 Argus 版本（边做边协作的几次迭代），每次重启都从原处接着做，记录在 `events.jsonl`。有一次（13:37–14:00）因为“同一任务上两次‘先看’同时进行”的 bug 停下，已经修好并补了测试。
- 四个角色的协作次数：回合中途写信 159 次，Reviewer 被请去“先看”130 次，Planner 实时收到交接抄送 216 次。
- 已知的小毛病：
  - 完成时 Manager 发了 3 条几乎一样的完工消息；
  - T74、T76 的原始临时审查目录没有保留下来，报告里已注明；
  - 差异清单的解析器只解析了一部分（METRIC-08）。

## 怎么复现 C 组的审计重放

在原机器上运行（工具链在验证 checkout 的 `.tools/` 下）。下面这条就是插件审计时跑的那条命令：

```bash
C=/fastdata/v-boxiuli/noah/exp/tsC
T=$C/verification/.tools
export RUSTUP_HOME=$T/rustup CARGO_HOME=$T/cargo
export SYSTEM_PROOF_VERUS_HOME=$T/system-proof-agent/vertical/argus_system_proof/tools/verus-validation/toolchain/bin
export VERUS_Z3_PATH=$SYSTEM_PROOF_VERUS_HOME/z3 PATH=$T/system-proof-agent/.venv/bin:$SYSTEM_PROOF_VERUS_HOME:$CARGO_HOME/bin:$PATH
verus-run verify $C/verification --preset standard     # 期望：172 verified, 0 errors
```

最终报告里的相对链接（`../src/...`、`t31/...`、`../t80-handoff/...`）都是相对 `$C/verification/.system-proof/` 的。本包只摘了其中的关键文件，完整证据在原机器的 `$C/verification/` 下。

## 文件一览

- `base64-v4-A/`
  - `final-report.md`、`inplace.log`、`recheck-*.log`、`drift.txt`
  - `project.json`、`events.jsonl`（Argus 的完整记录）
- `base64-v4-C/`
  - `final-report.md`、`limitation-report.md`、`effective-objective.txt`
  - `audit-verification.log`（插件原地重放）、`independent-replay.log`（独立重放）
  - `proof/`：规约、证明辅助、调用方、和生产代码的完整差异
  - `trust/`：Verus 补丁、vstd 增补、改写和对应关系的论证、生命周期范围、T80 交接说明、工具链身份
  - `operator/`：操作员消息和各时间点
  - `project.json`、`events.jsonl`、`usage.jsonl`

## E 组（最新）

- Argus 最新 ts-core 加技能包 0.2.1（后半段换成 0.2.3），工作目录不手动准备，开头就允许不改行为的改写。
- 原地审计重放 **179 verified, 0 errors**，我独立重放结果相同。实际干活约 2.5 小时，花费 $30.09，33 个任务。
- Verus 只改了一处：28 行的前端补丁（`base64-v4-E/trust/verus-t7-marker-metasized.patch`）。可信基为直接 10 组、剩余 2 组。
- 结论带前提 B：调用方的 `AsRef` 和输入对象的 `Drop` 正常执行完。原始 API 的 `Drop` panic 反例保留在报告里。
- 材料在 `base64-v4-E/`：报告、规约与证明、源码差异、补丁与评估、两份重放日志、操作员消息、完整记录。
