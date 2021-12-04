use std::io::BufRead;

use crate::input::Input;

fn set_lowest_bits(n: u8) -> u16 {
    u16::MAX >> (u16::BITS as u16 - u16::from(n))
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let bit_size = input
        .text
        .lines()
        .find(|l| !l.is_empty())
        .take()
        .map(|l| l.trim().len())
        .unwrap_or_default();

    if bit_size == 0 {
        return Err("Error: bit size is 0".to_string());
    }

    let bit_counter = &mut [0; 16][0..bit_size];
    for line in input.text.lines() {
        for (i, c) in line.trim().bytes().rev().enumerate() {
            bit_counter[i] += if c == b'1' { 1 } else { -1 };
        }
    }

    let gamma: u16 = bit_counter
        .iter()
        .enumerate()
        .filter(|(_, &c)| c >= 0)
        .map(|(i, _)| 1 << i)
        .sum();

    let epsilon: u16 = !gamma & set_lowest_bits(bit_size as u8);

    Ok(u32::from(gamma * epsilon))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let text = r#"
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
    "#;

    test_part_one!(text => 198);

    // test_part_two!(text => 900);
    // let file_input = include_str!("day03_input.txt");
    // test_part_one!(file_input => 3148794);
    // test_part_two!(file_input => 1872757425);
}
