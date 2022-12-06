use std::collections::HashSet;

use crate::utils::{FromInput, Solution};

pub struct Day6(String);

impl FromInput for Day6 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        Day6(input.last().unwrap())
    }
}

impl Solution for Day6 {
    fn part_one(&self) -> String {
        let offset = 4;
        
        let mut answer = 0;
        for i in 0..self.0.len() {
            let window = self.0.get(i..i+offset).unwrap(); 
            let set: HashSet<char> = HashSet::from_iter(window.chars());
            if set.len() == offset {
                answer = i+offset;
                break;
            }
        }
        format!("{}", answer)
    }

    fn part_two(&self) -> String {
        let offset = 14;
        
        let mut answer = 0;
        for i in 0..self.0.len() {
            let window = self.0.get(i..i+offset).unwrap(); 
            let set: HashSet<char> = HashSet::from_iter(window.chars());
            if set.len() == offset {
                answer = i+offset;
                break;
            }
        }
        format!("{}", answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/6.txt"));
        let day = Day6::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "7");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/6.txt"));
        let day = Day6::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "19");
    }
}
