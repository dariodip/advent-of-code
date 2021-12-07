#![forbid(unsafe_code)]
#![crate_name = "advent_of_code"]

mod common;
#[cfg_attr(test, macro_use)]
mod input;
mod year2021;

use crate::input::{Input, Part};

// Never inline to prevent stack size from blowing up in release builds.
#[inline(never)]
fn to_stringer_input<T: ToString>(
    function: fn(&mut Input) -> Result<T, String>,
    input: &mut Input,
) -> Result<String, String> {
    function(input).map(|value| value.to_string())
}

pub fn solve(day: u8, part: u8, input: &str) -> Result<String, String> {
    if input.is_empty() {
        return Err("No input provided".to_string());
    } else if input.len() > 200_000 {
        return Err("Input too large".to_string());
    } else if !input.is_ascii() {
        return Err("Non-ASCII input provided".to_string());
    } else if !matches!(day, 1..=25) {
        return Err("Invalid day provided".to_string());
    }

    let mut input = Input {
        part: if part == 1 { Part::One } else { Part::Two },
        text: input,
    };

    match day {
        1 => to_stringer_input(year2021::day01::solve, &mut input),
        2 => to_stringer_input(year2021::day02::solve, &mut input),
        3 => to_stringer_input(year2021::day03::solve, &mut input),
        4 => to_stringer_input(year2021::day04::solve, &mut input),
        5 => to_stringer_input(year2021::day05::solve, &mut input),
        6 => to_stringer_input(year2021::day06::solve, &mut input),
        _ => Err(format!("Unsupported day {}", day)),
    }
}

/// A version of [solve](fn.solve.html) that takes strings as arguments and parses them to the required types.
pub fn solve_raw(day: &str, part: &str, input: &str) -> Result<String, String> {
    let day = day.parse::<u8>().map_err(|_| "Invalid day")?;
    let part = part.parse::<u8>().map_err(|_| "Invalid part")?;
    solve(day, part, input)
}
