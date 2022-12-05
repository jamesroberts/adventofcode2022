#!/bin/sh

set -e

YEAR=2022

if test -z "$1"; then
  echo "Must provide a day"
  exit 1
fi

if [[ 1 -gt "$1" || 25 -lt "$1" ]]; then
  echo "Day must be between 1 and 25, inclusive"
  exit 1
fi

SESSION=$(cat .session)
if test -z "$SESSION"; then
  echo "Must set the session from the Advent of Code site"
  exit 1
fi

if test -e ".input/$1.txt"; then
  echo "Data already exists for day $1, skipping download..."
else
  echo "Downloading data for day $1 to .input/$1.txt..."
  mkdir -p .input
  mkdir -p .test_input
  curl "https://adventofcode.com/$YEAR/day/$1/input" \
    --silent --max-time 10 --cookie "session=$SESSION" > ".input/$1.txt"
  touch .test_input/$1.txt
fi

if test -e "src/day$1.rs"; then
  echo "src/day$1.rs already exists, skipping..."
else
  echo "Creating boilerplate module for day $1 at src/day$1.rs..."
  echo "Remember to update main.rs:"
  echo "  - Add 'mod day$1;'"
  echo "  - Add 'use day$1::Day$1;'"
  echo "  - Update 'get_solution' to use 'Day$1'"

  cat <<-EOF > "src/day$1.rs"
use crate::utils::{FromInput, Solution};

/// Model the problem for Day $1 using this struct
pub struct Day$1;

impl FromInput for Day$1 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        todo!("Implement input parsing for day $1");
    }
}

impl Solution for Day$1 {
    fn part_one(&self) -> String {
        todo!("Implement solution for part one");
    }

    fn part_two(&self) -> String {
        todo!("Implement solution for part two");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/$1.txt"));
        let day = Day$1::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "answer");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/$1.txt"));
        let day = Day$1::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "answer");
    }
}
EOF
fi

if test -e "$1.py"; then
  echo "$1.py already exists, skipping..."
else
  cat <<-EOF > "$1.py"
t = 0

while True:
    try:
        line = input().strip()
    except:
        break

    # a, b = map(int, line.split("-"))
    # t += 1

print(t)
EOF

fi

cat <<-EOF > "aoc"
python3 $1.py < .test_input/$1.txt
echo "======================"
python3 $1.py < .input/$1.txt
EOF
chmod +x aoc

echo "Happy coding!"
