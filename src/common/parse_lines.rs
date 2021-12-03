use std::str::FromStr;

pub fn parse_lines<T: FromStr>(input: &str) -> Result<Vec<T>, String> {
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| (idx, line.trim()))
        .filter(|(_, line)| !line.is_empty())
        .map(|(idx, line)| {
            line.parse::<T>()
                .map_err(|_| format!("Line {}: Not a valid integer", idx + 1))
        })
        .collect()
}
