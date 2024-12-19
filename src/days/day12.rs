use std::collections::{HashMap, HashSet, VecDeque};

use array2d::Array2D;
use itertools::Itertools;

use crate::error;
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North(i32, i32), 
    South(i32, i32), 
    West(i32, i32), 
    East(i32, i32),
}
impl Direction {
    fn neighbors(pos: &(usize, usize)) -> Vec<Direction> {
        let ipos = (pos.0 as i32, pos.1 as i32);
        vec![
            Direction::North(ipos.0-1, ipos.1),
            Direction::South(ipos.0+1, ipos.1),
            Direction::West(ipos.0, ipos.1+1),
            Direction::East(ipos.0, ipos.1-1)
        ]
    }
    fn pos(&self) -> Option<(usize, usize)> {     
        match self {
            Direction::North(row, col) if (row >= &0) & (col >= &0) => Some((*row as usize, *col as usize)),
            Direction::South(row, col) if (row >= &0) & (col >= &0) => Some((*row as usize, *col as usize)),
            Direction::West(row, col)  if (row >= &0) & (col >= &0) => Some((*row as usize, *col as usize)),
            Direction::East(row, col)  if (row >= &0) & (col >= &0) => Some((*row as usize, *col as usize)),
            _ => None
        }
    }
    fn arr_elem<'a>(&self, arr: &'a Array2D<char>) -> Option<&'a char> {
        match self.pos() {
            Some((row, col)) => arr.get(row, col),
            _ => None
        }
    }
    fn corners(&self, other: &Direction) -> usize {
        match (self, other) {
            (Direction::North(..), Direction::West(..)) => 1,
            (Direction::North(..), Direction::East(..)) => 1,
            (Direction::East(..), Direction::North(..)) => 1,
            (Direction::East(..), Direction::South(..)) => 1,
            (Direction::South(..), Direction::East(..)) => 1,
            (Direction::South(..), Direction::West(..)) => 1,
            (Direction::West(..), Direction::North(..)) => 1,
            (Direction::West(..), Direction::South(..)) => 1,
            _ => 0
        }
    }
    fn corner_from(d1: &Direction, d2: &Direction) -> Option<(usize, usize)> {
        match (d1, d2) {
            (Direction::North(r1, c1), Direction::West(r2, c2)) | (Direction::West(r1, c1), Direction::North(r2, c2)) if (r1 == r2) & (c1 == c2) & (*c1 > 0) => {
                Some(((r1 + 1) as usize, (c1 - 1) as usize))
            },
            (Direction::North(r1, c1), Direction::East(r2, c2)) | (Direction::East(r1, c1), Direction::North(r2, c2)) if (r1 == r2) & (c1 == c2) => {
                Some(((r1 + 1) as usize, (c1 + 1) as usize))
            },
            (Direction::East(r1, c1), Direction::South(r2, c2)) | (Direction::South(r1, c1), Direction::East(r2, c2)) if (r1 == r2) & (c1 == c2) & (*r1 > 0) => {
                Some(((r1 - 1) as usize, (c1 + 1) as usize))
            },
            (Direction::South(r1, c1), Direction::West(r2, c2)) | (Direction::West(r1, c1), Direction::South(r2, c2)) if (r1 == r2) & (c1 == c2) & (*r1 > 0) & (*c1 > 0) => {
                Some(((r1 - 1) as usize, (c1 - 1) as usize))
            },
            _ => None
        }

    }
}

fn neighbors(pos: &(usize, usize)) -> Vec<(usize, usize)> {
    match pos {
        (0, 0) => vec![(0, 1), (1, 0)],
        (0, col) => vec![(0, col+1), (0, col-1), (1, *col)],
        (row, 0) => vec![(row+1, 0), (row-1, 0), (*row, 1)],
        (row, col) => vec![(row+1, *col), (row-1, *col), (*row, col+1), (*row, col-1)]
    }
}

fn find_perimeter(arr: &Array2D<char>, pos: &(usize, usize), c: &char) -> (HashSet<(usize, usize)>, usize) {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut area: usize = 0;
    let mut perimeter: usize = 0;
    let mut discover: VecDeque<(usize, usize)> = VecDeque::from([*pos]);
    while let Some(square) = discover.pop_front() {
        if !seen.contains(&square) {
            seen.insert(square);
            perimeter += match neighbors(&square)
                .into_iter()
                .filter(|p| if let Some(elem) = arr.get(p.0, p.1) {elem == c} else {false})
                .fold(0, |acc, val| {
                if !seen.contains(&val) {
                    discover.push_back(val.clone());
                }
                acc+1
            }) {
                0 => 4,
                1 => 3,
                2 => 2,
                3 => 1,
                4 => 0,
                _ => panic!("more than 4 neighbors makes no sense")
            };
            area += 1;

        }
    }
    (seen, area * perimeter)
}

