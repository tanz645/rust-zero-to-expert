# Rust: Zero → Expert · Exercises

One folder per lesson day. Each has a `README.md` with the tasks and a starter crate.

| Day | Lesson | Video | Exercise |
|---|---|---|---|
| 1 | Why learn Rust? How your code actually runs | [▶ Watch](https://www.youtube.com/watch?v=wMsujLrYhe0) | [`day001_why_rust`](day001_why_rust) |
| 2 | Install Rust & write your first program | [▶ Watch](https://www.youtube.com/watch?v=1Fr9lXlpKRY) | [`day002_install_rust`](day002_install_rust) |
| 3 | The Cargo workflow: check, run, fmt, clippy & test | [▶ Watch](https://www.youtube.com/watch?v=ufgvI7LQ6L4) | [`day003_cargo`](day003_cargo) |
| 4 | Anatomy of a Rust program & comments | [▶ Watch](https://www.youtube.com/watch?v=1xdVQSOrFA8) | [`day004_anatomy`](day004_anatomy) |

```bash
cargo run -p day001_why_rust      # run a lesson's program
cargo test -p day003_cargo        # check a lesson's exercises (from Day 3 on)
```

The toolchain is pinned in `rust-toolchain.toml` so compiler messages match the videos.
From Day 3 on, starter code may contain warnings, messy formatting or failing tests **on purpose**: that's the exercise.
So run commands for one lesson at a time with `-p dayNNN_…`, not for the whole workspace.
Solutions are published on the `solutions` branch 7 days after each video.

Not affiliated with the Rust Foundation. Some examples are adapted from
[Rust by Example](https://doc.rust-lang.org/rust-by-example/) (MIT/Apache-2.0).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option. You are free to copy, change and reuse the exercises and your solutions.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in this repository, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.
