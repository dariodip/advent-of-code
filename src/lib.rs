#![forbid(unsafe_code)]
#![crate_name = "advent_of_code"]

mod year2021;

// Never inline to prevent stack size from blowing up in release builds.
#[inline(never)]
fn to_stringer_input<T: ToString>(
    function: fn(&mut String) -> Result<T, String>,
    input: &mut String,
) -> Result<String, String> {
    function(input).map(|value| value.to_string())
}

pub fn solve(day: u8, mut input: String) -> Result<String, String> {
    if input.is_empty() {
        return Err("No input provided".to_string());
    } else if input.len() > 200_00 {
        return Err("Input too large".to_string());
    } else if !input.is_ascii() {
        return Err("Non-ASCII input provided".to_string());
    } else if !matches!(day, 1..=25) {
        return Err("Invalid day provided".to_string());
    }

    match day {
        1 => to_stringer_input(year2021::day01::solve, &mut input),
        2 => to_stringer_input(year2021::day01::solve, &mut input),
        _ => Err(format!("Unsupported day {}", day)),
    }
}

/// A version of [solve](fn.solve.html) that takes strings as arguments and parses them to the required types.
pub fn solve_raw(day: &str, input: &str) -> Result<String, String> {
    let day = day.parse::<u8>().map_err(|_| "Invalid day")?;
    solve(day, input.to_string())
}
