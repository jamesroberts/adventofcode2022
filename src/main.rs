mod day1;
mod utils;

use std::io::{BufRead, BufReader};
use std::{env, fs};

use day1::Day1;
use utils::{time_execution, Solution};

use crate::utils::FromInput;

/// Reads the day from the arguments
fn get_day_from_args() -> usize {
    env::args()
        .nth(1)
        .expect("Must provide a day to solve")
        .parse::<usize>()
        .expect("Provided day wasn't a valid integer")
}

/// Reads the input for a day from the `.input` directory.
fn load_input(file_path: String) -> impl Iterator<Item = String> {
    let file = fs::OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("File should be present");

    let buffered_file = BufReader::new(file);

    buffered_file
        .lines()
        .map(|line| line.expect("Should parse valid line").trim().to_string())
}

///Get the solution for a specific day
fn get_solution(day: usize, input: impl Iterator<Item = String>) -> Box<dyn Solution> {
    match day {
        1 => Box::new(Day1::from_input(input)),
        _other => panic!("No solution for day {day}"),
    }
}

fn main() {
    let day = get_day_from_args();
    let file_path = format!(".input/{day}.txt");
    let input = load_input(file_path);
    let solution = get_solution(day, input);

    println!("Solving day {day}...");
    println!("==================================================");

    let (part_one, duration) = time_execution(|| solution.part_one());
    println!("Part 1: {part_one} (took {duration} seconds)");

    println!("==================================================");
    let (part_two, duration) = time_execution(|| solution.part_two());
    println!("Part 2: {part_two} (took {duration} seconds)");
}
