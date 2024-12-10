use std::collections::{HashMap, HashSet, VecDeque};

use crate::error;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct Point(usize, usize);
impl Point {
    fn next_point(&self, d: Direction, positions: &HashMap<Point, Space>) -> Option<Point> {
        match (self, d) {
            (point, Direction::East) => positions
                .get_key_value(&Point(point.0, point.1 + 1))
                .map(|(k, _)| *k),
            (point, Direction::South) => positions
                .get_key_value(&Point(point.0 + 1, point.1))
                .map(|(k, _)| *k),
            (point, Direction::West) => if point.1 <= 0 {
                    None
                } else {
                    Some(Point(point.0, point.1.abs_diff(1)))
                }
            (point, Direction::North) => if point.0 <= 0 {
                    None
                } else {
                    Some(Point(point.0.abs_diff(1), point.1))
                }
            }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Space {
    Visited(HashMap<Direction, usize>),
    Unseen,
    Obstruction,
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum NextSpace {
    Clear,
    Obstructed,
    OutOfBounds,
    Loopy,
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct Guard {
    point: Point,
    direction: Direction,
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            '^' => Direction::North,
            x => panic!("unexpected input {}", x),
        }
    }
    fn right_turn(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
impl Space {
    fn from_char(c: char) -> Space {
        match c {
            '.' => Space::Unseen,
            '#' => Space::Obstruction,
            _ => Space::Visited(HashMap::from([(Direction::from_char(c), 1)])),
        }
    }
}

struct LabMap {
    positions: HashMap<Point, Space>,
    guard_location: Guard,
    turns: usize,
}
impl LabMap {
    fn from_input(input: &String) -> LabMap {
        let positions: HashMap<Point, Space> = input
            .lines()
            .enumerate()
            .flat_map(|(i, ln)| {
                ln.chars()
                    .enumerate()
                    .map(move |(j, c)| (Point(i, j), Space::from_char(c)))
            })
            .collect();

        let guard_location: Guard = positions
            .iter()
            .filter_map(|x| {
                if let (p, Space::Visited(m)) = x {
                    let d = m.keys().next().unwrap();
                    Some(Guard {
                        point: *p,
                        direction: *d,
                    })
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        LabMap {
            positions,
            guard_location,
            turns: 0,
        }
    }

    fn next_point(&self) -> Option<Point> {
        self.guard_location.point.next_point(self.guard_location.direction, &self.positions)
    }
    fn turn_guard(&mut self) {
        self.guard_location.direction = self.guard_location.direction.right_turn()
    }
    fn move_guard(&mut self) {
        let next_point = self.next_point().unwrap();
        self.positions.entry(next_point).and_modify(|x| match x {
            Space::Obstruction => panic!("guard can not move into obstructed position"),
            Space::Unseen => {
                *x = Space::Visited(HashMap::from([(self.guard_location.direction, 1)]))
            }
            Space::Visited(c) => {
                if let Some(_) = c.insert(self.guard_location.direction, 1) {
                    panic!("The guard has entered a loop")
                }
            }
        });
        self.guard_location.point = next_point;
        self.turns += 1;
    }

    fn place_obstruction(&mut self, p: Point) {
        self.positions.insert(p, Space::Obstruction);
    }

    fn next_move(&self) -> NextSpace {
        match self.next_point() {
            None => NextSpace::OutOfBounds,
            Some(p) => match self.positions.get(&p).unwrap() {
                Space::Obstruction => NextSpace::Obstructed,
                Space::Unseen => NextSpace::Clear,
                Space::Visited(c) => match c.get(&self.guard_location.direction) {
                    None => NextSpace::Clear,
                    Some(_) => NextSpace::Loopy,
                },
            },
        }
    }
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let mut lab = LabMap::from_input(&input);

    while let NextSpace::Clear | NextSpace::Obstructed = lab.next_move() {
        if let NextSpace::Obstructed = lab.next_move() {
            lab.turn_guard()
        } else {
            lab.move_guard()
        }
    }
    let seen_count = lab
        .positions
        .iter()
        .flat_map(|(_, s)| match s {
            Space::Visited(m) => Some(m),
            _ => None,
        })
        .count();

    Ok(seen_count.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let mut lab = LabMap::from_input(&input);
    let starting_guard = lab.guard_location.point.clone();
    while let NextSpace::Clear | NextSpace::Obstructed = lab.next_move() {
        if let NextSpace::Obstructed = lab.next_move() {
            lab.turn_guard()
        } else {
            lab.move_guard()
        }
    }

    let mut possible_new_obstructions: Vec<Point> = Vec::new();
    for (placement_point, first_pass_result) in &lab.positions {
        if let Space::Visited(place_dirs) = first_pass_result {
            possible_new_obstructions.push(*placement_point);
        }
    }
    dbg!(&possible_new_obstructions);
    

    let mut counter = 0;
    for p in possible_new_obstructions {
        lab = LabMap::from_input(&input);
        lab.place_obstruction(p);
        while let NextSpace::Clear | NextSpace::Obstructed = lab.next_move() {
            if let NextSpace::Obstructed = lab.next_move() {
                lab.turn_guard()
            } else {
                lab.move_guard()
            }
        }
        // dbg!(lab.next_move());
        if let NextSpace::Loopy = lab.next_move() {
            counter += 1;
        }
    }
    Ok(counter.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn test_part1() {
        let input = INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "41")
    }
    #[test]
    fn test_part2() {
        let input = INPUT.to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "6")
    }
}
