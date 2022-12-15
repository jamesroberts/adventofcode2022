use crate::utils::{FromInput, Solution};

use std::{cmp::{min, max}, collections::HashMap};
use nom::{bytes::complete::tag, sequence::tuple, IResult, combinator::map};

pub struct Day15(Vec<SensorBeaconPair>);

#[derive(Debug)]
struct SensorBeaconPair {
    sensor: (isize, isize),
    beacon: (isize, isize),
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map(nom::character::complete::i64, |n| n as isize)(input)
}

fn parse_line(input: &str) -> IResult<&str, SensorBeaconPair> {
    map(
        tuple((
            tag("Sensor at x="),
            parse_isize,
            tag(", y="),
            parse_isize,
            tag(": closest beacon is at x="),
            parse_isize,
            tag(", y="),
            parse_isize
        )),
        |(_, sx, _, sy, _, bx, _, by)| SensorBeaconPair { sensor: (sx, sy), beacon: (bx, by) }
    )(input)
}

impl FromInput for Day15 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut pairs = Vec::new();
        for line in input {
            pairs.push(parse_line(line.as_str()).unwrap().1);
        }
        Day15(pairs)
    }
}

fn distance(x: (isize, isize), y: (isize, isize)) -> isize {
    (x.0.abs_diff(y.0) + x.1.abs_diff(y.1)) as isize
}

fn fill(pair: &SensorBeaconPair, y_target: isize, grid: &mut HashMap<(isize, isize), char>) {
    let d = distance(pair.sensor, pair.beacon);

    for nx in pair.sensor.0-d..=pair.sensor.0+d {
        let ny = y_target;
        if distance(pair.sensor, (nx as isize, ny)) <= d {
            if !grid.contains_key(&(nx, ny)) {
                grid.insert((nx, ny), '#');
            } 
        } 

    }
}


impl Solution for Day15 {
    fn part_one(&self) -> String {
        let y_target: usize;

        if cfg!(test) {
            y_target = 10;
        } else {
            y_target = 2000000;
        }

        let mut grid = HashMap::new();
        for pair in self.0.iter() {
            grid.insert(pair.sensor, 'S');
            grid.insert(pair.beacon, 'B');
            fill(pair, y_target as isize, &mut grid);
        }

        let mut answer = 0;
        for (k, &v) in grid.iter() {
            if k.1 == y_target as isize && v == '#' {
                answer += 1;
            }
        }
        format!("{}", answer)
    }

    fn part_two(&self) -> String {
        let limit: isize;

        if cfg!(test) {
            limit = 20;
        } else {
            limit = 4_000_000;
        }

        fn is_not_visible(x: isize, y: isize, pairs: &Vec<SensorBeaconPair>, limit: isize) -> bool {
            if x < 0 || x > limit || y < 0 || y > limit {
                return false;
            }

            for pair in pairs.iter() {
                let d = distance(pair.sensor, pair.beacon);
                if distance(pair.sensor, (x, y)) <= d {
                    return false;
                }
            }
            
            return true;
        }

        let mut answer = (0, 0);

        for pair in self.0.iter() {
            let d = distance(pair.sensor, pair.beacon);
            let right_corner = (pair.sensor.0 + d, pair.sensor.1);
            let left_corner = (pair.sensor.0 - d, pair.sensor.1);

            for i in 0..=d+1 {
                let (x, y) = (right_corner.0 + 1 - i, right_corner.1 - i);
                if is_not_visible(x, y, &self.0, limit) {
                    answer = (x, y);
                    break;
                }
                let (x, y) = (right_corner.0 + 1 - i, right_corner.1 + i);
                if is_not_visible(x, y, &self.0, limit) {
                    answer = (x, y);
                    break;
                }
                let (x, y) = (left_corner.0 + 1 - i, right_corner.1 - i);
                if is_not_visible(x, y, &self.0, limit) {
                    answer = (x, y);
                    break;
                }
                let (x, y) = (left_corner.0 + 1 - i, right_corner.1 + i);
                if is_not_visible(x, y, &self.0, limit) {
                    answer = (x, y);
                    break;
                }
            }
        }
        format!("{}", answer.0 * 4_000_000 + answer.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/15.txt"));
        let day = Day15::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "26");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/15.txt"));
        let day = Day15::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "56000011");
    }
}
