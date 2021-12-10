use std::collections::HashSet;

use crate::Input;

type Location = (u8, usize, usize);

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let risk_matrix = create_risk_matrix(input.text)?;

    let max_x = risk_matrix.len() - 1;
    let max_y = risk_matrix[0].len() - 1;
    let low_points = find_low_points(&risk_matrix, max_x, max_y);

    if input.is_part_one() {
        Ok(low_points.iter().map(|(el, _, _)| (*el as u32) + 1).sum())
    } else {
        let mut lp_basins_size = low_points
            .into_iter()
            .map(|lp| {
                let mut seen = HashSet::new();
                basin(lp, max_x, max_y, &risk_matrix, &mut seen);
                seen.len() as u32}
            )
            .collect::<Vec<_>>();
        lp_basins_size.sort_unstable();
        let lp_count = lp_basins_size.len() - 1;
        let mut result = 1;

        for i in 0..3 {
            result *= lp_basins_size[lp_count - i];
        }
        Ok(result)
    }
}

fn basin(
    loc: Location,
    max_x: usize,
    max_y: usize,
    matrix: &[Vec<u8>],
    seen: &mut HashSet<Location>
) -> HashSet<Location> {
    seen.insert(loc);
    let mut set = HashSet::new();

    // adj is the set of adjacents that are not in the seen set and are not 9s
    let adjacents = adjacents(loc.1, loc.2, max_x, max_y, matrix)
        .into_iter()
        .filter(|(adj_v, _, _)| *adj_v != 9)
        .filter(|adj| !seen.contains(adj))
        .collect::<HashSet<_>>();

    if adjacents.is_empty() {
        adjacents
    } else {
        let adjacent_basins = adjacents
            .into_iter()
            .map(|l| basin(l, max_x, max_y, matrix, seen));
        for basin in adjacent_basins {
            for el in basin {
                set.insert(el);
            }
        }
        set
    }
}

fn find_low_points(matrix: &[Vec<u8>], max_x: usize, max_y: usize) -> Vec<Location> {
    let mut low_points = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            let adjacents = adjacents(i, j, max_x, max_y, matrix);
            if is_min(*el, adjacents) {
                low_points.push((*el, i, j));
            }
        }
    }

    low_points
}

fn create_risk_matrix(input: &str) -> Result<Vec<Vec<u8>>, String> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| c.to_string()
                    .parse::<u8>()
                    .map_err(|err| err.to_string()))
                .collect::<Result<Vec<u8>, String>>()
        })
        .collect::<Result<Vec<Vec<u8>>, String>>()
}

fn is_min(n: u8, els: Vec<Location>) -> bool {
    n < els.into_iter().map(|el| el.0).min().unwrap_or(0)
}

fn adjacents(
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
    matrix: &[Vec<u8>],
) -> Vec<Location> {
    let mut adjacent = Vec::new();
    if x != 0 {
        adjacent.push((matrix[x - 1][y], x - 1, y));
    }
    if x < max_x {
        adjacent.push((matrix[x + 1][y], x + 1, y));
    }
    if y != 0 {
        adjacent.push((matrix[x][y - 1], x, y - 1));
    }
    if y < max_y {
        adjacent.push((matrix[x][y + 1], x, y + 1));
    }

    adjacent
}

#[test]
pub fn test_basin() -> Result<(), String> {
    let example_input = r#"
    2199943210
    3987894921
    9856789892
    8767896789
    9899965678"#;
    let max_x = 4;
    let max_y = 9;
    let risk_matrix = create_risk_matrix(example_input)?;
    let low_points = find_low_points(&risk_matrix, max_x, max_y);

    for low_point in low_points.into_iter() {
        let mut basin_set = HashSet::new();
        basin(low_point, max_x, max_y, &risk_matrix, &mut basin_set);
        assert!(!basin_set.is_empty());
    }

    Ok(())
}

#[test]
pub fn test_example() -> Result<(), String> {
    let example_input = r#"
    2199943210
    3987894921
    9856789892
    8767896789
    9899965678"#;
    test_part_one!(example_input => 15);
    test_part_two!(example_input => 1134);

    Ok(())
}
#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day08_input.txt");
    test_part_one!(file_input => 303);
    //test_part_two!(file_input => 0);
}
