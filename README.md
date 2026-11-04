# 让 AI 团队证明 base64 不丢字节

我们让 [Argus](https://github.com/lbx154/Argus) 组织的一组 AI 角色（Manager、Planner、Engineer、Reviewer）用 Verus 去证明一件看起来很简单的事：

> 用 base64 crate 把任意字节编码成文本，再解码回来，拿到的字节和原来一模一样。

这件事在 Verus 里并不简单。同一道题我们前后跑了好几次，这里收了三次有结果的：**A**、**C**、**E**。E 是最新的一次，也是最好的。

想直接看全部材料，打开网页版：**https://lbx154.github.io/system-proof-base64-v4-results/** ，每个文件都能点开看全文。

---

## 先说结论

| | A | C | **E** |
|---|:-:|:-:|:-:|
| 在真实代码上原地重放 | ❌ 只过了 1 项 | ✅ 172 项全过 | ✅ **179 项全过** |
| 对 Verus 本身的改动 | 3 个补丁 | 2 个编译器补丁 | **1 个 28 行的小补丁** |
| 花了多少 | $19 · 3 小时 | $104 · 6 小时 | **$30 · 2.5 小时** |
| 人插手了几次 | 0 | 3 | 1 |

- **E 是现在可以拿出来的结果**：证明直接做在 base64 的真实源码上，没有 `assume`，也没有 `admit`。生产代码一个字节没动。我在另一个干净目录里自己重跑了一遍，179 项全过。
- **C 也证出来了**：但它改了 Verus 编译器两处，那两处到现在还没人审过。而且它的大半时间都耗在了 Verus 编译器内部。
- **A 不算数**：它的证明放在一份抽出来的副本里，真实代码上重放只过了 1 项。

## E 证明了什么

用一句话说：**只要你的输入长度在合理范围内、缓冲区按库自己给的大小分配，那么：**

1. `encode_slice` 一定成功，写出的就是标准 base64 编码；
2. 只把它写出的那段交给 `decode_slice`，也一定成功，拿回的正是原来的字节，顺序和个数都对；
3. 输入不会被改动，两个缓冲区里写入范围之外的字节也不会被碰；
4. 过程中不会报错，也不会 panic。

“合理范围”是 n ≤ 3⌊isize::MAX/4⌋，这是从 Rust 切片的长度上限和编码后的长度推出来的。空输入、全零、非 UTF-8 的字节、块边界前后的各种长度，全都包括在内。

**有一个前提**：你自己的代码要正常跑完，也就是你传进来的对象的 `AsRef` 转换和 `Drop` 析构不能中途 panic。团队在审计时发现了一个真实的反例：base64 已经把字节写对了，但你的对象在析构时 panic，调用就失败了。这不是 base64 的 bug，但它说明“无条件一定成功”这句话不成立。所以结论带上了这个前提，反例也写进了报告。

**不包括**：其他字母表、外部传进来的乱码、缓冲区给小了、SIMD、流式接口。

## 请重点看这三处

只有这三处是“靠信任、没被机器证明”的：

1. **那个 28 行的 Verus 补丁**：[`base64-v4-E/trust/verus-t7-marker-metasized.patch`](base64-v4-E/trust/verus-t7-marker-metasized.patch)
   Verus 在翻译 base64 的 trait 时会卡住。这个补丁去掉了一个 Verus 自己合成出来的多余约束，真实的约束都保留着。第一版补丁的范围写错了，团队自己发现后重做了一版。它对不对，需要懂 Verus 的人看一眼。评估记录在 [`ASSESSMENT-t7.md`](base64-v4-E/trust/ASSESSMENT-t7.md)。

2. **为了让 Verus 能处理，验证用的代码做了一些改写**：[完整差异](base64-v4-E/proof/verification-vs-production.patch)
   比如：
   - 把几个 helper 调用就地展开；
   - 把迭代器换成等价的下标循环；
   - 把大端字节打包写成显式的形式。

   这些改写不改变程序行为，但“不改变”是人工论证的，不是机器证明的。有一处特别值得看：原版的 `encode_helper` 没有被证明（SIMD 后端还在用它），证明走的是展开后的那份。

3. **前面说的那个前提**，以及当时给出的三个选项：[`base64-v4-E/operator/`](base64-v4-E/operator/)

C 的两个编译器补丁也在仓库里（[`base64-v4-C/trust/`](base64-v4-C/trust/)）。如果有时间，T29 那个最值得看，它改的是 Rust 函数翻译给 Verus 的那一层。

## 三次尝试是怎么来的

**A** 是第一次。它其实没发现自己的问题：当时的检查只看副本，副本在它打过补丁的 Verus 上全过了。我们后来把“必须在真实代码上原地重放”这项检查补回去，它就不过了。

**C** 是第二次。它在一道坎上卡了 5 个多小时：Verus 处理不了 base64 里 trait 默认方法的一个调用循环。团队一直想在不改源码的前提下硬攻 Verus 编译器，攻了十几个任务都没过去。后来我告诉它“不改行为的改写可以接受”，8 分钟就过了这一关，剩下两个阶段 34 分钟做完。中途它还一度想自己宣布失败，也是被叫回来的。

**E** 是第三次。我们根据 C 的教训改了 Argus 和技能包：
- 能并行的工作同时开；
- 一开始就允许改写；
- 团队之间边做边写信，Reviewer 可以中途先看一眼；
- 工具由 Argus 自动装好，不再手动准备。

E 只用 59 分钟就过了 C 卡了 5 小时的那一关。

它中间停过一次：碰到上面那个 `Drop` 反例时，它停下来问我们选哪条路，等了一夜。其实这种事它该自己定，后来我们也把规则改成了这样。

## 仓库里有什么

```
index.html        网页版，所有材料全文
base64-v4-E/      E 组（最新）
  final-report.md   最终报告，Reviewer 写的
  proof/            规约、调用方、证明，以及和原代码的完整差异
  trust/            Verus 补丁和它的评估、改写的论证、前提的说明
  operator/         当时给我们的选项和我们的回复
  audit-verification.log / independent-replay.log   团队的重放和我自己的重放
  events.jsonl      全过程记录：每封信、每条命令、每一笔花费
base64-v4-C/      C 组，结构差不多
base64-v4-A/      A 组
```

## 自己重放一遍

在我们的机器上，下面这几行就能复现 E 的 179/0：

```bash
E=/fastdata/v-boxiuli/noah/exp/tsE
H=$E/home/pack-homes/system_proof/0.2.1
export RUSTUP_HOME=$H/rustup CARGO_HOME=$H/cargo
export SYSTEM_PROOF_VERUS_HOME=$H/system-proof-agent/vertical/argus_system_proof/tools/verus-validation/toolchain/bin
export VERUS_Z3_PATH=$SYSTEM_PROOF_VERUS_HOME/z3
export PATH=$H/system-proof-agent/.venv/bin:$SYSTEM_PROOF_VERUS_HOME:$CARGO_HOME/bin:$PATH
verus-run verify $E/task/verification --package base64
```

C 的复现命令在 [网页版](https://lbx154.github.io/system-proof-base64-v4-results/#repro) 里。

---

有问题直接在这个仓库开 issue。
