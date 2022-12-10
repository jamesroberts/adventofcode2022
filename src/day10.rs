use crate::utils::{FromInput, Solution};

pub struct Day10(Vec<String>);

impl FromInput for Day10 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        Day10(input.map(|s| s.trim().to_string()).collect())
    }
}

impl Solution for Day10 {
    fn part_one(&self) -> String {

        fn inc(c: &mut isize, answer: &mut isize, x: isize) {
            *c += 1; 
            if [20, 60, 100, 140, 180, 220].contains(&c) {
                *answer += *c * x;
            }
        }

        let mut answer = 0;
        let mut x = 1;
        let mut c = 1;

        for line in self.0.iter() {
            if line == "noop" {
                inc(&mut c, &mut answer, x);
                continue;
            }
            inc(&mut c, &mut answer, x);
            x += line.split_once(" ").unwrap().1.parse::<isize>().unwrap();
            inc(&mut c, &mut answer, x)
        }
        format!("{}", answer)

    }

    fn part_two(&self) -> String {
        fn inc(c: &mut usize, x: isize, grid: &mut Vec<Vec<&str>>) {
            *c += 1; 
            let row = usize::try_from(*c / 40).unwrap();
            let col = usize::try_from(*c % 40).unwrap();
            let pos = x - col as isize;

            if pos.abs() <= 1 {
                grid[row][col] = "#";
            }
        }

        fn build_grid<'a>() -> Vec<Vec<&'a str>> {
            let mut grid: Vec<Vec<&str>> = Vec::new();
            for _ in 0..6 {
                let mut row = Vec::with_capacity(40);
                for _ in 0..40 {
                    row.push(" ");
                }
                grid.push(row);
            }
            grid
        }

        let mut x = 1;
        let mut c = 0;
        let mut grid = build_grid();

        for line in self.0.iter() {
            if line == "noop" {
                inc(&mut c, x, &mut grid);
                continue;
            }
            inc(&mut c, x, &mut grid);
            x += line.split_once(" ").unwrap().1.parse::<isize>().unwrap();
            inc(&mut c, x, &mut grid);
        }
        let mut answer = "".to_string();

        for row in grid.iter() {
            let r = row.join("");
            answer = format!("{}\n{}", answer, r);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/10.txt"));
        let day = Day10::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "13140");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/10.txt"));
        let day = Day10::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "\n \
              #  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n\
             ###   ###   ###   ###   ###   ###   ### \n\
             ####    ####    ####    ####    ####    \n\
             #####     #####     #####     #####     \n\
             ######      ######      ######      ####\n\
             #######       #######       #######     ");
    }
}
