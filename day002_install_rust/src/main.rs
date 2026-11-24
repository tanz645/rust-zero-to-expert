// Day 2 · Install Rust & run your first program · Your turn
//
// Run this file from the repo folder:
//     cargo run -p day002_install_rust
// (The first run inside this repo may download the course's pinned Rust
//  version from rust-toolchain.toml. That's normal and happens once.)

fn main() {
    // WARM-UP (1 pip): install Rust, then in a NEW terminal run
    //     rustc --version
    //     cargo --version
    // Both should print a version number.

    // PRACTICE (2 pips): in your own folder run `cargo new hello_rust`,
    // open it in VS Code, and make it print three lines.
    // You can also practise right here: replace the three texts in quotes.
    println!("My name is Ferris.");
    println!("I'm from the Rust Playground.");
    println!("I'm learning Rust because I want fast AND safe programs.");

    // CHALLENGE (3 pips): build a program WITHOUT cargo.
    //   1. Make a file called hello.rs with a main function that prints something.
    //   2. Compile it:   rustc hello.rs
    //   3. Run it:       ./hello         (Windows: .\hello.exe)
    // Then find the program cargo built for your hello_rust project in
    // target/debug/ and run it directly, without `cargo run`.
}
