use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut initial_school = initial_school(input.text)?;
    for _ in 0..input.part_values(80, 256) {
        lanternfish_cycle(&mut initial_school)
    }

    Ok(initial_school.iter().sum::<u64>() as u64)
}

fn initial_school(text: &str) -> Result<[u64; 9], String> {
    let mut school = [0_u64; 9];
    let initial_fishes = text
        .split(',')
        .map(|n| n.trim())
        .filter(|n| !n.is_empty())
        .map(|n| {
            n.parse::<usize>()
                .map_err(|err| format!("Error while parsing: {}", err))
        })
        .collect::<Result<Vec<usize>, String>>()?;

    for fish in initial_fishes.into_iter() {
        school[fish] += 1;
    }

    Ok(school)
}

fn lanternfish_cycle(school: &mut [u64; 9]) {
    school.rotate_left(1);
    school[6] += school[8];
}

#[test]
pub fn test_example() -> Result<(), String> {
    let example_input = "3,4,3,1,2";
    let expected_lanternfish = 5934;

    let mut school = initial_school(example_input)?;
    for _ in 0..80 {
        lanternfish_cycle(&mut school);
    }
    assert_eq!(school.iter().sum::<u64>(), expected_lanternfish);

    Ok(())
}
#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day06_input.txt");
    test_part_one!(file_input => 360_761);
    test_part_two!(file_input => 1_632_779_838_045);
}
