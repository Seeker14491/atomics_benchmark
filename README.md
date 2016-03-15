# atomics_benchmark
The purpose of this code, motivated by [this forum thread](https://users.rust-lang.org/t/data-races-undefined-behavior/4960), is to compare the speed of atomic vs non-atomic operations in Rust that might be used in a [Buddhabrot](https://en.wikipedia.org/wiki/Buddhabrot) renderer. It's a work in progress; it does not accomplish this yet.

## Usage
A recent enough nightly Rust is needed. Clone the repo, then inside the cloned directory
```
cargo bench
```
