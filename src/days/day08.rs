use std::collections::{HashMap, HashSet};

use crate::error;

enum Part {
    One,
    Two,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Point(i32, i32);

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
    fn diff(&self, other: &Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
    fn inlimit(&self, num_cols: usize, num_rows: usize) -> bool {
        (self.0 >= 0) & (self.0 < num_rows as i32) & (self.1 >= 0) & (self.1 < num_cols as i32)
    }
    fn resonant_points(&self, other: &Point, num_cols: usize, num_rows: usize) -> Vec<Point> {
        let (x, y) = (self.add(&self.diff(other)), other.add(&other.diff(self)));
        let mut v = Vec::new();
        if x.inlimit(num_cols, num_rows) {
            v.push(x)
        }
        if y.inlimit(num_cols, num_rows) {
            v.push(y)
        }
        v
    }
    fn resonant_lines<'a>(&self, other: &Point, num_rows: usize, num_cols: usize) -> Vec<Point> {
        let up_diff = self.diff(other);
        let mut up_resonant_point = self.add(&up_diff);
        let dn_diff = other.diff(&self);
        let mut dn_resonant_point = other.add(&dn_diff);
        let mut result_set: Vec<Point> = vec![self.clone(), other.clone()];
        while up_resonant_point.inlimit(num_cols, num_rows) {
            result_set.push(up_resonant_point.clone());
            up_resonant_point = up_resonant_point.add(&up_diff);
        }
        while dn_resonant_point.inlimit(num_cols, num_rows) {
            result_set.push(dn_resonant_point.clone());
            dn_resonant_point = dn_resonant_point.add(&dn_diff);
        }
        result_set
    }
}

fn recursively_find_resonances(
    completed: &mut Vec<Point>,
    remaining: &[Point],
    acc: HashSet<Point>,
    num_rows: usize,
    num_cols: usize,
    part: Part,
) -> HashSet<Point> {
    if remaining.is_empty() {
        acc
    } else {
        let p = remaining[0];
        let resonances: HashSet<Point> = completed
            .iter()
            .flat_map(|seen| match part {
                Part::One => seen.resonant_points(&p, num_cols, num_rows),
                Part::Two => seen.resonant_lines(&p, num_cols, num_rows),
            })
            .chain(acc.into_iter())
            .collect();
        completed.push(p);
        recursively_find_resonances(
            completed,
            &remaining[1..],
            resonances,
            num_rows,
            num_cols,
            part,
        )
    }
}

fn resonances(p: &Vec<Point>, num_rows: usize, num_cols: usize, part: Part) -> HashSet<Point> {
    let mut first_is_completed = vec![p[0]];
    let remaining = &p[1..];
    recursively_find_resonances(
        &mut first_is_completed,
        remaining,
        HashSet::new(),
        num_rows,
        num_cols,
        part,
    )
}

fn parse_input(input: &str) -> HashMap<char, Vec<Point>> {
    let mut antenna: HashMap<char, Vec<Point>> = HashMap::new();
    input
        .lines()
        .enumerate()
        .flat_map(|(i, x)| {
            x.chars().enumerate().flat_map(move |(j, c)| match c {
                '.' => None,
                x => Some((x, Point(i as i32, j as i32))),
            })
        })
        .for_each(|(c, p)| {
            antenna
                .entry(c)
                .and_modify(|v| {
                    v.push(p);
                })
                .or_insert(vec![p]);
        });
    antenna
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().len();
    let antenna = parse_input(&input);

    let result = antenna
        .values()
        .map(|x| resonances(x, num_rows, num_cols, Part::One))
        .reduce(|x, y| x.union(&y).copied().collect())
        .unwrap();

    Ok(result.len().to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().len();
    let antenna = parse_input(&input);

    let result = antenna
        .values()
        .map(|x| resonances(x, num_rows, num_cols, Part::Two))
        .reduce(|x, y| x.union(&y).copied().collect())
        .unwrap();

    Ok(result.iter().count().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.";

    fn sanitise_test_input() -> String {
        INPUT.replace("#", ".")
    }

    #[test]
    fn test_resonant_point() {
        let p1 = Point(1, 2);
        let p2 = Point(2, 4);
        let expected = p1.resonant_points(&p2, 10, 10);

        assert!(vec![Point(0, 0), Point(3, 6)] == expected, "{:?}", expected)
    }

    #[test]
    fn test_part1() {
        let input = sanitise_test_input();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "14")
    }
    #[test]
    fn test_part2() {
        let input = sanitise_test_input();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "34")
    }
}
