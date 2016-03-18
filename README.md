# atomics_benchmark
The purpose of this code, motivated by [this forum thread](https://users.rust-lang.org/t/data-races-undefined-behavior/4960), is to compare the speed of atomic vs non-atomic operations in Rust that might be used in a [Buddhabrot](https://en.wikipedia.org/wiki/Buddhabrot) renderer.

## Usage
A recent enough nightly Rust is needed. Clone the repo, then inside the cloned directory run one of the following commands. There are two sets of benchmarks.

* The first uses Rust's own benchmarking infrastructure, as is run with
```
cargo bench
```

* The second set manually times the code and prints the result. Each part of this benchmark is run with
```
cargo run --release --example atomic
```
and
```
cargo run --release --example no_sharing
```
