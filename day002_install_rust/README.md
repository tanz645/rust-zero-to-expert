# Day 2 · Install Rust & run your first program

▶ Video: _link added at publish_ · 📖 Rust by Example: [Hello World](https://doc.rust-lang.org/rust-by-example/hello.html) · [Playground](https://doc.rust-lang.org/rust-by-example/meta/playground.html)

## 1. Install Rust

Everything starts at <https://rustup.rs>.

| System | What to do |
|---|---|
| **macOS / Linux** | Open a terminal and paste: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh`. Choose **1) Proceed with standard installation** (press Enter). |
| **Windows** | Download and run `rustup-init.exe` from <https://rustup.rs>. When it offers to install the Visual Studio C++ build tools, **say yes**. Then choose option 1. |

When you see `Rust is installed now. Great!`, **close the terminal and open a new one**.

## 2. Check it worked

```bash
rustc --version
cargo --version
```

Any recent version is fine. Inside this repo, `rust-toolchain.toml` pins the exact version used in the videos, so compiler messages match what you see on screen.

## 3. Editor

Install [VS Code](https://code.visualstudio.com), open the Extensions panel, search for **rust-analyzer** and install the one published by **The Rust Programming Language**.

## Your turn

| Level | Task |
|---|---|
| ● ○ ○ Warm-up | Install Rust. Check `rustc --version` and `cargo --version` |
| ● ● ○ Practice | `cargo new hello_rust`, open it in VS Code, print 3 lines: your name, where you're from, why you're learning Rust |
| ● ● ● Challenge | Write `hello.rs`, compile it with `rustc hello.rs`, run it. Then run the program cargo built in `target/debug/` directly |

You can also practise in this folder: `cargo run -p day002_install_rust`.

## Troubleshooting

| You see | Fix |
|---|---|
| `command not found: cargo` (or `'cargo' is not recognized` on Windows) | Close the terminal and open a new one. On macOS/Linux you can also run `. "$HOME/.cargo/env"` |
| ``error: linker `cc` not found`` (Linux) | Install a C toolchain: `sudo apt install build-essential` (Debian/Ubuntu) or `sudo dnf install gcc` (Fedora) |
| ``error: linker `link.exe` not found`` (Windows) | The C++ build tools are missing. Run `rustup-init.exe` again and say yes to them |
| A pop-up asks for "command line developer tools" (macOS) | Click **Install**. Rust uses Apple's linker |

Still stuck? Post the **exact** error under the video.

## Useful later

- `rustup update` updates Rust.
- `rustup doc` opens the docs offline, including The Book and Rust by Example.

Solutions are published on the `solutions` branch 7 days after the video.
