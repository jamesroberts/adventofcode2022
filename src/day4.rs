use crate::utils::{FromInput, Solution};

pub struct Day4(Vec<Vec<u16>>);

impl FromInput for Day4 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut ranges = Vec::new();
        for line in input {
            let line = line.replace(",", "-");
            let ids = line.split("-").map(|n| n.parse().unwrap()).collect();
            ranges.push(ids);
        }

        Day4(ranges)
    }
}

impl Solution for Day4 {
    fn part_one(&self) -> String {
        let mut total = 0;

        for ids in self.0.iter() {
            let (a, b, x, y) = (ids[0], ids[1], ids[2], ids[3]);

            if a <= x && b >= y || x <= a && y >= b {
                total += 1
            }
        }
        format!("{}", total)
    }

    fn part_two(&self) -> String {
        let mut total = 0;

        for ids in self.0.iter() {
            let (a, b, x, y) = (ids[0], ids[1], ids[2], ids[3]);
            
            if b >= x && a <= y {
                total += 1
            }
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
        let test_input = load_input(format!(".test_input/4.txt"));
        let day = Day4::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/4.txt"));
        let day = Day4::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "4");
    }
}
