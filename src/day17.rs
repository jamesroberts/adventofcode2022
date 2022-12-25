use std::collections::{HashSet, HashMap};

use crate::utils::{FromInput, Solution};

pub struct Day17 {
    rocks: Vec<Vec<(i64, i64)>>,
    jets: Vec<i64>
}

impl FromInput for Day17 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let rocks = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        ];
        let mut jets = Vec::new();
        for line in input {
            for c in line.chars() {
                let dir = match c {
                    '<' => -1,
                    '>' => 1,
                    _ => panic!("Invalid jet symbol")
                };
                jets.push(dir);
            }
        }
        Day17 {rocks, jets}
    }
}

impl Solution for Day17 {
    fn part_one(&self) -> String {
        let mut stopped: HashSet<(i64, i64)> = HashSet::new();
        for x in 0..7 {
            stopped.insert((x, -1));
        }
        
        let mut height = 0;
        let mut count = 0;
        let mut index = 0;
        
        let mut rock: HashSet<(i64, i64)> = HashSet::new();
        let cur_rock = self.rocks.get(index).unwrap();
        for (x, y) in cur_rock.iter() {
            rock.insert((x + 2, y + height + 3));
        }

        while count < 2022 {
            for jet in self.jets.iter() {
                let mut moved_rock = HashSet::new();

                // Apply jets to rock
                for (x, y) in rock.iter() {
                    moved_rock.insert((x + jet, *y));
                } 
                if moved_rock.iter().all(|&(x, _)| x >= 0 && x < 7) {
                    if moved_rock.intersection(&stopped).count() == 0 {
                        rock = moved_rock;
                    }
                }

                // Move rock down 
                moved_rock = HashSet::new();
                for (x, y) in rock.iter() {
                    moved_rock.insert((*x, y - 1));
                }

                if moved_rock.intersection(&stopped).count() == 0 {
                    rock = moved_rock;
                } else {
                    stopped.extend(rock.iter());
                    count += 1;
                    height = stopped.iter().map(|(_, y)| y).max().unwrap() + 1;
                    if count >= 2022 {
                        break;
                    }
                    index = (index + 1) % self.rocks.len();

                    // New rock
                    rock = HashSet::new();
                    let cur_rock = self.rocks.get(index).unwrap();
                    for (x, y) in cur_rock.iter() {
                        rock.insert((x + 2, y + height + 3));
                    }
                }
            } 
        }

        format!("{}", height)
    }

    fn part_two(&self) -> String {
        let mut stopped: HashSet<(i64, i64)> = HashSet::new();
        for x in 0..7 {
            stopped.insert((x, -1));
        }
        
        let mut height = 0;
        let mut count = 0;
        let mut offset = 0;
        let mut rock_index = 0;
        
        let mut rock: HashSet<(i64, i64)> = HashSet::new();
        let cur_rock = self.rocks.get(rock_index).unwrap();
        for (x, y) in cur_rock.iter() {
            rock.insert((x + 2, y + height + 3));
        }
        let mut seen = HashMap::new();
        let target_count: i64 = 1000000000000;

        fn height_map(stopped: &HashSet<(i64, i64)>) -> Vec<i64> {
            let mut heights = vec![0; 7];
            // Find tallest y in each x pos
            for (x, y) in stopped.iter() {
                if y > &heights[*x as usize] {
                    heights[*x as usize] = *y;
                }
            }
            // Clamp heights relative to higest value
            let m = heights.iter().max().unwrap();
            heights.iter().map(|&x| x - m).collect()
        }

        while count < target_count {
            for (jet_index, jet) in self.jets.iter().enumerate() {
                let mut moved_rock = HashSet::new();

                // Apply jets to rock
                for (x, y) in rock.iter() {
                    moved_rock.insert((x + jet, *y));
                } 
                if moved_rock.iter().all(|&(x, _)| x >= 0 && x < 7) {
                    if moved_rock.intersection(&stopped).count() == 0 {
                        rock = moved_rock;
                    }
                }

                // Move rock down 
                moved_rock = HashSet::new();
                for (x, y) in rock.iter() {
                    moved_rock.insert((*x, y - 1));
                }

                if moved_rock.intersection(&stopped).count() == 0 {
                    rock = moved_rock;
                } else {
                    stopped.extend(rock.iter());
                    count += 1;
                    height = stopped.iter().map(|(_, y)| y).max().unwrap() + 1;
                    if count >= target_count {
                        break;
                    }
                    rock_index = (rock_index + 1) % self.rocks.len();

                    // New rock
                    rock = HashSet::new();
                    let cur_rock = self.rocks.get(rock_index).unwrap();
                    for (x, y) in cur_rock.iter() {
                        rock.insert((x + 2, y + height + 3));
                    }

                    let key = (jet_index, rock_index, height_map(&stopped));
                    if seen.contains_key(&key) {
                        let (rc, h) = seen.get(&key).unwrap();
                        let remaining = target_count - count;
                        let repeat = remaining / (count - rc);
                        offset = repeat * (height - h);
                        count = count + (repeat * (count - rc));
                        seen = HashMap::new();
                    }
                    seen.insert(key, (count, height));
                }
            } 
        }
        format!("{}", height + offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/17.txt"));
        let day = Day17::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "3068");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/17.txt"));
        let day = Day17::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "1514285714288");
    }
}
