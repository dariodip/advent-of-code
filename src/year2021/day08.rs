use crate::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let (_input, output): (Vec<_>, Vec<_>) = input
        .text
        .lines()
        .map(|pattern|
            match pattern.split('|').collect::<Vec<&str>>()[..] {
                [input, output, ..] => Ok((input.trim(), output.trim())),
                _ => Err(format!("Invalid input: {}", pattern))
            } )
        .collect::<Result<Vec<(&str, &str)>, String>>()?
        .into_iter()
        .unzip();
    
    let valid_count = output
        .iter()
        .map(|s| 
            s
            .split_ascii_whitespace()
            .map(|value| value.len())
            .map(|count| match count {
                2 => Some(1),
                4 => Some(4),
                3 => Some(7),
                7 => Some(8),
                _ => None
            })
            .filter(Option::is_some)
            .count()
        )
        .sum::<usize>();

    Ok(valid_count as u32)
}

#[test]
pub fn test_example() -> Result<(), String> {
    let example_input = r#"
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
    fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
    fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
    cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
    efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
    gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
    gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
    cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
    ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
    gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
    fgae cfgab fg bagce"#;
    test_part_one!(example_input => 26);
    //test_part_two!(example_input => 168);

    Ok(())
}
#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day08_input.txt");
    test_part_one!(file_input => 0);
    //test_part_two!(file_input => 0);
}
