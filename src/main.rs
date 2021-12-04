#![forbid(unsafe_code)]
use std::env;
use std::io::Read;

use advent_of_code::solve_raw;

fn main() -> Result<(), String> {
    let usage = || -> ! {
        eprintln!("Arguments: <day> <part>");
        eprintln!("    where: day is 1-25");
        eprintln!("       and part is 1 or 2");
        std::process::exit(1);
    };

    let args: Vec<String> = env::args().collect();

    if args.iter().any(|s| s == "-v" || s == "--version") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if args.len() == 3 {
        let day = &args[1];
        let part = &args[2];
        let mut input = String::new();

        std::io::stdin()
            .read_to_string(&mut input)
            .map_err(|e| format!("Error reading input: {}", e.to_string()))?;

        let solution =
            solve_raw(day, part, input.as_ref()).unwrap_or_else(|e| format!("Error: {}", e));
        println!("{}", solution);
    } else {
        usage();
    }

    Ok(())
}
