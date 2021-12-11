use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut board = Grid::parse(input.text)?;

    for step in 1..=input.part_values(100, 1_000_000) {
        let previous_flashes = board.flashes_count;
        board.advance();
        let current_flashes = board.flashes_count;
        if input.is_part_two() && current_flashes - previous_flashes == 100 {
            return Ok(step);
        }
    }

    Ok(board.flashes_count)
}

struct Grid {
    cells: [u8; Self::SIZE * Self::SIZE],
    flashes_count: u32,
}

impl Grid {
    const SIZE: usize = 10;

    fn parse(s: &str) -> Result<Self, String> {
        let mut board = Self {
            cells: [0; Self::SIZE * Self::SIZE],
            flashes_count: 0,
        };

        // remove whitespaces and collect in a String
        let s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();

        if s.len() != 100 {
            return Err("Grid must have 100 cels".to_string());
        }
        for (i, c) in s.chars().enumerate() {
            let n = c.to_string().parse::<u8>().map_err(|err| err.to_string())?;
            board.cells[i] = n;
        }

        Ok(board)
    }

    fn at(&self, x: usize, y: usize) -> u8 {
        self.cells[x + (y * Self::SIZE)]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.cells[x + (y * Self::SIZE)] = value;
    }

    fn advance(&mut self) {
        // increment each cell
        self.cells.iter_mut().for_each(|c| *c += 1);

        for y in 0..10 {
            for x in 0..10 {
                // check for flashes
                if self.at(x, y) > 9 {
                    self.flash(x, y);
                }
            }
        }
    }

    fn flash(&mut self, x: usize, y: usize) {
        let current_value = self.at(x, y);
        // already flashed
        if current_value == 0 {
            return;
        }

        self.set(x, y, current_value + 1); // increment position

        if current_value + 1 > 9 {
            // flashed
            self.set(x, y, 0);
            self.flashes_count += 1;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dx, dy) != (0, 0) {
                        let n_x = x as i32 + dx;
                        let n_y = y as i32 + dy;
                        if (0..10).contains(&n_x) && (0..10).contains(&n_y) {
                            self.flash(n_x as usize, n_y as usize);
                        }
                    }
                }
            }
        }
    }
}

#[test]
pub fn test_example() -> Result<(), String> {
    use crate::input::{test_part_one, test_part_two};
    let example_input = r#"5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526"#;

    test_part_one!(example_input => 1656);
    test_part_two!(example_input => 195);

    Ok(())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day11_input.txt");
    test_part_one!(file_input => 1617);
    test_part_two!(file_input => 258);
}
