use crate::utils::{FromInput, Solution};
use std::collections::{VecDeque, HashSet};


pub struct Day12(Vec<Vec<char>>);

impl FromInput for Day12 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut grid = Vec::new();
        for line in input {
            grid.push(line.chars().collect::<Vec<char>>());
        }
        Day12(grid)
    }
}

fn elevation(letter: char) -> u32 {
   match letter {
       'S' => 1,
       'E' => 26,
       c => u32::from(c) - u32::from('a') + 1
   } 
} 

fn bfs(grid: Vec<Vec<char>>, queue: &mut VecDeque<(i32, usize, usize)>) -> String {
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut seen = HashSet::new();
    let mut answer = "0".to_string();

    while queue.len() > 0 {
        let (d, r, c) = queue.pop_front().unwrap();

        if seen.contains(&(r, c)) {
            continue;
        }

        seen.insert((r, c));

        if grid[r][c] == 'E' {
            answer = d.to_string();
        }

        let dirs = vec![(-1,0),(0,1),(1,0),(0,-1)];
        for (dr, dc) in dirs.iter() {
            let rr = (r as i32 + *dr as i32) as usize ;
            let cc = (c as i32 + *dc as i32) as usize ;

            if rr < rows && cc < cols {
                let ce = elevation(grid[r][c]);
                let de = elevation(grid[rr][cc]);

                if de <= ce + 1 {
                    queue.push_back((d + 1, rr, cc));
                }
            }
        }
    }
    answer
}

impl Solution for Day12 {
    fn part_one(&self) -> String {
        let mut queue = VecDeque::new();
        let grid = self.0.clone(); 

        let rows = grid.len();
        let cols = grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 'S' {
                    queue.push_back((0, r, c));
                }
            }
        }
        format!("{}", bfs(grid, &mut queue))
    }

    fn part_two(&self) -> String {
        let mut queue = VecDeque::new();
        let grid = self.0.clone(); 

        let rows = grid.len();
        let cols = grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if elevation(grid[r][c]) == 1 {
                    queue.push_back((0, r, c));
                }
            }
        }
        format!("{}", bfs(grid, &mut queue))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/12.txt"));
        let day = Day12::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "31");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/12.txt"));
        let day = Day12::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "29");
    }
}
