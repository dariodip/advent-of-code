use crate::input::Input;
use core::num;
use std::fmt;

/// Cell represents a cell of a board. Besides the number it contains the index
/// of the column and the row of that number to make it most efficient to mark
/// it as extracted
#[derive(Clone, Copy)]
struct Cell {
    /// the number
    value: u8,
    /// the index of the column
    col: usize,
    /// the index of the row
    row: usize,
}

struct Board {
    /// The sum of the numbers in the columns
    cols: [u32; 5],
    /// The sum of the numbers in the rows
    rows: [u32; 5],
    /// The cells of the board
    cells: [Cell; 25],
    /// The index of the last value set
    last_set: usize,
}

impl Board {
    fn new() -> Self {
        Self {
            cols: [0; 5],
            rows: [0; 5],
            cells: [Cell {
                value: 0,
                col: 0,
                row: 0,
            }; 25],
            last_set: 0,
        }
    }

    fn add_row(&mut self, row: &[u8; 5]) -> Result<(), String> {
        if self.last_set >= 24 {
            return Err(format!("Board already full when trying to insert {:?}", row));
        }

        for cell in row {
            let row = self.last_set / 5;
            let col = self.last_set % 5;

            self.cells[self.last_set] = Cell {
                value: *cell,
                row,
                col,
            };
            self.rows[row] += u32::from(*cell);
            self.cols[col] += u32::from(*cell);
            self.last_set += 1;
        }
        Ok(())
    }

    fn try_mark(&mut self, num: u8) -> bool {
        let mut win = false;
        for cell in self.cells {
            if cell.value == num {
                let row_idx = cell.row;
                let col_idx = cell.col;
                self.cols[col_idx] -= u32::from(num);
                self.rows[row_idx] -= u32::from(num);
                if self.cols[col_idx] == 0 {
                    win = true;
                    break;
                }
                if self.rows[row_idx] == 0 {
                    win = true;
                    break;
                }
            }
        }

        win
    }

    fn unmarked_sum(&self) -> Result<u32, String> {
        if self.cols.iter().any(|n| *n == 0) {
            Ok(self.cols.iter().map(|n| *n as u32).sum::<u32>())
        } else if self.rows.iter().any(|n| *n == 0) {
            Ok(self.rows.iter().map(|n| *n as u32).sum::<u32>())
        } else {
            Err("No winning side".to_string())
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = "".to_string();
        self.cells.iter().enumerate().for_each(|(i, cell)| {
            display.push_str(format!(" {} ", cell.value).as_str());
            if (i + 1) % 5 == 0 {
                display.push('\n');
            }
        });
        let rows = self.rows.map(|n| format!("{}", n)).join(" | ");
        let cols = self.cols.map(|n| format!("{}", n)).join(" | ");

        write!(f, "{}\nRows: {}\nCols: {}", display, rows, cols)
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut lines = input.text.lines();

    // extract first line
    let mut first_line = lines.next();
    while first_line.is_none() || first_line.unwrap_or("").is_empty() {
        first_line = lines.next();
    }
    let extraction = match first_line {
        Some(first_line) => row_to_extractions(first_line)?,
        None => return Err("Cannot parse extraction".to_string()),
    };

    // extract boards
    let mut boards = input
        .text
        .split("\n\n") // split boards
        .skip(1) // skip extraction line
        .map(|board_str| -> Result<Board, String> {
            let mut board = Board::new();
            board_str
                .lines()
                .try_for_each(|row| -> Result<(), String> {
                    let numbers = row_to_numbers(row)?;
                    board.add_row(&numbers)?;
                    Ok(())
                })?;

            Ok(board)
        })
        .collect::<Result<Vec<Board>, String>>()?;

    for number in extraction {
        for board in boards.iter_mut() {
            if board.try_mark(number) {
                return Ok(board.unmarked_sum()? * u32::from(number))
            }
        }
    }

    Err("No board won".to_string())
}

fn row_to_extractions(row: &str) -> Result<Vec<u8>, String> {
    let mut num_vec = Vec::new();
    let numbers = row.split(',').map(str::trim).filter(|n| !n.is_empty());

    for ns in numbers {
        let n = ns.parse::<u8>().map_err(|err| err.to_string())?;
        num_vec.push(n);
    }

    Ok(num_vec)
}

fn row_to_numbers(row: &str) -> Result<[u8; 5], String> {
    let mut numbers = [0_u8; 5];
    for (i, c) in row.split(' ').filter(|n| !n.is_empty()).enumerate() {
        let n = c
            .parse::<u8>()
            .map_err(|err| format!("Error {} while converting {}", err, c))?;
        numbers[i] = n;
    }

    Ok(numbers)
}

#[test]
pub fn test_winning_board() -> Result<(), String> {
    let input = r#"
    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19"#;

    let extraction = [0_u8, 22, 13, 8, 17, 11].iter();

    let mut board = Board::new();
    let lines = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty());

    for line in lines {
        let numbers = row_to_numbers(line)?;
        board.add_row(&numbers)?;
    }

    let mut won = false;
    for num in extraction {
        won = board.try_mark(*num);
    }
    assert!(won);
    let expected_rows = vec![0_u32, 53, 67, 42, 67];
    let expected_cols = vec![28_u32, 33, 60, 53, 55];
    for (i, row_value) in board.rows.into_iter().enumerate() {
        assert_eq!(row_value, expected_rows[i]);
    }
    for (i, col_value) in board.cols.into_iter().enumerate() {
        assert_eq!(col_value, expected_cols[i]);
    }
    Ok(())
}

#[test]
pub fn test_board() -> Result<(), String> {
    let input = r#"
    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19"#;

    let mut board = Board::new();
    let lines = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty());

    for line in lines {
        let numbers = row_to_numbers(line)?;
        board.add_row(&numbers)?;
    }
    let expected_rows = vec![63_u32, 61, 67, 42, 67];
    let expected_cols = vec![58_u32, 46, 77, 64, 55];
    for (i, row_value) in board.rows.into_iter().enumerate() {
        assert_eq!(row_value, expected_rows[i]);
    }
    for (i, col_value) in board.cols.into_iter().enumerate() {
        assert_eq!(col_value, expected_cols[i]);
    }
    Ok(())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day04_input.txt");
    test_part_one!(file_input => 111540);
    //test_part_two!(file_input => 0);
}
