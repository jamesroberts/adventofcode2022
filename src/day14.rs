use std::collections::HashMap;
use std::cmp::{min, max};
use crate::utils::{FromInput, Solution};
use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult, combinator::map};

pub struct Day14(Vec<Line>);

#[derive(Debug)]
struct Line {
    // start, end
    x: (usize, usize),
    y: (usize, usize)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u64, |n| n as usize)(input)
}

fn parse_point(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(parse_usize, tag(","), parse_usize)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(tag(" -> "), parse_point)(input)
}

impl FromInput for Day14 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut lines = Vec::new();

        for line in input {
            let points = parse_line(line.as_str()).unwrap().1;
            for i in 0..points.len()-1 {
                let xs = (points[i].0, points[i+1].0);
                let ys = (points[i].1, points[i+1].1);

                lines.push(Line {
                    x: (min(xs.0, xs.1), max(xs.0, xs.1)),
                    y: (min(ys.0, ys.1), max(ys.0, ys.1)),
                });    
            }
        }        
        Day14(lines)
    }
}

impl Solution for Day14 {
    fn part_one(&self) -> String {
        let mut grid = HashMap::new();
        let mut abyss = 0;

        for line in  self.0.iter() {
            for x in line.x.0..=line.x.1 {
                for y in line.y.0..=line.y.1 {
                    grid.insert((x, y), "#");
                    abyss = std::cmp::max(abyss, y+1);
                }
            }
        }

        let mut sx = 500;
        let mut sy = 0;
        
        let mut answer = 0;
        loop {
            if sy >= abyss {
                break;
            } 
            if !grid.contains_key(&(sx, sy+1)) {
                sy += 1;
            } else if !grid.contains_key(&(sx-1, sy+1)) {
                sy += 1;
                sx -= 1;
            } else if !grid.contains_key(&(sx+1, sy+1)) {
                sy += 1;
                sx += 1;
            } else {
                grid.insert((sx, sy), "o");
                answer += 1;
                sy = 0;
                sx = 500;
            } 
        }
        format!("{}", answer)
    }

    fn part_two(&self) -> String {
        let mut grid = HashMap::new();
        let mut abyss = 0;

        for line in  self.0.iter() {
            for x in line.x.0..=line.x.1 {
                for y in line.y.0..=line.y.1 {
                    grid.insert((x, y), "#");
                    abyss = max(abyss, y+2);
                }
            }
        }

        let mut sx = 500;
        let mut sy = 0;
        
        let mut answer = 0;
        loop {
            if sy == abyss {
                grid.insert((sx, sy), "o");
                sy = 0;
                sx = 500;
            } 
            if !grid.contains_key(&(sx, sy+1)) {
                sy += 1;
            } else if !grid.contains_key(&(sx-1, sy+1)) {
                sy += 1;
                sx -= 1;
            } else if !grid.contains_key(&(sx+1, sy+1)) {
                sy += 1;
                sx += 1;
            } else {
                grid.insert((sx, sy), "o");
                answer += 1;
                if sx == 500 && sy == 0 {
                    break;
                }
                sy = 0;
                sx = 500;
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
        let test_input = load_input(format!(".test_input/14.txt"));
        let day = Day14::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "24");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/14.txt"));
        let day = Day14::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "93");
    }
}
