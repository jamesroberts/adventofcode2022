use crate::utils::{FromInput, Solution};
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::tuple,
    IResult,
};
use std::cmp::{max, min};

#[derive(Debug)]
pub struct Day16 {
    names: Vec<String>,
    flows: Vec<usize>,
    distances: Vec<Vec<usize>>,
}

fn parse_valve(input: &str) -> IResult<&str, String> {
    map(nom::character::complete::alpha0, |s: &str| s.to_string())(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u64, |n| n as usize)(input)
}

fn parse_line(input: &str) -> IResult<&str, (String, usize, Vec<String>)> {
    map(
        tuple((
            tag("Valve "),
            parse_valve,
            tag(" has flow rate="),
            parse_usize,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), parse_valve),
        )),
        |(_, name, _, fr, _, ts)| (name, fr, ts),
    )(input)
}

fn build_distance_matrix(names: &Vec<String>, tunnels: &Vec<Vec<String>>) -> Vec<Vec<usize>> {
    let n = names.len();
    let mut distances = Vec::new();

    for r in 0..n {
        let mut row = Vec::new();
        for c in 0..n {
            if tunnels[r].contains(&names[c]) {
                row.push(1);
            } else {
                row.push(99);
            }
        }
        distances.push(row);
    }
    distances
}

fn floyd_warshall(names: &Vec<String>, tunnels: &Vec<Vec<String>>) -> Vec<Vec<usize>> {
    let n = names.len();
    let mut distances = build_distance_matrix(&names, &tunnels);

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                distances[i][j] = min(distances[i][j], distances[i][k] + distances[k][j])
            }
        }
    }
    distances
}

impl FromInput for Day16 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut names = Vec::new();
        let mut flows = Vec::new();
        let mut tunnels = Vec::new();

        for line in input {
            let (name, flow, tunnel_list) = parse_line(line.as_str()).unwrap().1;
            names.push(name);
            flows.push(flow);
            tunnels.push(tunnel_list);
        }
        let distances = floyd_warshall(&names, &tunnels);
        Day16 {
            names,
            flows,
            distances,
        }
    }
}

fn take_one2(choices: &Vec<usize>) -> Vec<(usize, Vec<usize>)> {
    let mut iter = Vec::new();

    for i in 0..choices.len() {
        let mut rest = choices.clone();
        let choice = rest.remove(i);
        iter.push((choice, rest));
    }
    iter
}

fn dfs(cur: &usize, choices: &Vec<usize>, time: &usize, distances: &Vec<Vec<usize>>, flows: &Vec<usize>) -> usize {
    let mut pressure: Vec<usize> = vec![0];

    for (choice, rest) in take_one2(choices).iter() {
        let distance = distances[*cur][*choice];
        if distance < *time {
            let nt = time - distance - 1;
            pressure.push((flows[*choice] * nt) + dfs(choice, rest, &nt, distances, flows));
        }
    }
    return *pressure.iter().max().unwrap();
}

fn dfs2(cur: &usize, choices: &Vec<usize>, time: &usize, distances: &Vec<Vec<usize>>, flows: &Vec<usize>) -> usize {
    let mut pressure: Vec<usize> = vec![0];

    for (choice, rest) in take_one2(choices).iter() {
        let distance = distances[*cur][*choice];
        if distance < *time {
            let nt = time - distance - 1;
            pressure.push(flows[*choice] * nt + dfs2(choice, rest, &nt, distances, flows));
        }
    }
    let elephant = dfs(&0usize, choices, &26usize, distances, flows);
    max(*pressure.iter().max().unwrap(), elephant)
}

impl Solution for Day16 {
    fn part_one(&self) -> String {
        let start = self.names.iter().position(|n| n == "AA").unwrap();
        let choices = self.flows.iter().enumerate().filter(|(_, &f)| f > 0).map(|(i, _)| i).collect();
        format!("{}", dfs(&start, &choices, &30usize, &self.distances, &self.flows))
    }

    fn part_two(&self) -> String {
        let start = self.names.iter().position(|n| n == "AA").unwrap();
        let choices = self.flows.iter().enumerate().filter(|(_, &f)| f > 0).map(|(i, _)| i).collect();
        format!("{}", dfs2(&start, &choices, &26usize, &self.distances, &self.flows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/16.txt"));
        let day = Day16::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "1651");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/16.txt"));
        let day = Day16::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "1707");
    }
}
