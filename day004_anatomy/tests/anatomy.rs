//! Checks for the Day 4 exercise. Run them from the repo folder:
//!
//!     cargo test -p day004_anatomy
//!
//! Don't change this file. Change `src/main.rs` until every test passes.

use std::process::Command;

/// The exercise file, read as text so the tests can look at your comments.
const SOURCE: &str = include_str!("../src/main.rs");

/// The text of the line you switch off.
const SWITCHED_OFF: &str = "This line should not print";

#[test]
fn prints_exactly_three_lines() {
    let out = Command::new(env!("CARGO_BIN_EXE_day004_anatomy"))
        .output()
        .expect("could not run the program");
    let printed = String::from_utf8_lossy(&out.stdout);
    let expected = "Rust: Zero to Expert\n\
                    Day 4: the anatomy of a Rust program\n\
                    I can name every symbol in this program.\n";
    assert_eq!(
        printed, expected,
        "\nThe program should print exactly 3 lines. \
         Did you switch off the line that should not print?\n"
    );
}

#[test]
fn switched_off_line_is_still_in_the_file() {
    assert!(
        SOURCE.contains(SWITCHED_OFF),
        "\nSwitch the line off with a comment. Don't delete it.\n"
    );
}

#[test]
fn main_has_a_doc_comment() {
    let lines: Vec<&str> = SOURCE.lines().map(str::trim).collect();
    let main_at = lines
        .iter()
        .position(|l| l.starts_with("fn main"))
        .expect("`fn main` is missing");
    let above = lines[..main_at].iter().rev().find(|l| !l.is_empty());
    assert!(
        above.is_some_and(|l| l.starts_with("///")),
        "\nAdd a /// doc comment on the line right above `fn main() {{`.\n"
    );
}

#[test]
fn has_a_block_comment() {
    assert!(
        SOURCE.contains("/*") && SOURCE.contains("*/"),
        "\nAdd at least one /* block comment */.\n"
    );
}

#[test]
fn every_line_of_code_has_a_comment() {
    let mut in_block = false;
    let mut comment_above = false;
    let mut missing = Vec::new();
    for (i, raw) in SOURCE.lines().enumerate() {
        let line = raw.trim();
        let is_comment = in_block || line.starts_with("//") || line.starts_with("/*");
        if in_block || line.starts_with("/*") {
            in_block = !line.contains("*/");
        } else if let Some(at) = line.find("/*") {
            in_block = !line[at..].contains("*/");
        }
        if line.is_empty() {
            continue;
        }
        if !is_comment {
            // Only look for a comment after the last quote, so text inside "..." doesn't count.
            let after_text = line.rsplit('"').next().unwrap_or(line);
            let has_comment = after_text.contains("//") || after_text.contains("/*");
            if !comment_above && !has_comment {
                missing.push(format!("  line {}: {}", i + 1, line));
            }
        }
        comment_above = is_comment || in_block;
    }
    assert!(
        missing.is_empty(),
        "\nThese lines have no comment. Add a // comment above each one, or at the end of it:\n{}\n",
        missing.join("\n")
    );
}
