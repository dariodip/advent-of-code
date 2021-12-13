use core::num;
use std::{str::FromStr, vec};

pub fn parse_lines<T: FromStr>(input: &str) -> Result<Vec<T>, String> {
    let f = |i: usize, line: &str| {
        line.parse::<T>()
            .map_err(|_| format!("Line {}: Not a valid integer {}", i + 1, line))
    };

    map_lines(input, f)
}

pub fn map_lines<T, F>(input: &str, f: F) -> Result<Vec<T>, String>
where
    F: Fn(usize, &str) -> Result<T, String>,
{
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| (idx, line.trim()))
        .filter(|(_, line)| !line.is_empty())
        .map(|(idx, line)| f(idx, line))
        .collect()
}

#[test]
pub fn test_map_lines() -> Result<(), String> {
    let lines = "1\n2\n3\n";

    let parse_and_increment = |i: usize, line: &str| {
        line.parse::<u8>()
            .map_err(|_| format!("error on {} with value {}", i, line))
            .map(|n| n + 1)
    };
    let nums = map_lines(lines, parse_and_increment)?;
    let expected = vec![2_u8, 3, 4];

    nums.iter()
        .zip(expected)
        .for_each(|(&l, r)| assert_eq!(l, r));

    Ok(())
}

#[test]
pub fn test_parse_lines() -> Result<(), String> {
    let lines = "1\n2\n3\n";

    let nums= parse_lines::<u8>(lines)?;
    let expected = vec![1_u8, 2, 3];

    nums.iter()
        .zip(expected)
        .for_each(|(&l, r)| assert_eq!(l, r));

    Ok(())
}