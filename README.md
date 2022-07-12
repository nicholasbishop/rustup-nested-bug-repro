# rustup-nested-bug-repro

Related <https://github.com/rust-lang/rustup/issues/3031>

## Rustup 1.25.0

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
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target\debug\rustup-nested-bug-repro.exe`
RUSTUP_TOOLCHAIN: "stable-x86_64-pc-windows-msvc"
Original PATH: "C:\\Users\\nicholasbishop\\src\\rustup-nested-bug-repro\\target\\debug\\deps;C:\\Users\\nicholasbishop\\src\\rustup-nested-bug-repro\\target\\debug;C:\\Users\\nicholasbishop\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib;C:\\Users\\nicholasbishop\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\bin;C:\\Users\\nicholasbishop\\AppData\\Local\\Programs\\Python\\Python310\\;C:\\Users\\nicholasbishop\\bin;C:\\Program Files\\Git\\mingw64\\bin;C:\\Program Files\\Git\\usr\\local\\bin;C:\\Program Files\\Git\\usr\\bin;C:\\Program Files\\Git\\usr\\bin;C:\\Program Files\\Git\\mingw64\\bin;C:\\Program Files\\Git\\usr\\bin;C:\\Users\\nicholasbishop\\bin;C:\\Program Files\\Oculus\\Support\\oculus-runtime;C:\\Program Files (x86)\\Razer Chroma SDK\\bin;C:\\Program Files\\Razer Chroma SDK\\bin;C:\\Windows\\system32;C:\\Windows;C:\\Windows\\System32\\Wbem;C:\\Windows\\System32\\WindowsPowerShell\\v1.0;C:\\WINDOWS\\system32;C:\\WINDOWS;C:\\WINDOWS\\System32\\Wbem;C:\\WINDOWS\\System32\\WindowsPowerShell\\v1.0;C:\\WINDOWS\\System32\\OpenSSH;C:\\Program Files\\Git\\cmd;C:\\Users\\nicholasbishop\\.cargo\\bin;C:\\Users\\nicholasbishop\\AppData\\Local\\Microsoft\\WindowsApps;C:\\Program Files\\Git\\usr\\bin\\vendor_perl;C:\\Program Files\\Git\\usr\\bin\\core_perl"
running command: "cargo" "+nightly" "version"
error: no such subcommand: `+nightly`
thread 'main' panicked at 'assertion failed: s.success()', src\main.rs:32:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\rustup-nested-bug-repro.exe` (exit code: 101)
```

But it succeeds if the `PATH` is modified to remove toolchain entries
added by rustup:

```
$ cargo run modify
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target\debug\rustup-nested-bug-repro.exe modify`
RUSTUP_TOOLCHAIN: "stable-x86_64-pc-windows-msvc"
Original PATH: "C:\\Users\\nicholasbishop\\src\\rustup-nested-bug-repro\\target\\debug\\deps;C:\\Users\\nicholasbishop\\src\\rustup-nested-bug-repro\\target\\debug;C:\\Users\\nicholasbishop\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib;C:\\Users\\nicholasbishop\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\bin;C:\\Users\\nicholasbishop\\AppData\\Local\\Programs\\Python\\Python310\\;C:\\Users\\nicholasbishop\\bin;C:\\Program Files\\Git\\mingw64\\bin;C:\\Program Files\\Git\\usr\\local\\bin;C:\\Program Files\\Git\\usr\\bin;C:\\Program Files\\Git\\usr\\bin;C:\\Program Files\\Git\\mingw64\\bin;C:\\Program Files\\Git\\usr\\bin;C:\\Users\\nicholasbishop\\bin;C:\\Program Files\\Oculus\\Support\\oculus-runtime;C:\\Program Files (x86)\\Razer Chroma SDK\\bin;C:\\Program Files\\Razer Chroma SDK\\bin;C:\\Windows\\system32;C:\\Windows;C:\\Windows\\System32\\Wbem;C:\\Windows\\System32\\WindowsPowerShell\\v1.0;C:\\WINDOWS\\system32;C:\\WINDOWS;C:\\WINDOWS\\System32\\Wbem;C:\\WINDOWS\\System32\\WindowsPowerShell\\v1.0;C:\\WINDOWS\\System32\\OpenSSH;C:\\Program Files\\Git\\cmd;C:\\Users\\nicholasbishop\\.cargo\\bin;C:\\Users\\nicholasbishop\\AppData\\Local\\Microsoft\\WindowsApps;C:\\Program Files\\Git\\usr\\bin\\vendor_perl;C:\\Program Files\\Git\\usr\\bin\\core_perl"
Modified PATH: "C:\\Users\\nicholasbishop\\src\\rustup-nested-bug-repro\\target\\debug\\deps;C:\\Users\\nicholasbishop\\src\\rustup-nested-bug-repro\\target\\debug;C:\\Users\\nicholasbishop\\AppData\\Local\\Programs\\Python\\Python310\\;C:\\Users\\nicholasbishop\\bin;C:\\Program Files\\Git\\mingw64\\bin;C:\\Program Files\\Git\\usr\\local\\bin;C:\\Program Files\\Git\\usr\\bin;C:\\Program Files\\Git\\usr\\bin;C:\\Program Files\\Git\\mingw64\\bin;C:\\Program Files\\Git\\usr\\bin;C:\\Users\\nicholasbishop\\bin;C:\\Program Files\\Oculus\\Support\\oculus-runtime;C:\\Program Files (x86)\\Razer Chroma SDK\\bin;C:\\Program Files\\Razer Chroma SDK\\bin;C:\\Windows\\system32;C:\\Windows;C:\\Windows\\System32\\Wbem;C:\\Windows\\System32\\WindowsPowerShell\\v1.0;C:\\WINDOWS\\system32;C:\\WINDOWS;C:\\WINDOWS\\System32\\Wbem;C:\\WINDOWS\\System32\\WindowsPowerShell\\v1.0;C:\\WINDOWS\\System32\\OpenSSH;C:\\Program Files\\Git\\cmd;C:\\Users\\nicholasbishop\\.cargo\\bin;C:\\Users\\nicholasbishop\\AppData\\Local\\Microsoft\\WindowsApps;C:\\Program Files\\Git\\usr\\bin\\vendor_perl;C:\\Program Files\\Git\\usr\\bin\\core_perl"
running command: "cargo" "+nightly" "version"
cargo 1.64.0-nightly (b1dd22e66 2022-07-09)
```

## Rustup 1.24.3

TODO
