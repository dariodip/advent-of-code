# Advent of Code solutions

[![Pipeline](https://github.com/dariodip/advent-of-code-2021/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/dariodip/advent-of-code-2021/actions/workflows/rust.yml)

Solutions to [Advent of Code](https://adventofcode.com/) problems 2021 in Rust.

This repository holds a single Rust project that contains all the days.

## Run the solutions

The command line tool takes `<day> <part>` as arguments and reads the problem input from `stdin`. 
To run the solution for the first part of day `1` you can run:
```sh
$ cat src/year2021/day01_input.txt | cargo run -q 1 2
```

## Add new solutions

To add a new solution (*day*), you need to:
- Create a file `day01.rs` under `src/year2021`;
- Add a function `pub fn solve(input: &mut Input) -> Result<u32, String>` under that file;
- Export the function in `src/year2021/mod.rs`;

## Git hooks configuration
Execute this command to setup git hooks:
```
git config --local core.hooksPath ./hooks
```
