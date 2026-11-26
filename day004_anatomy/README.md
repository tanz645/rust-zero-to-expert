# Day 4 · Anatomy of a Rust program & comments

▶ Video: [Day 4 on YouTube](https://www.youtube.com/watch?v=1xdVQSOrFA8) · 📖 Rust by Example: [Hello World](https://doc.rust-lang.org/rust-by-example/hello.html) · [Comments](https://doc.rust-lang.org/rust-by-example/hello/comment.html)

[`src/main.rs`](src/main.rs) is a short program with **three broken symbols** and **no comments yet**.
Fix it, then explain every line with comments. The tests in [`tests/anatomy.rs`](tests/anatomy.rs) tell you when you're done.

| Level | Command (run it from the repo folder) | Task |
|---|---|---|
| ● ○ ○ Warm-up | `cargo run -p day004_anatomy` | It doesn't compile: a `}`, a `;` and a `!` are missing. Read each error, fix it, run again |
| ● ● ○ Practice | `cargo run -p day004_anatomy` | Switch off the line that says it should not print. Use a comment, don't delete it |
| ● ● ○ Practice | | Give every line of code a `//` comment that says what it does: above the line, or at the end of it |
| ● ● ● Challenge | `cargo test -p day004_anatomy` | Add a `///` doc comment above `fn main` and one `/* block comment */`. All 5 tests pass |

You're done when this prints `test result: ok. 5 passed`:

```bash
cargo test -p day004_anatomy
```

**Bonus:** run `cargo doc -p day004_anatomy --open` and find your `///` comment on the web page it makes.
In VS Code, hover over `main` to see it there too.

## The pieces of a Rust program

| Piece | Name | What it does |
|---|---|---|
| `fn` | keyword | Starts a function: a named block of code |
| `main` | function name | Where every program starts. Exactly one per program |
| `()` | parameter list | The function's inputs. `main` takes none |
| `{ }` | braces | The body: the code that runs, top to bottom |
| `println!` | macro | `!` means macro: code that writes code while compiling |
| `"Hello"` | string literal | Text, written right in the code |
| `;` | semicolon | Ends a statement |

## Three kinds of comments

| Comment | Used for |
|---|---|
| `// …` | Line comment: Rust skips the rest of the line. Most comments look like this |
| `/* … */` | Block comment: can cover several lines, or part of one |
| `/// …` | Doc comment: describes the item below it. Shows up in `cargo doc` and when you hover in VS Code |

**Useful later:**
- VS Code toggles `//` on the current line with **Ctrl + /** (**Cmd + /** on a Mac).
- `//!` is a doc comment for the whole file or crate, written at the very top.
- Rust block comments can nest: `/* outer /* inner */ still a comment */`.

Solutions are published on the `solutions` branch 7 days after the video.
