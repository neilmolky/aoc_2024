use memoize::memoize;

use crate::error;

use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
fn numbers(s: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, map_res(digit1, str::parse::<usize>))(s)
}

#[memoize]
fn get_length(number: usize, depth: usize) -> usize {
    if depth == 0 {
        1
    } else {
        match number.to_string().as_str() {
            "0" => get_length(1, depth - 1),
            x if x.len() % 2 == 0 => {
                let first = x[0..x.len() / 2].parse().unwrap();
                let second = x[x.len() / 2..x.len()].parse().unwrap();
                // println!("{},{} = {}", &first, &second, number);
                get_length(first, depth - 1) + get_length(second, depth - 1)
            }
            _ => get_length(number * 2024, depth - 1),
        }
    }
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let pebbles = numbers(&input).ok().unwrap().1;
    let result: usize = pebbles.into_iter().map(|p| get_length(p, 25)).sum();

    Ok(result.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let pebbles = numbers(&input).ok().unwrap().1;

    let result: usize = pebbles.into_iter().map(|p| get_length(p, 75)).sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "125 17";
    #[test]
    fn test_part1() {
        let input = INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "55312")
    }
}
