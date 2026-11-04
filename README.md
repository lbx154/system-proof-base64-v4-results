# base64-v4 审阅包：Argus A、C、E 三组的 Verus 证明成果

本仓库收录三组实验的完整成果，请评审。三组都用 [Argus](https://github.com/lbx154/Argus)（TypeScript 版）加 `system_proof` 技能包，后端是 codex，模型是 gpt-6.1-sol，用 Verus 证明 base64 crate 的编解码往返。技能包不在本仓库，这里只放成果。

**网页版**：所有文件都能在线展开看全文，入口是 https://lbx154.github.io/system-proof-base64-v4-results/

## 任务

题目是 system-proof-benchmarks 的 `base64-v4`。原文在各组的 `request.txt` / `effective-objective.txt`。

- **对象**：base64 crate，`GeneralPurpose` 引擎配 `general_purpose::STANDARD`，构建参数 `--no-default-features --features std`，不用 SIMD。
- **入口**：`Engine::encode_slice`、`Engine::decode_slice`，以及给缓冲区定大小的 `encoded_len`、`decoded_len_estimate`。
- **要证的性质**：
  - 对支持范围内的任意字节，编码再解码都成功，并原样拿回；
  - 解码时只传编码报告的那段前缀；
  - 输入不被改动；
  - 两个目标缓冲区里报告长度之后的字节不变；
  - 支持范围内不允许出现编解码错误或 panic。
- **生产代码**：每组都是从同一份源码新生成一个提交，所以提交哈希不同（A `5d574f8`、C `47fa113`、E `d9ce0de`），但 `src` 逐字节相同，我 diff 过。三组都没有改生产代码。

## 结论

| | A 组 | C 组 | **E 组（最新）** |
|---|---|---|---|
| **插件的原地审计重放** | **不通过**：原地只验证了 1 项 | **通过**：172 verified, 0 errors | **通过**：179 verified, 0 errors |
| 我在另一个 worktree 上独立重放 | — | 172/0 | 179/0 |
| 证明在哪份代码上 | 抽出来的副本（handoff 目录里的证明文件） | 验证 checkout 里的真实源码 | 验证 checkout 里的真实源码 |
| 对 Verus 的改动 | 3 个补丁，另往 vstd 加了规约；原版 Verus 编译不过 | 2 个编译器补丁（T3 +34/−20，T29 +73/−22）+ 一条 vstd axiom | **1 个前端补丁（28 行）** |
| 可信基（TCB，只信不证的部分） | 直接 20 组 + 剩余 6 组 | 直接 14 组 + 剩余 1 组 | **直接 10 组 + 剩余 2 组** |
| `assume` / `admit` | — | 0 / 0 | 0 / 0 |
| 三个阶段（ground → prove → audit） | 停在 audit，最终报告没装好 | 全过 | 全过（audit 曾退回 prove 一次） |
| 实际干活时间 | 3h06 | 5h55 | 约 2.5h（另有一夜在等操作员） |
| 花费 / 任务数 | $18.69 / 19 | $104.12 / 82 | **$30.09 / 33** |
| 同时开着的任务，最多 | 1 | 13 | 6 |
| 人工介入 | 0 次 | 3 次 | 1 次 |

**一句话评价**：
- **A 组**：结果过不了插件自己的原地审计，不算交付。
- **C 组**：交付了完整的功能证明，但依赖两处没人审过的 Verus 编译器补丁，而且大部分时间花在了攻编译器内部。
- **E 组**：结果最干净。对 Verus 的改动最小，可信基最小，花费约为 C 组的三成。

“原地审计重放”指插件审计时跑的那条命令，直接在验证 checkout 上跑 Verus，不是在副本上。C 组用的是 `verus-run verify <verification> --preset standard`，E 组用的是 `verus-run verify <verification> --package base64`。

## 请重点审（按风险从高到低）

1. **E 组的 Verus 前端补丁**：`base64-v4-E/trust/verus-t7-marker-metasized.patch`，28 行，改的是 trait 的翻译。它只去掉合成 marker 关联类型上的 `MetaSized` 约束，真实的外部 `Self` 和泛型前提都保留。第一版补丁的作用范围有缺陷，是团队自己发现并重做的，两版都留了档。独立评估见 `trust/ASSESSMENT-t7.md`。可信基里对应 TCB-03。
2. **E 组的改写与生产代码怎么对应（TCB-09）**：验证源码对生产源码 +1640/−185 行，完整差异在 `base64-v4-E/proof/verification-vs-production.patch`。大部分是证明注解，但也有不改行为的改写：
   - 把 helper 和 padding 调用在原处就地展开；
   - 把标量的回调特化；
   - 迭代器换成等价的下标循环；
   - 大端字节打包改为显式写出；
   - 等值的字面量和配置换了位置。

   要特别注意一处：原来的 `encode_helper` 被标成 `external_body`。它在源码里仍被 SIMD 后端调用，但不在这次证明的 scalar 路径上；scalar 路径用的是展开后的那份。展开后和原来是否等价，靠人工论证，不是机器证明。论证见 `trust/production-correspondence-t21.diff` 和最终报告的 TCB-09。
3. **E 组的前提 B（TCB-10/11）**：调用方的 `AsRef` 转换和输入对象的完整 `Drop` 要正常执行完，被观察的输入、缓冲区和容量保持不变。审计发现，原始 API 对自带对象的输入可能在写对字节之后、返回之前，因为析构 panic 而失败。这条无条件保证被推翻了，反例保留在报告里。操作员选了 B，原文和当时的选项在 `base64-v4-E/operator/`。
4. **C 组的两个编译器补丁**：
   - `base64-v4-C/trust/verus-T3-compiler.patch`：把常量初始化的求值推迟到 THIR 擦除初始化之后，仍然检查所有未用到的常量。
   - `base64-v4-C/trust/verus-T29-compiler-combined.patch`：改了 `rust_to_vir_func.rs`，修 closed-array 签名的匹配。这个改的是 Rust 函数翻译成 VIR 的部分，最可能影响可靠性。
5. **C 组的 vstd axiom**：`base64-v4-C/trust/vstd-T29-delta.patch`。内容是 `<[T; N]>::try_from(&[T])` 成功当且仅当 `len == N`，成功时内容相同。我核对过，和 Rust 标准库的行为一致。
6. **A 组为什么不算**：A 的证明代码放在各个 handoff 目录里，靠一份抽出来的副本引进来（比如 `src/../t13-handoff/chunk-lemmas.rs`），验证 checkout 的 `src` 里只有规约和调用方。所以原地审计只验证了 1 项（`inplace.log`）。副本在 A 打过补丁的 Verus 上是 146/0（`recheck-patched.log`），换原版 Verus 编译不过（`recheck-stock.log`）。

## E 组证明了什么

最终报告：`base64-v4-E/final-report.md`，由 Reviewer 撰写。

- **规约**（`proof/round_trip_spec.rs`）：按 STANDARD 定义规范编码。设 n 为输入长度，E = 4⌈n/3⌉，D = 3⌈E/4⌉。
- **支持范围**：n ≤ 3⌊isize::MAX/4⌋，由字节切片的长度上限和补齐后的输出容量推出，不是“实现接受什么就算什么”。两个目标缓冲区的长度分别至少为 E 和 D，原有内容任意。空输入、全零、非 UTF-8 字节、以及块边界两侧的每种余数，都在范围内。
- **结论**：在前提 B 下，
  - 编码返回 `Ok(E)`，写入的前缀就是规范编码；
  - 只用这段前缀解码，返回 `Ok(n)`，内容就是原字节，顺序和个数都对；
  - 在这些条件下合约里 `Err` 不可能出现，内部 helper 出错和编解码 panic 都不算允许的结果；
  - 输入在存活期间不变，两个缓冲区报告长度之后的字节不变。
- **调用方**（`proof/round_trip_use.rs`）：按公开的长度 helper 给缓冲区定大小，然后先编码、再只取报告的前缀去解码。另有一个调用方覆盖自带对象的编码和借用前缀的解码。两个调用方都只为验证而写，不是生产里真实存在的调用方。
- **只信不证**：
  - 源码里 `assume` 0 处、`admit` 0 处；
  - `external_body` 9 处：
    - 5 处是这条往返路径用不到的 String/Vec 便利接口（`encode`、`encode_string`、`decode`、`decode_vec`、`decode_slice_unchecked`），另有它们内部的 3 个 `inner`；
    - `encode_helper`：原来那份 helper，见上面第 2 点；
    - `byte_slice_valid`：切片长度不超过 isize::MAX；
    - `complete_owned_cleanup`：前提 B 里的完整析构；
  - 1 处 `assume_specification`，给 `core::panicking::assert_failed`。

  可信基完整清单见报告的 “Complete TCB ledger”。
- **不覆盖**：其他字母表和配置、外部来的畸形字符串、缓冲区不够、长度超出可表示范围、分配器内部、流式接口、SIMD、时序，以及完整的标准兼容性。

## C 组证明了什么

最终报告：`base64-v4-C/final-report.md`，由 Reviewer 撰写。

- **规约**（`proof/round_trip_spec.rs`）：按 RFC 4648 STANDARD 逐字节定义编码，包括字母表、每 3 字节变 4 个字符、用 `=` 补齐。
- **结论**：证明了真实 `encode_slice` 的输出逐字节等于这个编码；只用返回的前缀去 `decode_slice` 能拿回原字节；两个缓冲区报告长度之后的字节不变；支持范围内没有错误也没有 panic。
- **调用方**（`proof/caller-t31.rs`）：写法照抄库自带的文档测试。
- **前提**：用户的 `AsRef`/`Drop` 要正常返回（TCB-13），只在 x86_64 上重放过。
- **改写**：11 处行为等价改写（C1–C11）记录在 `trust/ADAPTATIONS-T80.md`，其中包括把形成 trait 字典循环的调用就地展开。这条路线是操作员批准的（`operator/route.txt`）。
- **佐证**：RFC 4648 第 10 节的 7 个测试向量，加 771 个往返运行检查，全部通过。

## 目录

```
index.html                     网页版（所有材料全文）
base64-v4-E/                   E 组
  final-report.md              最终报告
  effective-objective.txt      任务原文
  audit-verification.log       插件原地审计重放：179/0
  independent-replay.log       我的独立重放：179/0
  proof/                       规约、调用方、证明辅助、源码对生产的完整差异
  trust/                       Verus 前端补丁及其评估、生产对应差异、代码绑定、生命周期说明
  operator/                    当时给操作员的 A/B/C 选项，以及操作员的回复
  project.json events.jsonl usage.jsonl   Argus 的完整记录（每封信、每条命令、每次调用的花费）
base64-v4-C/                   C 组，结构同上；另有 limitation-report.md，operator/ 里是 3 条操作员消息
base64-v4-A/                   A 组：final-report.md、inplace.log、recheck-*.log、drift.txt、proof/、trust/
```

## 复现审计重放

需要在原机器上运行，工具链和改过的 Verus 都在原机器上。

E 组：工具由 Argus 按技能包的准备脚本自动装好，装在 `pack-homes/` 下面。

```bash
E=/fastdata/v-boxiuli/noah/exp/tsE
H=$E/home/pack-homes/system_proof/0.2.1
export RUSTUP_HOME=$H/rustup CARGO_HOME=$H/cargo
export SYSTEM_PROOF_VERUS_HOME=$H/system-proof-agent/vertical/argus_system_proof/tools/verus-validation/toolchain/bin
export VERUS_Z3_PATH=$SYSTEM_PROOF_VERUS_HOME/z3
export PATH=$H/system-proof-agent/.venv/bin:$SYSTEM_PROOF_VERUS_HOME:$CARGO_HOME/bin:$PATH
verus-run verify $E/task/verification --package base64     # 期望：179 verified, 0 errors
```

C 组：

```bash
C=/fastdata/v-boxiuli/noah/exp/tsC
T=$C/verification/.tools
export RUSTUP_HOME=$T/rustup CARGO_HOME=$T/cargo
export SYSTEM_PROOF_VERUS_HOME=$T/system-proof-agent/vertical/argus_system_proof/tools/verus-validation/toolchain/bin
export VERUS_Z3_PATH=$SYSTEM_PROOF_VERUS_HOME/z3
export PATH=$T/system-proof-agent/.venv/bin:$SYSTEM_PROOF_VERUS_HOME:$CARGO_HOME/bin:$PATH
verus-run verify $C/verification --preset standard     # 期望：172 verified, 0 errors
```

报告里的相对链接，比如 `../src/...`、`t31/...`，都是相对原机器上 `verification/.system-proof/` 写的。关键文件已收进本仓库，完整证据在原机器上。

## 过程，以及对比不公平的地方

- **三组跑的版本不同**：
  - A 组用的是早期技能包，那时还没把“原地审计重放”这项检查补回去，所以 A 自己没发现结果过不了。
  - C 组跑的过程中，Argus 换了 5 次版本，逐步加上了四个角色边做边协作和持久 Reviewer。
  - E 组一开始就用最新的 Argus 和技能包 0.2.1。
- **C 组有人推**：C 组一度要自己宣布失败，是操作员发消息让它继续的。之后操作员又要求它多开并行，并批准了改写路线。批准前，它在 ground 阶段花了 5 小时 21 分钟，大部分耗在 Verus 编译器内部；批准后 8 分钟就过了 ground，prove 用了 16 分钟，audit 用了 18 分钟。
- **E 组的经过**：
  - 技能包 0.2.1 让 Planner 默认把不用等的工作同时开成任务，ground 阶段 59 分钟、$10.6 就过了。
  - 到 audit 时碰上 `Drop` 反例。按当时的规则，Manager 停下来请操作员在 A/B/C 里选，空等了一夜。
  - 操作员选 B，并指示以后这类选择由团队自己做。此后 Argus 和技能包都换成了新规则（Argus #436、技能包 0.2.3），E 组后半段用的就是新规则。14 分钟后重新进入 audit，26 分钟后完成。
  - 审计重放第一次有 1 项超出 Verus 的资源上限，团队 1 分钟内自己修好。
- **另有一组 D**：用旧版技能包从头跑，Planner 一次只开 1 个任务，一路给 Verus 打补丁。跑了 55 分钟、花了 $6.98 后被停掉，没有结果，未收进本仓库。
- **已知的小问题**：
  - C、E 两组完成时，Manager 都把几乎一样的完工消息发了 2–3 遍；
  - C 组的 T74、T76 原始临时审查目录没有保留下来，报告里已注明；
  - C 组的差异清单解析器只解析了一部分（METRIC-08）。

## 术语

- **ground / prove / audit**：插件的三个阶段。
  - ground：把要证的规约和可行的路线定下来，并对规约做攻击测试。
  - prove：在真实代码上把证明做出来。
  - audit：独立复核，并在原地重放。
- **TCB（可信基）**：结论依赖、但没有被证明的东西，比如 Verus 本身、库的模型、人工论证的对应关系、前提条件等。“直接”项会影响结论；“剩余”项不在这次证明的路径上。
- **验证 checkout / 生产 checkout**：同一份源码的两个 git worktree。证明工作只在验证 checkout 里做；生产 checkout 只读，用来对照原始行为。
