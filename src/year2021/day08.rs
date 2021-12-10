use std::collections::HashSet;

use crate::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let (left, right) = extract_input(input.text)?;

    let decode_fn: fn(&str, usize, Vec<&str>) -> Option<u32> =
        input.part_values(decode_one, decode_two);

    if input.is_part_one() {
        Ok(right
            .iter()
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|value| decode_fn(value, value.len(), left.clone()))
                    .filter(|v| v.is_some())
                    .count()
            })
            .sum::<usize>() as u32)
    } else {
        let valid_count = right
            .iter()
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|value| decode_fn(value, value.len(), left.clone()))
                    .map(|v| v.unwrap())
                    .reduce(|a, num| a * 10 + num)
                    .unwrap_or(0)
            })
            .sum::<u32>();

        Ok(valid_count as u32)
    }
}

fn decode_one(_value: &str, count: usize, _input: Vec<&str>) -> Option<u32> {
    do_decode_one(count)
}

fn do_decode_one(count: usize) -> Option<u32> {
    match count {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

fn decode_two(value: &str, count: usize, input: Vec<&str>) -> Option<u32> {
    let mut patterns: Vec<_> = input
        .iter()
        .map(|d| d.chars().collect::<HashSet<_>>())
        .collect();
    patterns.sort_unstable_by_key(|p| p.len());

    let decoded = decode(value, count, patterns);
    println!("{} decoded is {}", value, decoded);
    Some(decoded)
}

//  00000000
// 1        2
// 1        2
// 1        2
//  33333333
// 4        5
// 4        5
// 4        5
//  66666666
fn decode(value: &str, count: usize, input: Vec<HashSet<char>>) -> u32 {
    if let Some(digit) = do_decode_one(count) {
        return digit;
    }

    let digit: HashSet<char> = value.chars().into_iter().collect::<HashSet<_>>();

    if digit.len() == 5 {
        if input[0].difference(&digit).count() == 0 {
            3
        } else if input[2].difference(&digit).count() == 1 {
            5
        } else {
            2
        }
    } else if input[0].difference(&digit).count() > 0 {
        6
    } else if input[2].difference(&digit).count() > 0 {
        0
    } else {
        9
    }
}

fn extract_input(text: &str) -> Result<(Vec<&str>, Vec<&str>), String> {
    let (input, output): (Vec<_>, Vec<_>) = text
        .lines()
        .filter(|l| !l.is_empty())
        .map(
            |pattern| match pattern.split('|').collect::<Vec<&str>>()[..] {
                [input, output, ..] => Ok((input.trim(), output.trim())),
                _ => Err(format!("Invalid input: {}", pattern)),
            },
        )
        .collect::<Result<Vec<(&str, &str)>, String>>()?
        .into_iter()
        .unzip();
    Ok((input, output))
}

#[test]
pub fn test_example() -> Result<(), String> {
    let example_input = r#"
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
    test_part_one!(example_input => 26);

    Ok(())
}
#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day08_input.txt");
    test_part_one!(file_input => 303);
    test_part_two!(file_input => 961734);
}
