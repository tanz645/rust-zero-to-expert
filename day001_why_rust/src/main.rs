// Day 1 · Why Rust? · Your turn
//
// No install needed today: paste this whole file into
// https://play.rust-lang.org and press "Run".
// (From Day 2 on you can run it locally: cargo run -p day001_why_rust)

fn main() {
    // WARM-UP (1 pip): run the program exactly as it is.
    println!("Hello, Rustacean!");

    // PRACTICE (2 pips): replace the two texts in quotes with your own
    // name and your background, e.g. "I know Python" or "I'm new to programming".
    let name = "Ferris";
    let background = "I'm new to programming";
    println!("I'm {name}. {background}. Day 1 of 259 done!");

    // CHALLENGE (3 pips): break this program on purpose, three different ways.
    //   1. Delete the semicolon `;` at the end of the `let name` line.
    //   2. Misspell `println!` as `printn!`.
    //   3. Delete the closing quote `"` after your name.
    // Each time: run it, read the error, and write down
    //   (a) what the compiler says went wrong, and
    //   (b) which line it points at.
    // Then fix it again. Surprise to look for: in one of the three cases the
    // compiler points at a different line than the one you broke. Which one?
}
