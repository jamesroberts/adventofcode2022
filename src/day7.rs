use crate::utils::{FromInput, Solution};

pub struct Day7(Dir);

impl FromInput for Day7 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let lines: Vec<String> = input.map(|x| x.clone()).collect();
        Day7(Dir::new(&mut lines.iter().map(AsRef::as_ref)))
    }
}

struct Dir {
    size: usize,
    sub_dirs: Vec<Dir>,
}

impl Dir {
    fn new<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Dir {
        let mut size = 0;
        let mut sub_dirs = Vec::new();

        while let Some(line) = lines.next() {
            let skipable =["$ cd /", "dir", "$ ls"];
            if skipable.iter().any(|s| line.starts_with(s)) {
                continue;
            } else if line == "$ cd .." {
                break;
            }
            
            if let Ok(filesize) = line.split_once(' ').unwrap().0.parse::<usize>() {
                size += filesize;
            } else {
                let new_dir = Self::new(lines);
                size += new_dir.size;
                sub_dirs.push(new_dir);
            }
        }
        Dir { size, sub_dirs }
    }

    fn recurse(&self) -> Box<dyn Iterator<Item = &Self> + '_> {
        Box::new(std::iter::once(self).chain(self.sub_dirs.iter().flat_map(Self::recurse)))
    }
}

impl Solution for Day7 {
    fn part_one(&self) -> String {
        let sizes: Vec<usize> = self.0.recurse().map(|dir| dir.size).collect();
        let answer = sizes.iter().filter(|&&size| size <= 100000).sum::<usize>();
        format!("{}", answer)
    }

    fn part_two(&self) -> String {
        let sizes: Vec<usize> = self.0.recurse().map(|dir| dir.size).collect();
        let required = 30000000 - (70000000 - self.0.size);
        let answer = sizes.iter().filter(|&&size| size >= required).min().unwrap();
        format!("{}", answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/7.txt"));
        let day = Day7::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "95437");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/7.txt"));
        let day = Day7::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "24933642");
    }
}
