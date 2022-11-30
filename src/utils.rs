use std::time::Instant;

/// Reads the lines from the input file into a relevant
/// model of the data for the day's solution.
pub trait FromInput {
    fn from_input(lines: impl Iterator<Item = String>) -> Self;
}

/// Solutions for a day of Advent of Code.
pub trait Solution {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

/// Times the execution of a function.
// pub fn time_execution(func: impl Fn() -> String) -> (String, u128) {
pub fn time_execution(func: impl Fn() -> String) -> (String, f32) {
    let timer = Instant::now();
    let result = func();
    let duration = timer.elapsed();

    (result, duration.as_secs_f32())
}
