# 在自己的机器上复现 E 的 179/0

这个目录是自包含的，不依赖我们的机器。

| 文件 | 是什么 |
|---|---|
| `base64-verification/` | **完整的** base64 crate，带上了全部证明。审计时检查的就是这一份。 |
| `base64-production/` | 同一份源码的原版，一个字节没改。用 `diff -r base64-production base64-verification` 就能看到所有改动。 |
| `verus-marker-metasized.patch` | 给 Verus 打的唯一一个补丁，28 行。 |
| `run.sh` | 一键复现：拉 Verus、打补丁、编译、跑验证。 |

## 怎么跑

需要 git、curl、unzip 和 [rustup](https://rustup.rs)。Linux x86_64 或 macOS 都可以。

```bash
git clone https://github.com/lbx154/system-proof-base64-v4-results
cd system-proof-base64-v4-results/base64-v4-E/reproduce
./run.sh
```

第一次大约 15–30 分钟，大部分时间花在编译 Verus 上。最后一行应该是：

```
verification results:: 179 verified, 0 errors
```

完整输出存在 `verify.log`。

## run.sh 做了什么

1. 拉取 Verus：`tangruize/verus` 的提交 `d85f7578`（"support & and | for bool (#3007)"）。
2. **先编译原版 Verus**，vstd 一起编好。按 Verus 自己的要求，用 Rust 1.98.1、Z3 4.16.0。
3. 打上 `verus-marker-metasized.patch`，**只重新编译 `rust_verify`**。这个补丁只改一个文件：`source/rust_verify/src/rust_to_vir_func.rs`。
4. 用审计时那条命令验证 `base64-verification/src/lib.rs`：

   ```bash
   verus base64-verification/src/lib.rs --crate-type lib --edition 2021 \
     --cfg 'feature="std"' --cfg 'feature="alloc"' --rlimit 20
   ```

   对应的配置是 `--no-default-features --features std`。这条命令把 crate 根直接交给 Verus，不经过 Cargo。

## 一个要注意的地方：打了补丁的 Verus 编不出自己的 vstd

第 2、3 步的顺序不能换。如果先打补丁再完整编译，Verus 本体能编过，但 vstd 会报 52 个 `external_trait_specification trait bound mismatch` 错误。

原因是补丁让 Verus 在检查 trait 规约时去掉了一个合成出来的 `MetaSized` 约束。可是 vstd 里的规约是按原来的规则写的，在新规则下就对不上了。

团队当时就是先编原版、再只重编 `rust_verify`。所以 179/0 实际是"打过补丁的编译器 + 原版编译器编出的 vstd"这个组合跑出来的。团队的报告锁定了这份 vstd 的指纹，但没有提到它在补丁下编不出来。

我们的理解是：这个检查只管 Verus 前端接不接收一个 trait 规约，不改变证明的含义。但这只是我们的判断。这个补丁算不算一个可以接受的 Verus 修改，需要懂 Verus 的人来判断。

## 源码从哪来

`base64-production/` 是 rust-base64 在 `5b98ee18` 的源码快照，只保留了 45 个源文件，不含 git 历史。

`base64-verification/Cargo.toml` 里 vstd 的路径指向 `../verus/source/vstd`，也就是 `run.sh` 拉下来的那份 Verus。这个路径只有用 Cargo 构建时才会用到，上面那条直接验证的命令用不到它。

## 我们这边的核对

我们在一个全新的环境里从零跑了 `run.sh`：全新的 Rust 安装，重新从 GitHub 拉 Verus 并编译。结果是 **179 verified, 0 errors**。

编出来的 vstd 文件和团队那份字节上不完全一样，但源码和编译步骤是同一套。

如果在你的机器上跑不通，请开 issue。
