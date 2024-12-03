use crate::error;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::{anychar, digit1}, combinator::{map, map_res}, multi::{many1, many_till}, sequence::{delimited, separated_pair}, IResult
};
#[derive(Debug, Copy, Clone)]
enum UncorruptedMemory {
    Mul(i32, i32),
    On(i32),
    Off(i32)
}

impl UncorruptedMemory {
    fn build_switches(s: &str) -> Result<UncorruptedMemory, nom::Err<&str>> {match s {
        "do()" => Ok(UncorruptedMemory::On(0)),
        "don't()" => Ok(UncorruptedMemory::Off(0)),
        _ => panic!("filter beffore build")
    }
}
    fn build_mul(p: (i32, i32))  -> Result<UncorruptedMemory, nom::Err<&'static str>> {
        Ok(UncorruptedMemory::Mul(p.0, p.1))
    }
    fn to_string(&self) -> String { match *self {
        UncorruptedMemory::Mul(i, j) => (i * j).to_string(),
        UncorruptedMemory::On(i) => i.to_string(),
        UncorruptedMemory::Off(i) => i.to_string()
    }
    }
}

fn filtered_inputs(s: &str) -> IResult<&str, Vec<UncorruptedMemory>> {
    let mut multiplication_parser = 
        delimited(
            tag("mul("), 
            map_res(separated_pair(
                map_res(digit1, str::parse::<i32>), 
                tag(","), 
                map_res(digit1, str::parse::<i32>)), UncorruptedMemory::build_mul),
            tag(")")
    );
    let filter_parser = alt((
        multiplication_parser, 
        map_res(tag("do()"), UncorruptedMemory::build_switches), 
        map_res(tag("don't()"), UncorruptedMemory::build_switches)
    ));
    many1(map(many_till(anychar, filter_parser), |(_, r)| r))(s)
}
fn inputs(s: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let mut multiplication_parser = 
        delimited(
            tag("mul("), 
            separated_pair(
                map_res(digit1, str::parse::<i32>), 
                tag(","), 
                map_res(digit1, str::parse::<i32>)),
            tag(")")
    );
    many1(map(many_till(anychar, multiplication_parser), |(_, r)| r))(s)
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let result: i32 = inputs(&input).unwrap().1
        .iter()
        .map(|(x, y)| x * y)
        .sum();
    Ok(result.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let result = filtered_inputs(input.as_str()).unwrap().1
        .iter()
        .fold(UncorruptedMemory::On(0), |acc, n| match (acc, *n) {
            (UncorruptedMemory::On(i), UncorruptedMemory::Off(_)) => UncorruptedMemory::Off(i),
            (UncorruptedMemory::On(i), UncorruptedMemory::Mul(x, y)) => UncorruptedMemory::On(i + (x * y)),
            (UncorruptedMemory::Off(i), UncorruptedMemory::On(_)) => UncorruptedMemory::On(i),
            (UncorruptedMemory::Mul(_, _), _) => panic!("accumulator should only be On or Off: {:?}", &acc),
            (x, _) => x
        });
    Ok(result.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "161")

    }
    
    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "48")

    }
}