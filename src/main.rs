mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod utils;

use std::io::{BufRead, BufReader};
use std::{env, fs};

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;
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
        .map(|line| line.expect("Should parse valid line").to_string())
}

///Get the solution for a specific day
fn get_solution(day: usize, input: impl Iterator<Item = String>) -> Box<dyn Solution> {
    match day {
        1 => Box::new(Day1::from_input(input)),
        2 => Box::new(Day2::from_input(input)),
        3 => Box::new(Day3::from_input(input)),
        4 => Box::new(Day4::from_input(input)),
        5 => Box::new(Day5::from_input(input)),
        6 => Box::new(Day6::from_input(input)),
        7 => Box::new(Day7::from_input(input)),
        8 => Box::new(Day8::from_input(input)),
        9 => Box::new(Day9::from_input(input)),
        10 => Box::new(Day10::from_input(input)),
        11 => Box::new(Day11::from_input(input)),
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
