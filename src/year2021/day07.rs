use crate::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let numbers = input
        .text
        .split(',')
        .map(str::parse::<u32>)
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|_| "Input is not comma-separated u16 values".to_string())?;
    
    let max_value = numbers
        .iter()
        .max()
        .cloned()
        .unwrap_or(0);

    let distance_fn: fn(i32) -> i32 = input.part_values(identity, gaussian_sum);

    let mut min_consuption = i32::MAX;
    for i in 0..=max_value {
        let consumption = numbers
            .iter()
            .map(|n| 
                distance_fn(*n as i32 - i as i32).abs())
            .sum::<i32>();
        if min_consuption > consumption {
            min_consuption = consumption;
        }
    }

    Ok(min_consuption as u32)
}

fn identity(distance: i32) -> i32 {
    distance
}

fn gaussian_sum(distance: i32) -> i32 {
    (distance * (distance + 1)) / 2
}

#[test]
pub fn test_example() -> Result<(), String> {
    let example_input = "16,1,2,0,4,2,7,1,2,14";
    test_part_one!(example_input => 37);
    test_part_two!(example_input => 145);

    Ok(())
}
#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two_error};

    let file_input = include_str!("day07_input.txt");
    test_part_one!(file_input => 333_755);

    test_part_one_error!("" => "Input is not comma-separated u16 values");
    test_part_two_error!("" => "Input is not comma-separated u16 values");
}
