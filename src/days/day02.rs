use crate::error;

use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
#[derive(Debug, Copy, Clone)]
enum Order {
    Initialised(i32),
    Undefined,
    Increasing(i32),
    Decreasing(i32),
    Failed,
}

impl Order {
    fn next_order(&self, i: i32) -> Order {
        match self {
            Order::Undefined => Order::Initialised(i),
            Order::Initialised(n) => {
                if (n + 1..=n + 3).contains(&i) {
                    Order::Increasing(i)
                } else if (n - 3..=n - 1).contains(&i) {
                    Order::Decreasing(i)
                } else {
                    Order::Failed
                }
            }
            Order::Failed => Order::Failed,
            Order::Increasing(n) => {
                if (n + 1..=n + 3).contains(&i) {
                    Order::Increasing(i)
                } else {
                    Order::Failed
                }
            }
            Order::Decreasing(n) => {
                if (n - 3..=n - 1).contains(&i) {
                    Order::Decreasing(i)
                } else {
                    Order::Failed
                }
            }
        }
    }
}

fn numbers(s: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, map_res(digit1, str::parse::<i32>))(s)
}

fn passes(n: &Vec<i32>) -> bool {
    match n.iter().fold(Order::Undefined, |acc, i| acc.next_order(*i)) {
        Order::Failed => false,
        _ => true,
    }
}

fn any_passes(n: &Vec<i32>) -> bool {
    if passes(n) {
        true
    } else {
        (0..n.len())
            .into_iter()
            .map(|i| {
                n.iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != i)
                    .map(|(_, elem)| *elem)
                    .collect::<Vec<i32>>()
            })
            .any(|ns| passes(&ns))
    }
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let count: usize = input
        .lines()
        .map(|ln| numbers(ln).unwrap().1)
        .filter(passes)
        .count();
    Ok(count.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let count: usize = input
        .lines()
        .map(|ln| numbers(ln).unwrap().1)
        .filter(any_passes)
        .count();
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
8 9 7 6 5
8 9 9 8 7
6 8 7 6 5";
    #[test]
    fn test_part1() {
        let input = TEST_INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2")
    }
    #[test]
    fn test_part2() {
        let input = TEST_INPUT.to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "6")
    }
}
