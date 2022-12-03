use std::collections::HashSet;
use crate::utils::{FromInput, Solution};

pub struct Day3(Vec<String>);

impl FromInput for Day3 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut rucksacks = Vec::new();
        for line in input {
            rucksacks.push(line.to_string());
        }
        Day3(rucksacks)
    }
}

impl Solution for Day3 {
    fn part_one(&self) -> String {
    
        let mut total = 0;
        for rucksack in self.0.iter() {
            let middle = rucksack.len() / 2; 
            let (first, second) = rucksack.split_at(middle);

            let first_set: HashSet<char> = HashSet::from_iter(first.chars());
            let second_set: HashSet<char> = HashSet::from_iter(second.chars());

            let intersection = first_set.intersection(&second_set);
            let common_letter = intersection.last().expect("At least one letter");
            let ord: u32 = common_letter.to_owned().into();

            if common_letter.is_lowercase() {
                total += ord - (97-1); // a = 1, b = 2
            } else {
                total += ord - (65-27);  // A = 27, B = 28
            }

        }
        format!("{}", total)
    }

    fn part_two(&self) -> String {
        let mut total = 0;
        let mut start = 0;

        while start <= (self.0.len() - 3) {
            let first_rucksack = &self.0[start];
            let mut set: HashSet<char> = HashSet::from_iter(first_rucksack.chars());

            for offset in 1..3 {
                let backpack = &self.0[start+offset];
                let temp_set: HashSet<char> = HashSet::from_iter(backpack.chars());
                set = set.intersection(&temp_set).copied().collect();
            }
            
            let common_letter = set.iter().last().unwrap();
            let ord: u32 = common_letter.to_owned().into();

            if common_letter.is_lowercase() {
                total += ord - (97-1); // a = 1, b = 2
            } else {
                total += ord - (65-27);  // A = 27, B = 28
            }

            start += 3;
        }
        format!("{}", total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/3.txt"));
        let day = Day3::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "157");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/3.txt"));
        let day = Day3::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "70");
    }
}
