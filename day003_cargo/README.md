# Day 3 · The Cargo workflow

▶ Video: _link added at publish_ · 📖 Rust by Example: [Cargo](https://doc.rust-lang.org/rust-by-example/cargo.html) · [Tests](https://doc.rust-lang.org/rust-by-example/cargo/test.html)

[`src/main.rs`](src/main.rs) has **four problems on purpose**. You don't need to find them yourself: cargo will.

| Level | Command (run it from the repo folder) | Task |
|---|---|---|
| ● ○ ○ Warm-up | `cargo run -p day003_cargo` | It runs, but there's a warning. Read it and fix it |
| ● ● ○ Practice | `cargo fmt -p day003_cargo` | Tidy the messy code with one command, then look at what changed |
| ● ● ○ Practice | `cargo clippy -p day003_cargo` | Fix what clippy suggests |
| ● ● ● Challenge | `cargo test -p day003_cargo` | One test fails. Read `left` and `right`, then fix the code (not the test) |

You're done when all four commands finish with no warning and no failure:

```bash
cargo fmt -p day003_cargo --check && cargo clippy -p day003_cargo && cargo test -p day003_cargo
```

**Bonus:** build it in release mode with `cargo build --release -p day003_cargo` and find the program in `target/release/`.

## The five commands

| Command | What it does |
|---|---|
| `cargo check` | Checks your code without making a program. The fastest feedback |
| `cargo run` | Builds the program, then runs it |
| `cargo fmt` | Formats your code the standard Rust way |
| `cargo clippy` | Lints: points out code that works but could be simpler or more idiomatic |
| `cargo test` | Runs the tests |

Solutions are published on the `solutions` branch 7 days after the video.
