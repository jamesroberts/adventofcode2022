use crate::utils::{FromInput, Solution};


#[derive(Debug, Clone)]
pub struct Day11 {
    items: Vec<Vec<usize>>,
    ops: Vec<(char, usize)>,
    test: Vec<usize>,
    _true: Vec<usize>,
    _false: Vec<usize>,
    count: Vec<usize>
}

impl FromInput for Day11 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let lines = input.collect::<Vec<String>>();
        let mut input = lines.iter();

        let mut items = Vec::new();
        let mut ops = Vec::new();
        let mut test = Vec::new();
        let mut _true = Vec::new();
        let mut _false = Vec::new();
        let mut count = Vec::new();

        loop {
            let line = match input.next() {
                Some(line) => line,
                None => break,
            };
            if *line == "".to_string() {
                continue;
            }

            items.push(input.next().unwrap().trim().split_once("Starting items: ").unwrap().1.split(", ").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>());
            let op = input.next().unwrap().trim().split_once("Operation: new = old ").unwrap().1.split_once(" ").unwrap();
            let sign = op.0.chars().nth(0).unwrap();
            let op_num = op.1.parse::<usize>().unwrap_or(0);
            ops.push((sign, op_num));
            test.push(input.next().unwrap().trim().split_once("Test: divisible by ").unwrap().1.parse::<usize>().unwrap());
            _true.push(input.next().unwrap().trim().split_once("If true: throw to monkey ").unwrap().1.parse::<usize>().unwrap());
            _false.push(input.next().unwrap().trim().split_once("If false: throw to monkey ").unwrap().1.parse::<usize>().unwrap());
            count.push(0);
        }
        Day11 { items, ops, test, _true, _false, count }
    }
}

fn worry_level(old: usize, op: (char, usize)) -> usize {
    let (sign, mut num) = op;
    if num == 0 {
        num = old;
    }
    match sign {
        '+' => old + num,
        '*' => old * num,
        _ => panic!("Invalid sign")
    }
}

impl Solution for Day11 {
    fn part_one(&self) -> String {
        let mut items: Vec<Vec<usize>> = self.items.clone();
        let mut count: Vec<usize> = self.count.clone();

        for _ in 0..20 {
            for i in 0..count.len() {
                if self.items[i].is_empty() {
                    continue;
                }
               
                for item in 0..items[i].len() {
                    let op = self.ops[i];
                    let test = self.test[i];
                    let item = items[i][item];
                    let x = worry_level(item, op) / 3;

                    match x % test == 0 {
                        true => items.get_mut(self._true[i]).unwrap().push(x),
                        false => items.get_mut(self._false[i]).unwrap().push(x),
                    };
                }
                count[i] += items[i].len();
                items[i] = Vec::new();  
            }
        }
        count.sort();
        let answer = count.iter().rev().take(2).fold(1, |acc, c| acc * c);
        format!("{}", answer)
    }

    fn part_two(&self) -> String {
        let mut items: Vec<Vec<usize>> = self.items.clone();
        let mut count: Vec<usize> = self.count.clone();
        let lcm = self.test.iter().fold(1, |acc, t| acc * t);

        for _ in 0..10000 {
            for i in 0..count.len() {
                if self.items[i].is_empty() {
                    continue;
                }
               
                for item in 0..items[i].len() {
                    let op = self.ops[i];
                    let test = self.test[i];
                    let item = items[i][item];
                    let x = worry_level(item, op) % lcm;

                    match x % test == 0 {
                        true => items.get_mut(self._true[i]).unwrap().push(x),
                        false => items.get_mut(self._false[i]).unwrap().push(x),
                    };
                }
                count[i] += items[i].len();
                items[i] = Vec::new();  
            }
        }
        count.sort();
        let answer = count.iter().rev().take(2).fold(1, |acc, c| acc * c);
        format!("{}", answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/11.txt"));
        let day = Day11::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "10605");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/11.txt"));
        let day = Day11::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "2713310158");
    }
}
