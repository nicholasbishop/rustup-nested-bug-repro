# rustup-nested-bug-repro

Related <https://github.com/rust-lang/rustup/issues/3031>

On Linux this command succeeds:

```
$ cargo run
   Compiling rustup-nested-bug-repro v0.1.0 (/var/home/nbishop/src/rustup-nested-bug-repro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/rustup-nested-bug-repro`
running command: "cargo" "+nightly" "version"
cargo 1.64.0-nightly (b1dd22e66 2022-07-09)
```

On Windows it fails:

```
TODO
```

Rustup version:
```
$ rustup --version
rustup 1.25.0 (90365aa81 2022-06-15)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.62.0 (a8314ef7d 2022-06-27)`
```
