use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::input::Input;

type Coord = (i16, i16);

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut grid = parse(input.text);
    if input.is_part_two() {
        let (width, height) = grid
            .keys()
            .max()
            .map(|(x, y)| (x + 1, y + 1))
            .ok_or_else(|| "Cannot expand grid".to_string())?;
        grid = grid
            .into_iter()
            .flat_map(|((x, y), risk)| {
                (0..5).cartesian_product(0..5).map(move |(tile_x, tile_y)| {
                    (
                        (tile_x * width + x, tile_y * height + y),
                        (risk + tile_x as u32 + tile_y as u32 - 1) % 9 + 1,
                    )
                })
            })
            .collect();
    }
    Ok(best_total_risk(&grid))
}

fn parse(text: &str) -> HashMap<Coord, u32> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i16, y as i16), c.to_digit(10).unwrap()))
        })
        .collect::<HashMap<_, _>>()
}

fn best_total_risk(grid: &HashMap<Coord, u32>) -> u32 {
    let mut best_known = HashMap::new();
    // start from top left
    let mut queue = BinaryHeap::from([(Reverse(0), 0, 0)]);
    while let Some((Reverse(total_risk), x, y)) = queue.pop() {
        let best_known_risk = best_known.entry((x, y)).or_insert(u32::MAX);
        if total_risk < *best_known_risk {
            *best_known_risk = total_risk;
            for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let (x, y) = (x + dx, y + dy);
                if let Some(risk) = grid.get(&(x, y)) {
                    queue.push((Reverse(total_risk + risk), x, y));
                }
            }
        }
    }
    best_known[best_known.keys().max().unwrap()]
}

#[test]
pub fn test_example() -> Result<(), String> {
    use crate::input::{test_part_one, test_part_two};
    let example_input = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";

    test_part_one!(example_input => 40);
    test_part_two!(example_input => 315);

    Ok(())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day15_input.txt");
    test_part_one!(file_input => 673);
    test_part_two!(file_input => 2893);
}
