use crate::input::Input;

enum SubmarineCommand {
    Forward(u32),
    Up(u32),
    Down(u32),
}

struct Submarine {
    x: u32,
    y: u32,
    aim: u32,
}

impl Submarine {
    fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }

    fn take_command(&mut self, command: SubmarineCommand) {
        match command {
            SubmarineCommand::Forward(x) => self.x += x,
            SubmarineCommand::Up(y) => self.y -= y,
            SubmarineCommand::Down(y) => self.y += y,
        }
    }

    fn take_aim_command(&mut self, command: SubmarineCommand) {
        match command {
            SubmarineCommand::Forward(s) => {
                self.x += s;
                self.y += self.aim * s;
            }
            SubmarineCommand::Down(s) => self.aim += s,
            SubmarineCommand::Up(s) => self.aim -= s,
        }
    }
}

impl SubmarineCommand {
    fn translate(raw_command: &str) -> Result<Self, String> {
        let raw_command: Vec<&str> = raw_command.trim().split(' ').collect();
        let direction = match raw_command.first() {
            Some(direction) => direction,
            None => return Err("Cannot extract a direction".to_string()),
        };

        let steps = match raw_command.last() {
            Some(steps) => steps,
            None => return Err("Cannot extract number of steps".to_string()),
        };

        Self::raw_to_command(direction, steps)
    }

    fn raw_to_command(direction: &str, steps: &str) -> Result<Self, String> {
        let steps = steps
            .parse::<u32>()
            .map_err(|err| format!("Error: {}", err))?;

        let command = match direction.to_lowercase().as_ref() {
            "forward" => Self::Forward(steps),
            "up" => Self::Up(steps),
            "down" => Self::Down(steps),
            _ => return Err(format!("{} is not a valid direction", direction)),
        };

        Ok(command)
    }
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let commands = input
        .text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|line| {
            SubmarineCommand::translate(line)
                .map_err(|err| format!("Cannot translate raw command because: {}", err))
        })
        .collect::<Result<Vec<SubmarineCommand>, String>>()?;

    let mut submarine = Submarine::new();

    for command in commands {
        if input.is_part_one() {
            submarine.take_command(command);
        } else {
            submarine.take_aim_command(command);
        }
    }

    Ok(submarine.x * submarine.y)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let text = r#"
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
    "#;

    test_part_one!(text => 150);
    test_part_two!(text => 900);

    let file_input = include_str!("day02_input.txt");
    test_part_one!(file_input => 1636725);
    test_part_two!(file_input => 1872757425);
}
