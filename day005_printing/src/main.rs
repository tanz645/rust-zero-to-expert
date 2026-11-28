// Day 5 · Printing & placeholders
// Print one line four ways, send a note to standard error, then print real braces.
// Check your work with:  cargo test -p day005_printing

fn main() {
    let name = "Ferris";
    let language = "Rust";
    let day = 5;

    // 1 · Positional slots: the values fill the {} in order.
    println!("{} is learning {}, day {} of 259.", name, language);

    // 2 · Named slots: print the same line with {who}, {what} and {when}.

    // 3 · Inline slots: print the same line with {name}, {language} and {day}.
    //     Nothing goes after the text.

    // 4 · format!: build the same line into `card`, then print `card`.

    // 5 · eprintln!: print this note to standard error:  Note: 1 line, 4 ways.

    // Challenge · print this line, braces and all:  { name: Ferris, day: 5 }
}
