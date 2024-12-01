use std::collections::HashMap;

use crate::error;
use nom::{
    character::complete::{digit1, space1}, combinator::map_res, sequence::separated_pair, IResult
};

fn number_pair(s: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(
        map_res(digit1, str::parse), 
        space1, 
        map_res(digit1, str::parse))(s)
}


pub fn part1(input: String) -> Result<String, error::Error> {
    let (mut vec1, mut vec2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(number_pair)
        .map(|r| r.unwrap().1)
        .unzip();
    vec1.sort();
    vec2.sort();
    let result: u32 = vec1
        .iter()
        .zip(vec2)
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    Ok(result.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let (vec1, vec2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(number_pair)
        .map(|r| r.unwrap().1)
        .unzip();
    let counter = vec2
        .iter()
        .fold(HashMap::new(), |mut acc: HashMap<u32, u32>, i| {
            acc
                .entry(*i)
                .and_modify(|count| *count += 1 )
                .or_insert(1);
            acc
        });
    let result: u32 = vec1
        .iter()
        .map(|a| counter.get(a).get_or_insert(&0).to_owned() * a)
        .sum();
    Ok(result.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    #[test]
    fn test_part1() {
        let input = TEST_INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "11")
}
    #[test]
    fn test_part2() {
        let input = TEST_INPUT.to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "31")
    }
}