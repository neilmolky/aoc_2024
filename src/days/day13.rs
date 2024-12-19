use crate::error;
use nom::{
    bytes::complete::tag,
    character::{
        complete::{digit1, multispace0, multispace1},
        streaming::alpha1,
    },
    combinator::map_res,
    multi::many1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn build_from<'a>(
        t: ((i64, i64), (i64, i64), (i64, i64)),
    ) -> Result<Machine, nom::Err<nom::error::Error<&'a str>>> {
        Ok(Machine {
            a: t.0,
            b: t.1,
            prize: t.2,
        })
    }
    fn cramers_method(&self, prize_add: i64) -> Option<i64> {
        // A = (p_x*b_y - prize_y*b_x) / (a_x*b_y - a_y*b_x)
        // B = (a_x*p_y - a_y*p_x) / (a_x*b_y - a_y*b_x)
        let prize = (prize_add + self.prize.0, prize_add + self.prize.1);
        let a_num = (prize.0 * self.b.1) - (prize.1 * self.b.0);
        let b_num = (self.a.0 * prize.1) - (self.a.1 * prize.0);
        let denom = (self.a.0 * self.b.1) - (self.a.1 * self.b.0);

        if (a_num % denom == 0) & (b_num % denom == 0) {
            let a = a_num / denom;
            let b = b_num / denom;
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

fn button(s: &str) -> IResult<&str, (i64, i64)> {
    delimited(
        delimited(tag("Button "), alpha1, tag(": ")),
        separated_pair(
            map_res(preceded(tag("X+"), digit1), str::parse),
            tag(", "),
            map_res(preceded(tag("Y+"), digit1), str::parse),
        ),
        multispace1,
    )(s)
}
fn prize(s: &str) -> IResult<&str, (i64, i64)> {
    delimited(
        tag("Prize: "),
        separated_pair(
            map_res(preceded(tag("X="), digit1), str::parse),
            tag(", "),
            map_res(preceded(tag("Y="), digit1), str::parse),
        ),
        multispace0,
    )(s)
}
fn machines(s: &str) -> Vec<Machine> {
    let machines = many1(map_res(tuple((button, button, prize)), Machine::build_from))(s);
    machines.ok().unwrap().1
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let machines = machines(&input);
    // dbg!(&machines);
    let result: i64 = machines.into_iter().flat_map(|m| m.cramers_method(0)).sum();
    Ok(result.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let machines = machines(&input);
    // dbg!(&machines);
    let result: i64 = machines
        .into_iter()
        .flat_map(|m| m.cramers_method(10000000000000))
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn test_part1() {
        let input = INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "480")
    }
    #[test]
    fn test_part2() {
        let input = INPUT.to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "875318608908")
    }
}