/// Concept
/// 
/// each square has the following view of neighbors
///    [ ]    
/// [ ] x [ ] 
///    [ ]    
/// 
/// 
/// 1 corner  2 corners  4 corners
///  c [ ]     c [ ]     c [ ] c   
/// [ ] x [x] [ ] x [x] [ ] x [ ]
///    [x]     c [ ]     c [ ] c   
/// 
/// if external each square is pointed to by some internal square
///   hole      edge       edge       nook      corner
///  x [v] x   x [v] x    x [v] x    x [v] x   x [v] x  
/// [>] 4 [<] [ ] 0 [ ]  [ ] 0 [ ]  [>] 2 [ ] [>] 1 [ ] 
///  x [^] x   x [^] x    x [ ] x    x [^] x   x [ ] x  
/// 
/// hole nook and corner would not be picked up by the view from the internal square alone
/// note that if x is not actually present in the internal squares the hole nook and corners would be invalid
/// as such checks for the position in seen are also required
/// 
///   x [ ] x    x [v] x      [v]      x [ ] x  
///  [>] 0 [ ]  [>] 1 [ ]  [>] 0 [<]  [>] 0 [ ] 
///     [^] x      [^] x      [^]        [^] x  
fn find_sides(arr: &Array2D<char>, pos: &(usize, usize), c: &char) -> (HashSet<(usize, usize)>, usize) {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut shared_external_squares: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
    let mut internal_corners: HashMap<(usize, usize), usize> = HashMap::new();
    let mut area: usize = 0;
    let mut discover: VecDeque<(usize, usize)> = VecDeque::from([*pos]);
    while let Some(square) = discover.pop_front() {
        if !seen.contains(&square) {
            seen.insert(square);
            let mut inside: HashSet<Direction> = HashSet::new();
            for n in Direction::neighbors(&square) {
                match n.arr_elem(arr) {
                    Some(found) if found == c => {
                        discover.push_back(n.pos().unwrap());
                        inside.insert(n.clone());
                    },
                    Some(_) =>  {
                        shared_external_squares
                            .entry(n.pos().unwrap())
                            .and_modify(|v| v.push(n.clone()))
                            .or_insert(vec![n]);
                    },
                    None => ()
                }
            }
            internal_corners.insert(square, match inside.len() {
                0 => 4,
                1 => 2,
                2 => if let Some((x, y)) = inside.iter().collect_tuple() {
                    x.corners(y)
                } else {0},
                _ => 0
            });
            area += 1;
        }
    }
    let num_corners = shared_external_squares
        .iter()
        .map(|(_center_square, from_squares)| {
            from_squares.into_iter()
                .fold(HashSet::<(usize, usize)>::new(), |mut acc, x| {
                    from_squares
                        .iter()
                        .flat_map(|d| Direction::corner_from(d, x))
                        .filter(|p| seen.contains(p))
                        .for_each(|p| {acc.insert(p);});
                    acc
                }).len()
            }
        )
        .inspect(|x| println!("external corners + {x}"))
        .chain(internal_corners.iter().map(|(_, v)|*v))
        .sum::<usize>();
    (seen, area * num_corners)
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let arr = Array2D::from_rows(
        &input
            .lines()
            .map(|ln| ln.chars().collect_vec())
            .collect_vec(),
    ).unwrap();
    let mut total: usize = 0;
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for (pos, c) in arr.enumerate_row_major() {
        if !seen.contains(&pos) {
            let (squares, price) = find_perimeter(&arr, &pos, &c);
            squares.into_iter().for_each(|s| {assert!(seen.insert(s));});
            total += price;
        }
    }
    Ok(total.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let arr = Array2D::from_rows(
        &input
            .lines()
            .map(|ln| ln.chars().collect_vec())
            .collect_vec(),
    ).unwrap();
    let mut total: usize = 0;
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for (pos, c) in arr.enumerate_row_major() {
        if !seen.contains(&pos) {
            let (squares, price) = find_sides(&arr, &pos, &c);
            squares.into_iter().for_each(|s| {assert!(seen.insert(s));});
            total += price;
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";


    #[test]
    fn test_part1() {
        let input = INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1930")
    }

    #[test]
    fn test_part2() {
        let input = INPUT.to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1206")
    }

    #[test]
    fn test_part2_e() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE".to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "236")
    }
    #[test]
    fn test_part2_2_internal() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA".to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "368")
    }
}
