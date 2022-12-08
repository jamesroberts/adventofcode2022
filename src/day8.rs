use crate::utils::{FromInput, Solution};

pub struct Day8(Vec<Vec<u32>>);

impl FromInput for Day8 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut grid = Vec::new();
        for line in input {
            let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            grid.push(row);
        }
        Day8(grid)
    }
}

fn tree_visible(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    if x <= 0 || y <= 0 || x >= grid.len() - 1 || y >= grid[x].len() - 1 {
        return true;
    }
    
    for xi in (x + 1)..grid.len() {
        if grid[xi][y] >= grid[x][y] {
            break;
        }
        if xi == grid.len() - 1 {
            return true;
        }
    }

    for xd in (0..x).rev() {
        if grid[xd][y] >= grid[x][y] {
            break;
        }
        if xd == 0 {
            return true;
        }
    }

    for yi in (y + 1)..grid.len() {
        if grid[x][yi] >= grid[x][y] {
            break;
        }
        if yi == grid.len() - 1 {
            return true;
        }
    }

    for yd in (0..y).rev() {
        if grid[x][yd] >= grid[x][y] {
            break;
        }
        if yd == 0 {
            return true;
        }
    }
    return false;
}

fn tree_scenic_score(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let mut score = 1;
    let mut ss = 0;

    for xi in (x + 1)..grid.len() {
        ss += 1;
        if grid[xi][y] >= grid[x][y] {
            break;
        }
    }
    score *= ss;

    ss = 0;
    for xd in (0..x).rev() {
        ss += 1;
        if grid[xd][y] >= grid[x][y] {
            break;
        }
    }
    score *= ss;

    ss = 0;
    for yi in (y + 1)..grid.len() {
        ss += 1;
        if grid[x][yi] >= grid[x][y] {
            break;
        }
    }
    score *= ss;

    ss = 0;
    for yd in (0..y).rev() {
        ss += 1;
        if grid[x][yd] >= grid[x][y] {
            break;
        }
    }
    score *= ss;

    return score;
}

impl Solution for Day8 {
    fn part_one(&self) -> String {
        let mut total = 0;
        let grid = self.0.clone();
        let x_len = grid.len();
        let y_len = grid[0].len();

        for x in 0..x_len {
            for y in 0..y_len {
                if tree_visible(&grid, x, y) {
                    total += 1;
                }
            }
        }
        format!("{}", total)
    }

    fn part_two(&self) -> String {
        let mut max = 0;
        let grid = self.0.clone();
        let x_len = grid.len();
        let y_len = grid[0].len();

        for x in 0..x_len {
            for y in 0..y_len {
                let score = tree_scenic_score(&grid, x, y);
                max = std::cmp::max(score, max);
            }
        }
        format!("{}", max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/8.txt"));
        let day = Day8::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "21");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/8.txt"));
        let day = Day8::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "8");
    }
}
