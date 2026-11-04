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

1. 拉取 Verus：`tangruize/verus`，提交 `d85f7578`（"support & and | for bool (#3007)"）。
2. 打上 `verus-marker-metasized.patch`。只改一个文件：`source/rust_verify/src/rust_to_vir_func.rs`。
3. 编译 Verus。按 Verus 自己的要求，用 Rust 1.98.1、Z3 4.16.0。
4. 用审计时那条命令验证 `base64-verification/src/lib.rs`：

   ```bash
   verus base64-verification/src/lib.rs --crate-type lib --edition 2021 \
     --cfg 'feature="std"' --cfg 'feature="alloc"' --rlimit 20
   ```

   就是 `--no-default-features --features std` 这个配置，直接把 crate 根交给 Verus，没有经过 Cargo。

## 源码从哪来

`base64-production/` 是 rust-base64 在 `5b98ee18` 的源码快照，只保留了 45 个源文件，不含 git 历史。

`base64-verification/Cargo.toml` 里 vstd 的路径指向 `../verus/source/vstd`，也就是 `run.sh` 拉下来的那份 Verus。这个路径只有用 Cargo 构建时才会用到，上面那条直接验证的命令用不到它。

## 我们这边的核对

把这个目录里的 `base64-verification/` 单独拷到一个空目录，用同一个打过补丁的 Verus 跑，结果是 179 verified, 0 errors。

`run.sh` 从零拉取并编译 Verus 的整条路径，我们还没有在一台干净的机器上完整跑过。如果卡住了，请开 issue。
