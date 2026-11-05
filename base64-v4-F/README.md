# F：同一个证明，在默认 rlimit 下通过

E 的证明是对的，但要把 Verus 的 rlimit 调到 20 才能过。瑞泽审的时候指出了这一点：证明太脆，是"报一个错修一个错"堆出来的，在大项目里会拖慢证明、改一处代码就要大面积返工。

F 从 E 的证明出发，把它重新组织了一遍，**在 Verus 默认设置下 182 项全过、0 错误**。花了 21 分钟、$3.99，没人插手。

## 改了什么

只改了一个文件：`src/engine/general_purpose/decode.rs`。完整差异见 [`F-vs-E.patch`](F-vs-E.patch)，共 267 行。

- 两个解码循环（E 里的 `decode.rs:284` 和 `:390`）每一步要用的事实，拆成了 3 个单独的引理：
  - `canonical_quads_bounds`
  - `prepare_decode_32`
  - `prepare_decode_4`

  多出来的 3 项就是这 3 个引理，所以从 179 变成了 182。
- 有一个定义改成了不透明的，用到它的地方再显式展开。
- 定理没变，生产代码没动。

## 效果

默认 rlimit 10 对应的单个查询上限是 3000 万单位：

| 循环 | E | F |
|---|---|---|
| 32 字节循环 | 超过 3000 万，被中止 | 1096 万 |
| 4 字节循环 | 超过 3000 万，被中止 | 644 万 |

3 个新引理加起来只用了约 90 万单位，算不上把开销藏到了别处。数据来自 [`final-report.md`](final-report.md) 的 METRIC-02、03。

## 要注意的两点

1. **验证代码比 E 多了一处改写。** 团队按要求做了一个"维护者可能会做的小改动"：把解码里的一处下标计算换了一种写法，行为不变。这处改动也留在了最终验证的代码里。所以 F 验证的代码相对生产代码，比 E 多了这一处等价改写。论证见 [`evidence/t3/preservation-check.txt`](evidence/t3/preservation-check.txt) 和 [`evidence/t4/correspondence.txt`](evidence/t4/correspondence.txt)。

2. **维护性测试没测到点子上。** 选的这处改动，在 E 和 F 的证明上都不用改一行证明，而且碰巧让 E 原本超时的两个循环也过了。所以它说明不了"重组后改代码返工更少"，团队在报告里也照实写了。要真测维护性，得选一处会碰到循环不变式的改动。

## 复现

```bash
cd base64-v4-F/reproduce
./run.sh   # 最后一行应该是：verification results:: 182 verified, 0 errors
```

这条命令会先调用 E 的 `run.sh` 把打过补丁的 Verus 编出来，然后不带 `--rlimit` 验证 F 的完整源码。

我把这份源码单独拷到一个干净目录里跑过，结果是 182/0，日志在 [`independent-replay.log`](independent-replay.log)。这个 `run.sh` 本身没有从零跑过一遍，但它用的 Verus 构建步骤和 E 的相同，E 那条路径已经从零跑通过。

## 目录

```
final-report.md     最终报告（Reviewer 写的）
F-vs-E.patch        F 相对 E 的全部改动
reproduce/          F 的完整验证版源码和复现脚本
evidence/           报告引用的证据：t1 准备，t2 重组证明，t3 维护性测试，t4 审计
events.jsonl        全过程记录：每封信、每条命令、每一笔花费
request.txt         给团队的原始要求
```
