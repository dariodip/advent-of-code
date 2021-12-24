use crate::input::Input;

pub fn solve(_input: &mut Input) -> Result<u32, String> {
    Err("Not implemented".to_string())
}

#[test]
pub fn test_example() -> Result<(), String> {
    use crate::input::{test_part_one_error, test_part_two_error};
    let example_input = "";

    test_part_one_error!(example_input => "Not implemented".to_string());
    test_part_two_error!(example_input => "Not implemented".to_string());
    Ok(())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_error, test_part_two_error};

    let file_input = include_str!("day17_input.txt");
    test_part_one_error!(file_input => "Not implemented".to_string());
    test_part_two_error!(file_input => "Not implemented".to_string());
}
