use crate::utils::{FromInput, Solution};

/// Model the problem for Day 1 using this struct
pub struct Day1 {
    elves: Vec<Vec<u32>>,
}

impl FromInput for Day1 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut elves = vec![Vec::new()];

        for line in input {
            let calories = elves.last_mut().expect("At least one vector");

            match line.parse() {
                Ok(number) => calories.push(number),
                Err(_) => elves.push(Vec::new()),
            }
        }

        Day1 { elves }
    }
}

impl Solution for Day1 {
    fn part_one(&self) -> String {
        let max: u32 = self
            .elves
            .iter()
            .map(|calories| calories.iter().sum())
            .max()
            .expect("At least one value to be the max");

        format!("{}", max)
    }

    fn part_two(&self) -> String {
        let mut sums: Vec<u32> = self.elves.iter().map(|cals| cals.iter().sum()).collect();

        sums.sort();
        format!("{}", sums.iter().rev().take(3).sum::<u32>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/1.txt"));
        let day = Day1::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "24000");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/1.txt"));
        let day = Day1::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "45000");
    }
}
