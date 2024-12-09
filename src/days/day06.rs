use std::collections::{HashMap, HashSet, VecDeque};

use crate::error;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct Point(usize, usize);

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

enum GuardStatus {
    Active(Guard),
    OutOfBounds,
    Looping,
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
        match self.guard_location {
            Guard {
                point,
                direction: Direction::East,
            } => self
                .positions
                .get_key_value(&Point(point.0, point.1 + 1))
                .map(|(k, _)| *k),
            Guard {
                point,
                direction: Direction::South,
            } => self
                .positions
                .get_key_value(&Point(point.0 + 1, point.1))
                .map(|(k, _)| *k),
            Guard {
                point,
                direction: Direction::West,
            } => {
                if point.1 <= 0 {
                    None
                } else {
                    Some(Point(point.0, point.1.abs_diff(1)))
                }
            }
            Guard {
                point,
                direction: Direction::North,
            } => {
                if point.0 <= 0 {
                    None
                } else {
                    Some(Point(point.0.abs_diff(1), point.1))
                }
            }
        }
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

// struct LabGrid {
//     horizontal_lines: HashMap<usize, VecDeque<usize>>,
//     vertical_lines: HashMap<usize, VecDeque<usize>>,
//     moves: HashSet<Guard>,
// }

// fn parse(s: &str) -> (LabGrid, Guard) {
//     todo!()
// }

// impl LabGrid {
//     fn move_guard(&mut self, g: &Guard) -> GuardStatus {
//         let h = match g.direction {
//             Direction::East => self
//                 .horizontal_lines
//                 .get(&g.point.0)
//                 .unwrap()
//                 .iter()
//                 .find(|n| (*n < &g.point.1)),
//             Direction::West => self
//                 .horizontal_lines
//                 .get(&g.point.0)
//                 .unwrap()
//                 .iter()
//                 .rev()
//                 .find(|n| (*n > &g.point.1)),
//             _ => Some(&g.point.1),
//         };
//         let v = match g.direction {
//             Direction::North => self
//                 .horizontal_lines
//                 .get(&g.point.1)
//                 .unwrap()
//                 .iter()
//                 .rev()
//                 .find(|n| (*n > &g.point.0)),
//             Direction::South => self
//                 .horizontal_lines
//                 .get(&g.point.1)
//                 .unwrap()
//                 .iter()
//                 .find(|n| (*n < &g.point.0)),
//             _ => Some(&g.point.0),
//         };
//         let direction = g.direction.right_turn();
//         match (h, v) {
//             (None, _) => GuardStatus::OutOfBounds,
//             (_, None) => GuardStatus::OutOfBounds,
//             (Some(x), Some(y)) => {
//                 let guard = Guard {
//                     point: Point(*x, *y),
//                     direction,
//                 };
//                 if self.moves.contains(&guard) {
//                     GuardStatus::Looping
//                 } else {
//                     GuardStatus::Active(guard)
//                 }
//             }
//         }
//     }
// }
// pub fn part1_refac(input: String) -> Result<String, error::Error> {
//     let (mut lab, mut guard) = parse(&input);
//     while let GuardStatus::Active(g) = lab.move_guard(&guard) {
//         guard = g;
//     }
//     Ok(lab.moves.len().to_string())
// }

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
    while let NextSpace::Clear | NextSpace::Obstructed = lab.next_move() {
        if let NextSpace::Obstructed = lab.next_move() {
            lab.turn_guard()
        } else {
            lab.move_guard()
        }
    }

    let possible_new_obstructions = lab.positions.into_iter().flat_map(|(k, v)| match v {
        Space::Visited(_) => Some(k),
        _ => None,
    });

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
