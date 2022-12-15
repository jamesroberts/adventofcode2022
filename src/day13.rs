use std::cmp::Ordering;

use crate::utils::{FromInput, Solution};

use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list0,
    sequence::delimited, IResult,
};


#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn as_slice(&self) -> &[Packet] {
        match self {
            Packet::Int(_) => std::slice::from_ref(self),
            Packet::List(list) => list.as_slice(),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (_, _) => self.as_slice().cmp(other.as_slice()),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u64, |n| n as usize)(input)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(parse_usize, Packet::Int),
        map(delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")), Packet::List),
    ))(input)
}

pub struct Day13(Vec<Packet>);

impl FromInput for Day13 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let lines: Vec<String> = input.collect();
        let mut i = lines.iter();
        let mut packets = Vec::new();

        loop {
            let packet1 = i.next().unwrap();
            let packet2 = i.next().unwrap();
           
            let a = parse_packet(packet1.trim()).unwrap();
            let b = parse_packet(packet2.trim()).unwrap();
            
            packets.push(a.1);
            packets.push(b.1);

            if i.next() == None {
                break;
            };
        }
        Day13(packets)
    }
}

impl Solution for Day13 {
    fn part_one(&self) -> String {
        let mut answer = 0;
        let mut pair = 0;
        for i in (1..self.0.len()).step_by(2) {
            pair += 1;
            if self.0[i-1] < self.0[i] {
                answer += pair;
            } 
        }
        format!("{}", answer)
    }

    fn part_two(&self) -> String {
        let mut answer = 1;
        let mut packets = self.0.clone();

        let x = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
        let y = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

        packets.push(x.clone());
        packets.push(y.clone());

        packets.sort();
        answer *= packets.iter().position(|i| i == &x).unwrap() + 1;
        answer *= packets.iter().position(|i| i == &y).unwrap() + 1;

        format!("{}", answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/13.txt"));
        let day = Day13::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/13.txt"));
        let day = Day13::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "140");
    }
}
