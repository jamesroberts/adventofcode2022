use crate::utils::{FromInput, Solution};

#[derive(Clone, Copy)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(Clone, Copy)]
enum Strategy {
    X,
    Y,
    Z,
}

struct Game {
    choice: Choice,
    strategy: Strategy,
}

pub struct Day2 {
    games: Vec<Game>,
}

impl FromInput for Day2 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut choices = Vec::new();
        let mut strategies = Vec::new();

        for line in input {
            let mut guide = line.split_whitespace();

            match guide.next() {
                Some("A") => choices.push(Choice::ROCK),
                Some("B") => choices.push(Choice::PAPER),
                Some("C") => choices.push(Choice::SCISSORS),
                _ => panic!("Invalid letter"),
            };

            match guide.next() {
                Some("X") => strategies.push(Strategy::X),
                Some("Y") => strategies.push(Strategy::Y),
                Some("Z") => strategies.push(Strategy::Z),
                _ => panic!("Invalid letter"),
            };
        }
        let mut games: Vec<Game> = Vec::new();
        let iter = choices.iter().zip(strategies.iter());

        for (choice, strategy) in iter {
            games.push(Game {
                choice: *choice,
                strategy: *strategy,
            });
        }

        Day2 { games }
    }
}

impl Solution for Day2 {
    fn part_one(&self) -> String {
        let mut score = 0;

        for game in self.games.iter() {
            score += match game.strategy {
                Strategy::X => 1, // play ROCK
                Strategy::Y => 2, // play PAPER
                Strategy::Z => 3, // play SCISSORS
            };

            score += match (game.choice, game.strategy) {
                (Choice::ROCK, Strategy::X) => 3,
                (Choice::ROCK, Strategy::Y) => 6,
                (Choice::ROCK, Strategy::Z) => 0,

                (Choice::PAPER, Strategy::X) => 0,
                (Choice::PAPER, Strategy::Y) => 3,
                (Choice::PAPER, Strategy::Z) => 6,

                (Choice::SCISSORS, Strategy::X) => 6,
                (Choice::SCISSORS, Strategy::Y) => 0,
                (Choice::SCISSORS, Strategy::Z) => 3,
            };
        }

        format!("{}", score)
    }

    fn part_two(&self) -> String {
        let mut score = 0;

        for game in self.games.iter() {
            score += match game.strategy {
                Strategy::X => 0, // LOSS
                Strategy::Y => 3, // DRAW
                Strategy::Z => 6, // WIN
            };

            score += match (game.strategy, game.choice) {
                // Must choose to lose
                (Strategy::X, Choice::ROCK) => 3,
                (Strategy::X, Choice::PAPER) => 1,
                (Strategy::X, Choice::SCISSORS) => 2,
                // Must choose to draw
                (Strategy::Y, Choice::ROCK) => 1,
                (Strategy::Y, Choice::PAPER) => 2,
                (Strategy::Y, Choice::SCISSORS) => 3,
                // Must choose to win
                (Strategy::Z, Choice::ROCK ) => 2,
                (Strategy::Z, Choice::PAPER) => 3,
                (Strategy::Z, Choice::SCISSORS ) => 1,
            };
        }
        format!("{}", score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/2.txt"));
        let day = Day2::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "15");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/2.txt"));
        let day = Day2::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "12");
    }
}
