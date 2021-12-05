use crate::input::Input;

fn set_lowest_bits(n: u8) -> u16 {
    u16::MAX >> (u16::BITS as u16 - u16::from(n))
}

fn is_mostly_set(values: &[u16], index: usize) -> bool {
    values
        .iter()
        .fold(0, |acc, x| acc + if x & (1 << index) == 0 { -1 } else { 1 })
        >= 0
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

    if input.is_part_two() {
        return solve_two(input.text, bit_size);
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

fn solve_two(input: &str, bitsize: usize) -> Result<u32, String> {
    // create numbers
    let mut numbers = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|line| {
            u16::from_str_radix(line, 2)
                .map_err(|_| format!("Byte {} is not a binary integer", line))
        })
        .collect::<Result<Vec<u16>, _>>()?;

    let oxygen = keep_one_by_criteria(&mut numbers, bitsize, true)?;
    let co2 = keep_one_by_criteria(&mut numbers, bitsize, false)?;

    Ok(u32::from(oxygen) * u32::from(co2))
}

fn keep_one_by_criteria(
    numbers: &mut [u16],
    bitsize: usize,
    want_most: bool,
) -> Result<u16, String> {
    let mut candidates_count = numbers.len();

    for i in (0..bitsize).rev() {
        let mostly_set = is_mostly_set(&numbers[0..candidates_count], i); // true -> 1
        let mut candidate_index = 0;
        while candidate_index < candidates_count {
            let is_bit_set = (numbers[candidate_index] & (1 << i)) != 0;
            if (is_bit_set == mostly_set) == want_most {
                candidate_index += 1;
            } else {
                candidates_count -= 1;
                numbers.swap(candidate_index, candidates_count);
            }
        }
        if candidates_count == 1 {
            return Ok(numbers[0]);
        }
    }

    Err("Bit criteria did not result in single number".to_string())
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
    test_part_two!(text => 230);
}
