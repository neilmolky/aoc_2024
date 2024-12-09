use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{fmt::Debug, hash::Hash};

use crate::error;

struct Calibration(u64, Vec<u64>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Opperation {
    Add,
    Multiply,
    Concatenation,
}

struct OperationNode {
    pub value: u64,
}

impl OperationNode {
    fn new(value: u64) -> OperationNode {
        OperationNode { value }
    }

    fn result(&self, i: &u64, o: &Opperation) -> OperationNode {
        let value = match o {
            Opperation::Add => self.value + i,
            Opperation::Multiply => self.value * i,
            Opperation::Concatenation => {
                let mut s = self.value.to_string();
                s.push_str(i.to_string().as_str());
                s.parse().unwrap()
            }
        };
        OperationNode { value }
    }
}

impl Calibration {
    pub fn try_build(pair: (u64, Vec<u64>)) -> Result<Calibration, &'static str> {
        Ok(Calibration(pair.0, pair.1))
    }

    fn recursive_possible_solution<'a>(
        parent: &'a mut OperationNode,
        v: &[u64],
        op: &[Opperation],
        result: u64,
    ) -> Option<u64> {
        if v.is_empty() & (parent.value == result) {
            Some(parent.value)
        } else if v.is_empty() | (parent.value > result) {
            None
        } else {
            op.iter()
                .flat_map(|o| {
                    let mut child = parent.result(&v[0], o);
                    Calibration::recursive_possible_solution(&mut child, &v[1..], op, result)
                })
                .next()
        }
    }

    pub fn possible_solution(&self, for_opperators: &[Opperation]) -> Option<u64> {
        let mut parent = OperationNode::new(*&self.1[0]);
        Calibration::recursive_possible_solution(&mut parent, &self.1[..], for_opperators, self.0)
    }
}

fn calibrations(s: &str) -> IResult<&str, Calibration> {
    map_res(
        separated_pair(map_res(digit1, str::parse), tag(": "), numbers),
        Calibration::try_build,
    )(s)
}

fn numbers(s: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, map_res(digit1, str::parse::<u64>))(s)
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let operations = [Opperation::Add, Opperation::Multiply];
    let result: u64 = input
        .lines()
        .map(calibrations)
        .map(|r| r.unwrap().1)
        .filter_map(|c| c.possible_solution(&operations))
        .sum();
    Ok(result.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let operations = [
        Opperation::Add,
        Opperation::Multiply,
        Opperation::Concatenation,
    ];
    let result: u64 = input
        .lines()
        .map(calibrations)
        .map(|r| r.unwrap().1)
        .flat_map(|c| c.possible_solution(&operations))
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn test_part1() {
        let input = INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "3749")
    }
    #[test]
    fn test_part2() {
        let input = INPUT.to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "11387")
    }
}
