use std::collections::HashMap;

use crate::input::Input;

type Pair = (char, char);

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut lines = input.text.lines();

    let polymer_template = lines
        .next()
        .ok_or("Cannot read first line")?
        .chars()
        .collect::<Vec<_>>();

    lines.next();

    let pairs = lines
        .map(|ln| {
            let chars = ln.chars().collect::<Vec<_>>();
            ((chars[0], chars[1]), chars[6])
        })
        .collect::<Vec<_>>();

    let mut pair_map = HashMap::new();
    // add first step
    for pair in polymer_template.windows(2) {
        *pair_map.entry((pair[0], pair[1])).or_default() += 1;
    }

    for _ in 0..input.part_values(10, 40) {
        let mut to_add: Vec<(Pair, u64)> = Vec::new();
        let mut to_remove: Vec<(Pair, u64)> = Vec::new();
        for &(new_pair, new_char) in pairs.iter() {
            if let Some(&count) = pair_map.get(&new_pair) {
                to_add.push(((new_pair.0, new_char), count));
                to_add.push(((new_char, new_pair.1), count));
                to_remove.push((new_pair, count));
            }
        }
        for (added, count) in to_add {
            *pair_map.entry(added).or_default() += count;
        }
        for (removed, count) in to_remove {
            *pair_map.entry(removed).or_default() -= count;
        }
    }

    let mut element_freqs: HashMap<char, u64> = HashMap::new();
    for (key, count) in pair_map.iter() {
        *element_freqs.entry(key.0).or_default() += count;
        *element_freqs.entry(key.1).or_default() += count;
    }

    // edge elements
    *element_freqs.entry(polymer_template[0]).or_default() += 1;
    *element_freqs
        .entry(polymer_template[polymer_template.len() - 1])
        .or_default() += 1;

    let most_common = element_freqs
        .iter()
        .map(|(_, c)| c)
        .max()
        .ok_or("Cannot find max on element frequencies")?;
    let least_common = element_freqs
        .iter()
        .map(|(_, c)| c)
        .min()
        .ok_or("Cannot find max on element frequencies")?;
    Ok((most_common - least_common) / 2)
}

#[test]
pub fn test_example() -> Result<(), String> {
    use crate::input::{test_part_one, test_part_two};
    let example_input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    test_part_one!(example_input => 1_588);
    test_part_two!(example_input => 2_188_189_693_529);

    Ok(())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day14_input.txt");
    test_part_one!(file_input => 2112);
    test_part_two!(file_input => 3243771149914);
}
