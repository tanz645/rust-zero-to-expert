//! Checks for the Day 5 exercise. Run them from the repo folder:
//!
//!     cargo test -p day005_printing
//!
//! Don't change this file. Change `src/main.rs` until every test passes.

use std::process::Command;

/// The exercise file, read as text so the tests can see which slots you used.
const SOURCE: &str = include_str!("../src/main.rs");

/// The line every way should print.
const LINE: &str = "Ferris is learning Rust, day 5 of 259.";

/// The note for standard error.
const NOTE: &str = "Note: 1 line, 4 ways.";

/// The challenge line, braces and all.
const BRACES: &str = "{ name: Ferris, day: 5 }";

/// Runs the program and returns what it printed: (standard output, standard error).
fn run() -> (String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_day005_printing"))
        .output()
        .expect("could not run the program");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Line `n` (counting from 1) of standard output, or "(nothing)".
fn line(n: usize) -> String {
    let (stdout, _) = run();
    stdout.lines().nth(n - 1).unwrap_or("(nothing)").to_string()
}

/// The source code with every comment taken out (text inside "..." is kept).
fn code() -> String {
    let mut out = String::new();
    let mut chars = SOURCE.chars().peekable();
    let mut in_text = false;
    while let Some(c) = chars.next() {
        if in_text {
            out.push(c);
            if c == '\\' {
                out.extend(chars.next());
            } else if c == '"' {
                in_text = false;
            }
        } else if c == '"' {
            in_text = true;
            out.push(c);
        } else if c == '/' && chars.peek() == Some(&'/') {
            while chars.peek().is_some_and(|&n| n != '\n') {
                chars.next();
            }
        } else if c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            let mut prev = ' ';
            for n in chars.by_ref() {
                if prev == '*' && n == '/' {
                    break;
                }
                prev = n;
            }
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

/// Every `name!(...)` call in your code, as (the text in quotes, what comes after it).
fn calls(name: &str) -> Vec<(String, String)> {
    let code = code();
    let needle = format!("{name}!(");
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(at) = code[from..].find(&needle) {
        let start = from + at;
        from = start + needle.len();
        // `println!(` is also the end of `eprintln!(`, so check the letter before it.
        if code[..start]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_alphanumeric() || c == '_')
        {
            continue;
        }
        let Some(args) = code[from..].trim_start().strip_prefix('"') else {
            continue;
        };
        // Find the closing quote of the text, then the closing ) of the call.
        let (mut text_end, mut escaped) = (None, false);
        for (i, c) in args.char_indices() {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                text_end = Some(i);
                break;
            }
        }
        let Some(text_end) = text_end else { continue };
        let after = &args[text_end + 1..];
        let rest = after.split(')').next().unwrap_or("");
        let rest = rest.trim().trim_start_matches(',').trim().to_string();
        found.push((args[..text_end].to_string(), rest));
    }
    found
}

#[test]
fn line_1_positional_slots() {
    assert_eq!(
        line(1),
        LINE,
        "\nLine 1 should be: {LINE}\nIf it doesn't compile: give every {{}} slot a value.\n"
    );
}

#[test]
fn line_2_named_slots() {
    assert_eq!(line(2), LINE, "\nLine 2 should be the same line again.\n");
    let named = calls("println")
        .iter()
        .any(|(text, rest)| text.contains("is learning") && rest.contains('='));
    assert!(
        named,
        "\nPrint line 2 with named slots, like {{who}}, and say what each name means after the text: who = name.\n"
    );
}

#[test]
fn line_3_inline_slots() {
    assert_eq!(line(3), LINE, "\nLine 3 should be the same line again.\n");
    let inline = calls("println")
        .iter()
        .any(|(text, rest)| text.contains("is learning") && text.contains('{') && rest.is_empty());
    assert!(
        inline,
        "\nPrint line 3 with inline slots: put the names in the slots, like {{name}}, and nothing after the text.\n"
    );
}

#[test]
fn line_4_built_with_format() {
    assert_eq!(line(4), LINE, "\nLine 4 should be the same line again.\n");
    let built = calls("format")
        .iter()
        .any(|(text, _)| text.contains("is learning"));
    assert!(
        built,
        "\nBuild line 4 with format! first: let card = format!(...); then print card.\n"
    );
}

#[test]
fn note_goes_to_standard_error() {
    let (stdout, stderr) = run();
    assert!(
        !stdout.contains(NOTE),
        "\nThe note went to standard output. Print it with eprintln!, not println!.\n"
    );
    assert_eq!(
        stderr.trim_end(),
        NOTE,
        "\nPrint exactly this to standard error, with eprintln!: {NOTE}\n"
    );
}

#[test]
fn challenge_real_braces() {
    let (stdout, _) = run();
    assert_eq!(
        line(5),
        BRACES,
        "\nLine 5 should be: {BRACES}\nTo print a real brace, double it: {{{{ prints {{ and }}}} prints }}.\n"
    );
    assert_eq!(
        stdout.lines().count(),
        5,
        "\nThe program should print exactly 5 lines.\n"
    );
}
