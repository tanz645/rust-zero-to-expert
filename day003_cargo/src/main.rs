// Day 3 · The Cargo workflow · Your turn
//
// This program has four problems ON PURPOSE. Let cargo find each one:
//
//   cargo run    -p day003_cargo   WARM-UP    It runs, but read the warning and fix it
//   cargo fmt    -p day003_cargo   PRACTICE   Tidy up the messy code (one command)
//   cargo clippy -p day003_cargo   PRACTICE   Fix what clippy suggests
//   cargo test   -p day003_cargo   CHALLENGE  Make the failing test pass
//
// You don't need to understand every line yet: functions and tests get their
// own lessons later. Read what each tool tells you and follow its hints.
// Done when all four commands finish without a warning or a failure.

fn main() {
let name = "Ferris";
        let mood = "happy";
  println!("Hello, {name}!");
    println!("");
println!("Days left in the course: {}", days_left( 3 ));
}

/// How many lessons are still ahead after finishing `today`.
fn days_left(today:u32)->u32{
    259 - today - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_day_3_there_are_256_days_left() {
        assert_eq!(days_left(3), 256);
    }
}
