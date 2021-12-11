use std::collections::HashMap;

use crate::input::Input;

type Point = (u32, u32);

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let coords = get_points(input.text)?
        .into_iter()
        .filter(|(p1, p2)| {
            is_straight_line(p1, p2) || (is_perfect_diagonal_line(p1, p2) ^ !input.is_part_two())
        })
        .collect::<Vec<(Point, Point)>>();

    let mut point_map: HashMap<Point, u32> = HashMap::new();
    for (p1, p2) in coords {
        // for each point
        for point in get_line(&p1, &p2) {
            // iterate over each point in line between them
            let count = point_map.entry(point).or_insert(0);
            *count += 1;
        }
    }
    Ok(point_map.into_iter().filter(|(_, c)| *c >= 2).count() as u32)
}

fn get_line(p1: &Point, p2: &Point) -> Vec<Point> {
    if is_straight_line(p1, p2) {
        get_straight_line(p1, p2)
    } else {
        get_diagonal_line(p1, p2)
    }
}

fn get_straight_line(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut line = Vec::new();
    if p1.0 != p2.0 {
        // y are the same (vertical)
        let start = u32::min(p1.0, p2.0);
        let end = u32::max(p1.0, p2.0);
        for x in start..=end {
            line.push((x, p1.1));
        }
    } else {
        // x are the same (horizontal)
        let start = u32::min(p1.1, p2.1);
        let end = u32::max(p1.1, p2.1);
        for y in start..=end {
            line.push((p1.0, y));
        }
    }

    line
}

fn get_diagonal_line(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut line = Vec::new();
    let mut x = p1.0 as i32;
    let mut y = p1.1 as i32;

    let dx = ((p2.0 as i32) - x).signum();
    let dy = ((p2.1 as i32) - y).signum();

    while (x, y) != (p2.0 as i32, p2.1 as i32) {
        line.push((x as u32, y as u32));
        x += dx;
        y += dy;
    }
    line.push((x as u32, y as u32));

    line
}

fn get_points(text: &str) -> Result<Vec<(Point, Point)>, String> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|line| points_from_line(line, "->"))
        .collect::<Result<Vec<(Point, Point)>, String>>()
}

fn is_straight_line(start: &Point, end: &Point) -> bool {
    start.0 == end.0 || start.1 == end.1
}

fn is_perfect_diagonal_line(start: &Point, end: &Point) -> bool {
    i32::abs(start.0 as i32 - end.0 as i32) == i32::abs(start.1 as i32 - end.1 as i32)
}

fn points_from_line(line: &str, sep: &str) -> Result<(Point, Point), String> {
    // line is something like x1,y1 -> x2,y2
    let points = line
        .split(sep)
        .map(|str_point| str_points_to_point(str_point.trim()))
        .collect::<Result<Vec<Point>, String>>()?;

    match points[..] {
        [start, end, ..] => Ok((start, end)),
        _ => Err(format!("Invalid points string: {}", line)),
    }
}

fn str_points_to_point(str_points: &str) -> Result<Point, String> {
    let coords = str_points
        .split(',')
        .map(|p| {
            p.trim()
                .parse::<u32>()
                .map_err(|err| format!("Error while parsing ponints: {}", err))
        })
        .collect::<Result<Vec<u32>, String>>()?;

    match coords[..] {
        [x, y, ..] => Ok((x, y)),
        _ => Err("Invalid point string".to_string()),
    }
}

#[test]
pub fn test_get_straight_line() {
    let p1: Point = (1, 1);
    let p2: Point = (1, 3);

    let line = get_straight_line(&p1, &p2);
    let expected_line: Vec<Point> = vec![(1, 1), (1, 2), (1, 3)];
    for (i, p) in line.iter().enumerate() {
        assert_eq!(p, expected_line.get(i).unwrap());
    }
}

#[test]
pub fn test_get_diagonal_line() {
    let p1: Point = (1, 1);
    let p2: Point = (3, 3);

    let line = get_diagonal_line(&p1, &p2);
    let expected_line: Vec<Point> = vec![(1, 1), (2, 2), (3, 3)];
    for (i, p) in expected_line.iter().enumerate() {
        assert_eq!(p, line.get(i).unwrap());
    }
}

#[test]
pub fn test_points_from_line() -> Result<(), String> {
    let line = "0,9 -> 5,9";
    let points = points_from_line(line, "->")?;

    assert_eq!(points.0, (0, 9));
    assert_eq!(points.1, (5, 9));
    Ok(())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day05_input.txt");
    test_part_one!(file_input => 7318);
    test_part_two!(file_input => 19939);
}
