#![allow(unused)]
pub mod days;

use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
pub struct Args {
    /// use example input instead of solution input
    #[arg(short, long, default_value_t = false)]
    pub example: bool,

    /// which part to run (1 or 2)
    #[arg(short, long, default_value_t = 1)]
    pub part: usize,
}

pub fn run_day<F1, F2>(day: u16, part1: F1, part2: F2)
where
    F1: FnOnce(&str),
    F2: FnOnce(&str),
{
    let args = Args::parse();

    if args.part != 1 && args.part != 2 {
        panic!("Invalid part: {}. Must be 1 or 2", args.part);
    }

    let input_type = if args.example { "example" } else { "solution" };
    let input_path = format!("src/days/day{}/{}.txt", day, input_type);

    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", input_path));

    if args.part == 1 {
        part1(&input);
    } else {
        part2(&input);
    }
}
