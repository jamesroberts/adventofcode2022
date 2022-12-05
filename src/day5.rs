use crate::utils::{FromInput, Solution};
use std::collections::VecDeque;

struct MoveOp {
    num: u8,
    from: usize,
    to: usize,
}

pub struct Day5 {
    stacks: Vec<VecDeque<String>>,
    moves: Vec<MoveOp>
}

impl FromInput for Day5 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut stacks = Vec::new();
        let mut moves = Vec::new();
        for _ in 0..10 {
            stacks.push(VecDeque::new());
        }

        for line in input {
            let mut pos = 0;
            let mut index = 1;
            if line.contains("[") {
                while index <= line.len() {
                    let c = line.get(index..index+1).unwrap().to_string();
                    if !(c.contains(" ")){
                        let stack = stacks.get_mut(pos).unwrap();
                        stack.push_front(c);
                    }
                    index += 4;
                    pos += 1;
                }
            }
            
            if line.contains("move") {
                let op = line.replace("move ", "").replace(" from ", "-").replace(" to ", "-");
                let mut op = op.split("-");

                let mut move_op = MoveOp {
                    num: op.next().unwrap().parse().expect("Valid number"),
                    from: op.next().unwrap().parse().expect("Valid number"),
                    to: op.next().unwrap().parse().expect("Valid number"),
                };

                // Play nice with 0 based indexes 
                move_op.from -= 1;
                move_op.to -= 1;

                moves.push(move_op);
            }
        }
        Day5 {stacks, moves}
    }
}

impl Solution for Day5 {
    fn part_one(&self) -> String {
        let mut answer = String::new();
        let mut stacks = self.stacks.clone();

        for move_op in self.moves.iter() {
            for _ in 0..move_op.num {
                let cargo = stacks[move_op.from].pop_back().unwrap();
                stacks[move_op.to].push_back(cargo);
            }
        }

        for stack in stacks.iter() {
            if !stack.is_empty() {
                let letter = stack.to_owned().pop_back().unwrap();
                answer = format!("{}{}", answer, letter);
            }
        }
        format!("{}", answer)
    }

    fn part_two(&self) -> String {
        let mut answer = String::new();
        let mut stacks = self.stacks.clone();

        for move_op in self.moves.iter() {
            let stack = &stacks.clone()[move_op.from];
            let crates_to_move = stack.iter().rev().take(move_op.num as usize);

            for cargo in crates_to_move.rev() {
                stacks[move_op.to].push_back(cargo.to_owned());
            }

            for _ in 0..move_op.num {
                stacks[move_op.from].pop_back();
            }

            stacks = stacks;
        }

        for stack in stacks.iter() {
            if !stack.is_empty() {
                let letter = stack.to_owned().pop_back().unwrap();
                answer = format!("{}{}", answer, letter);
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
        let test_input = load_input(format!(".test_input/5.txt"));
        let day = Day5::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/5.txt"));
        let day = Day5::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "MCD");
    }
}
