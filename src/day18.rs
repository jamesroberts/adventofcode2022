use std::{cmp::{min, max}, collections::{VecDeque, HashSet}, isize::MAX};

use crate::utils::{FromInput, Solution};

pub struct Day18(Vec<(isize, isize, isize)>);

impl FromInput for Day18 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut points = Vec::new();
        for line in input {
            let entry: Vec<isize> = line.split(",").map(|c| c.parse().unwrap()).collect();
            points.push((entry[0], entry[1], entry[2]));
        }
        Day18(points)
    }
}

impl Solution for Day18 {
    fn part_one(&self) -> String {
        let points = &self.0;
        let mut count = 0;
        
        for (x, y, z) in points.iter() {
            for (xn, yn, zn) in points.iter() {
                if x == xn && y == yn && z == zn {
                    continue;
                } 
                if (x-xn).abs() == 1 && y == yn && z == zn {
                    count += 1;
                }
                if (y-yn).abs() == 1 && x == xn && z == zn {
                    count += 1;
                }
                if (z-zn).abs() == 1 && y == yn && x == xn {
                    count += 1;
                }
            }
        }

        format!("{}", 6 * points.len() - count)
    }

    fn part_two(&self) -> String {
        let mut points: HashSet<(isize, isize, isize)> = HashSet::new();

        let [mut minx, mut miny, mut minz] = [MAX, MAX, MAX];
        let [mut maxx, mut maxy, mut maxz] = [0, 0, 0];

        for p in self.0.iter() {
            points.insert(*p); 
            minx = min(minx, p.0); 
            miny = min(miny, p.1); 
            minz = min(minz, p.2); 

            maxx = max(maxx, p.0); 
            maxy = max(maxy, p.1); 
            maxz = max(maxz, p.2); 
        }

        let mut faces = 0;
        let mut queue = VecDeque::new();
        let mut air = HashSet::new();
        let dirs = [(1,0,0), (-1,0,0), (0,1,0), (0,-1,0), (0,0,1), (0,0,-1)];

        queue.push_back((minx, miny, minz));
        air.insert((minx, miny, minz));
    
        while !queue.is_empty() {
            let (x, y, z) = queue.pop_front().unwrap();

            for (dx, dy, dz) in dirs.iter() {
                let nx = x + dx;
                let ny = y + dy;
                let nz = z + dz;
                
                if !(nx >= minx-1 && nx <= maxx+1) { continue; }
                if !(ny >= miny-1 && ny <= maxy+1) { continue; }
                if !(nz >= minz-1 && nz <= maxz+1) { continue; }

                let k = (nx, ny, nz);

                if points.contains(&k) {
                    faces += 1;
                    continue;
                } 

                if air.contains(&k) {
                    continue;
                }

                air.insert(k);
                queue.push_back(k);
            }
        }
        format!("{}", faces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/18.txt"));
        let day = Day18::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "64");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/18.txt"));
        let day = Day18::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "58");
    }
}
