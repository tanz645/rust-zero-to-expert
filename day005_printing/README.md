# Day 5 · Printing & placeholders

▶ Video: [Day 5 on YouTube](https://www.youtube.com/watch?v=PO5e7TG11Ks) · 📖 Rust by Example: [Formatted print](https://doc.rust-lang.org/rust-by-example/hello/print.html)

[`src/main.rs`](src/main.rs) prints one line about Ferris. Print that **same line four ways**, send a note to standard error, then print a line with **real curly braces** in it.
The tests in [`tests/printing.rs`](tests/printing.rs) tell you when you're done.

| Level | Command (run it from the repo folder) | Task |
|---|---|---|
| ● ○ ○ Warm-up | `cargo run -p day005_printing` | It doesn't compile: the first line has 3 slots `{}` but only 2 values. Read the error and add the missing value |
| ● ● ○ Practice | `cargo run -p day005_printing` | Print the same line again with **named** slots (`{who}` … `who = name`), then with **inline** slots (`{name}`, nothing after the text) |
| ● ● ○ Practice | | Build the same line with `format!` into `card`, then print `card`. Send `Note: 1 line, 4 ways.` to standard error with `eprintln!` |
| ● ● ● Challenge | `cargo test -p day005_printing` | Print `{ name: Ferris, day: 5 }`, braces and all. All 6 tests pass |

You're done when this prints `test result: ok. 6 passed`:

```bash
cargo test -p day005_printing
```

When it's all done, `cargo run -p day005_printing` prints the line four times, the note, then the braces line.

**Bonus:** run `cargo run -p day005_printing > out.txt`. The note still shows on screen, but the five lines go into `out.txt`.

## Three ways to fill a slot

| Slot | Name | The values go… | Example |
|---|---|---|---|
| `{}` | positional | after the text, in order | `println!("{} is {}", name, language);` |
| `{0}` `{1}` | numbered | after the text; the number picks one (from 0) | `println!("{0} + {1} = {1} + {0}", name, language);` |
| `{who}` | named | after the text, as `who = …` | `println!("{who} is {what}", who = name, what = language);` |
| `{name}` | inline | nowhere: the slot uses the variable `name` | `println!("{name} is {language}");` |

- An inline slot holds a **name only**: `{259 - day}` doesn't compile. A calculation goes **after the text**, with an empty slot or a named one:
  `println!("Days left: {}", 259 - day);` or `println!("Days left: {left}", left = 259 - day);` Both print `Days left: 254`.
- To print a real brace, double it: `{{` prints `{` and `}}` prints `}`.
- `let name = "Ferris";` gives a value a name. Variables get their own lesson on Day 9.

## The print family

| Macro | Where the text goes | New line? |
|---|---|---|
| `println!` | standard output: your results | yes |
| `print!` | standard output | no |
| `eprintln!` | standard error: errors and notes | yes |
| `eprint!` | standard error | no |
| `format!` | nowhere: you get a `String` back | – |

**Useful later:**
- `cargo run 2> notes.txt` sends standard error to a file instead.
- Width, alignment and decimals, like `{:>8}` and `{:.2}`, are Day 6.
- `{:?}` prints a value the way a programmer wants to see it. That's Day 7.

Solutions are published on the `solutions` branch 7 days after the video.
