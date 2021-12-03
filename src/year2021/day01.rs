use crate::common::parse_lines;
use crate::input::{Input, self};

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let measurement = parse_lines::<u32>(input.text)?;

    if input.is_part_one() {
        solve_part_one(measurement)
    } else {
        solve_part_two(measurement)
    }
}

pub fn solve_part_one(measurement: Vec<u32>) -> Result<u32, String> {
    Ok(measurement
        .windows(2)
        .filter(|data| data.last() > data.first())
        .count()
        .try_into()
        .unwrap())
}

pub fn solve_part_two(measurement: Vec<u32>) -> Result<u32, String> {
    Ok(measurement
        .windows(3)
        .map(|window| -> u32 { window.iter().sum() })
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|data| data.last() > data.first())
        .count()
        .try_into()
        .unwrap())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day01_input.txt");
    test_part_one!(file_input => 1681);
    test_part_two!(file_input => 1704);
}
