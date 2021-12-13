use crate::common::recognize_chars;
use crate::input::Input;
use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Point(u32, u32),
    FoldAlongX(u32),
    FoldAlongY(u32),
}

impl Instruction {
    fn from_str(input: &str) -> Result<Self, String> {
        if input.to_lowercase().starts_with("fold along y") {
            let foldy = input.replace("fold along y=", "");
            let y = foldy.parse::<u32>().map_err(|err| {
                format!("Error while parsing line {}: {}", input, err.to_string())
            })?;
            Ok(Self::FoldAlongY(y))
        } else if input.to_lowercase().starts_with("fold along x") {
            let foldx = input.replace("fold along x=", "");
            let x = foldx.parse::<u32>().map_err(|err| {
                format!("Error while parsing line {}: {}", input, err.to_string())
            })?;
            Ok(Self::FoldAlongX(x))
        } else {
            let (x, y) = input
                .trim()
                .split_once(",")
                .ok_or(format!("Invalid string {}", input))?;
            let x = x.parse::<u32>().map_err(|err| {
                format!("Error while parsing line {}: {}", input, err.to_string())
            })?;
            let y = y.parse::<u32>().map_err(|err| {
                format!("Error while parsing line {}: {}", input, err.to_string())
            })?;
            Ok(Self::Point(x, y))
        }
    }
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let instructions = parse(input.text)?;
    let mut dots = Vec::new();

    for instruction in instructions {
        match instruction {
            Instruction::Point(x, y) => dots.push((x, y)),
            Instruction::FoldAlongX(n) => {
                for (x, _) in dots.iter_mut() {
                    if *x > n {
                        *x = 2 * n - *x;
                    }
                }
                if input.is_part_one() {
                    break;
                }
            }
            Instruction::FoldAlongY(n) => {
                for (_, y) in dots.iter_mut() {
                    if *y > n {
                        *y = 2 * n - *y;
                    }
                }
                if input.is_part_one() {
                    break;
                }
            }
        }
    }
    if input.is_part_one() {
        dots.sort_unstable();
        dots.dedup();
        return Ok(dots.len().to_string());
    }

    let dots = HashSet::<(u32, u32)>::from_iter(dots);
    let mut code = String::new();

    for letter in 0..8 {
        let mut char_string = String::new();
        for y in 0..6 {
            for x in 0..5 {
                let x_pos = letter * 5 + x;
                char_string.push(if dots.contains(&(x_pos, y)) { '#' } else { ' ' });
            }
            if y != 5 {
                char_string.push('\n');
            }
        }
        let c = recognize_chars::recognize(&char_string)?;
        code.push(c);
    }
    Ok(code)
}

fn parse(text: &str) -> Result<Vec<Instruction>, String> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Instruction::from_str(l))
        .collect::<Result<Vec<Instruction>, String>>()
}

#[test]
pub fn test_example() -> Result<(), String> {
    use crate::input::{test_part_one, test_part_two};
    let example_input = r#"6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5"#;

    test_part_one!(example_input => 17);
    // test_part_two!(example_input => 36);

    Ok(())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day13_input.txt");
    // test_part_one!(file_input => 5920);
    // test_part_two!(file_input => 155477);
}
