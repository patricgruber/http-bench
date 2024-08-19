# http-bench

Very simple benchmark for HTTP servers. You define the URL, the amount of requests and the amount of threads.

Written in Rust using clap, threadpool and reqwest.

## Compilation
- Execute `cargo build --release`
- Use the executable `./target/release/http-bench`
