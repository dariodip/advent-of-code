use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let scores = input
        .text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| line_value(l))
        .filter(|(_, corrupt)| *corrupt == input.part_values(true, false))
        .map(|(v, _)| v);

    if input.is_part_one() {
        Ok(scores.sum::<u64>())
    } else {
        let mut scores = scores.collect::<Vec<u64>>();
        scores.sort_unstable();
        Ok(scores[scores.len() / 2])
    }
}

fn line_value(line: &str) -> (u64, bool) {
    // delimitet stack contains expected parentesis
    let mut delimiter_stack = Vec::new();
    for c in line.trim().chars() {
        match c {
            // insert into the stack stack
            '(' => delimiter_stack.push(')'),
            '[' => delimiter_stack.push(']'),
            '{' => delimiter_stack.push('}'),
            '<' => delimiter_stack.push('>'),
            _ => {
                if let Some(penality_points) = closing_value(c, delimiter_stack.pop()) {
                    if penality_points != 0 {
                        // invalid match
                        return (penality_points, true);
                    }
                } // else it is incomplete
            }
        }
    }
    delimiter_stack.reverse();
    (remaining_points(&delimiter_stack), false)
}

/// if matching is None, returns None => incomplete
/// otherwise it returns penality score
fn closing_value(c: char, matching: Option<char>) -> Option<u64> {
    matching.map(|matching| corruption_penality_points(c, matching))
}

/// if `c` matches `matching` returns 0, otherwise its penality score
fn corruption_penality_points(c: char, matching: char) -> u64 {
    if c == matching {
        return 0;
    }
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

/// returns the remaining points
fn remaining_points(remaining_stack: &[char]) -> u64 {
    let mut current_score = 0;
    for v in remaining_stack.iter() {
        current_score *= 5;
        current_score += match v {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    }

    current_score
}

#[test]
pub fn test_remaining_points() {
    assert_eq!(
        remaining_points(&"])}>".chars().collect::<Vec<char>>()),
        294
    );
}

#[test]
pub fn test_penality_points() {
    assert_eq!(corruption_penality_points(')', ')'), 0);
    assert_eq!(corruption_penality_points(')', '}'), 3);
    assert_eq!(corruption_penality_points(']', '>'), 57);
    assert_eq!(corruption_penality_points('}', '>'), 1197);
    assert_eq!(corruption_penality_points('>', ')'), 25137);
}

#[test]
pub fn test_line_corrupted_value() {
    assert_eq!(line_value("(())"), (0, false));
    assert_eq!(line_value("{([(<{}[<>[]}>{[]{[(<()>"), (1197, true));
}

#[test]
pub fn test_example() -> Result<(), String> {
    let example_input = r#"
    [({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]"#;

    test_part_one!(example_input => 26397);
    test_part_two!(example_input => 288957);

    Ok(())
}

#[test]
pub fn tests() {
    use crate::input::test_part_one;

    let file_input = include_str!("day10_input.txt");
    test_part_one!(file_input => 464991);
    //test_part_two!(file_input => 0);
}
