use core::panic;
use std::collections::HashSet;

use crate::utils::{FromInput, Solution};

pub struct Day9(Vec<String>);

impl FromInput for Day9 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        Day9(input.collect())
    }
}

fn get_change(d: &str) -> (i32, i32) {
    match d {
        "U" => (0, 1),
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => panic!("Invalid move")
    }
}

fn update(rope: &mut Vec<Vec<i32>>) {
    for i in 1..rope.len() {
        let (hx, hy) = (rope[i-1][0], rope[i-1][1]);
        let (tx, ty) = (rope[i][0], rope[i][1]);
        let (dx, dy) = (hx-tx, hy-ty);

        if dx.abs() > 1 || dy.abs() > 1 {
            rope[i][0] += dx.signum();
            rope[i][1] += dy.signum();
        }
    }
}


impl Solution for Day9 {
    fn part_one(&self) -> String {
        let mut rope = vec![vec![0, 0], vec![0, 0]];
        let end = rope.len() - 1;

        let mut positions = HashSet::new();
        positions.insert((0, 0));

        for line in self.0.iter() {
            let (d, n) = line.split_once(" ").unwrap();
            for _ in 0..n.parse::<usize>().unwrap() {
                let (dx, dy) = get_change(d);
                rope[0][0] += dx;
                rope[0][1] += dy;
                update(&mut rope);

                let (tx, ty) = (rope[end][0], rope[end][1]);
                positions.insert((tx, ty));
            }     

        }
        format!("{}", positions.len())
    }

    fn part_two(&self) -> String {
        let mut positions = HashSet::new();
        positions.insert((0, 0));

        let mut rope = Vec::new();
        for _ in 0..10 {
            rope.push(vec![0,0]);
        }
        let end = rope.len() - 1;

        for line in self.0.iter() {
            let (d, n) = line.split_once(" ").unwrap();
            for _ in 0..n.parse::<usize>().unwrap() {
                let (dx, dy) = get_change(d);
                rope[0][0] += dx;
                rope[0][1] += dy;
                update(&mut rope);

                let (tx, ty) = (rope[end][0], rope[end][1]);
                positions.insert((tx, ty));
            }     

        }
        format!("{}", positions.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/9a.txt"));
        let day = Day9::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/9b.txt"));
        let day = Day9::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "36");
    }
}
