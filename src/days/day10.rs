use std::collections::HashSet;

use crate::error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Topogaphy {
    row_num: i32,
    col_num: i32,
    height: i32,
}

impl Topogaphy {
    fn build(i: usize, j: usize, c: char) -> Topogaphy {
        Topogaphy {
            row_num: i as i32,
            col_num: j as i32,
            height: c.to_digit(10).unwrap() as i32,
        }
    }
    fn ascending_neighbors(&self) -> Vec<Topogaphy> {
        vec![
            Topogaphy {
                row_num: self.row_num + 1,
                col_num: self.col_num,
                height: self.height + 1,
            },
            Topogaphy {
                row_num: self.row_num - 1,
                col_num: self.col_num,
                height: self.height + 1,
            },
            Topogaphy {
                row_num: self.row_num,
                col_num: self.col_num + 1,
                height: self.height + 1,
            },
            Topogaphy {
                row_num: self.row_num,
                col_num: self.col_num - 1,
                height: self.height + 1,
            },
        ]
    }
}

struct LavaProductionFacility {
    map: HashSet<Topogaphy>,
    starting_positions: Vec<Topogaphy>,
}

impl LavaProductionFacility {
    fn from_input(input: &str) -> LavaProductionFacility {
        let mut starting_positions: Vec<Topogaphy> = Vec::new();
        // let successful_paths: HashSet<[Topogaphy; 10]> = HashSet::new();
        let map: HashSet<Topogaphy> = input
            .lines()
            .enumerate()
            .flat_map(|(i, ln)| {
                ln.chars()
                    .enumerate()
                    .map(move |(j, c)| Topogaphy::build(i, j, c))
            })
            .inspect(|p| {
                if p.height == 0 {
                    starting_positions.push(*p);
                }
            })
            .collect();
        LavaProductionFacility {
            map,
            starting_positions,
        }
    }

    fn routes<'a>(&self, node: &Topogaphy) -> HashSet<Vec<Topogaphy>> {
        match node.height {
            9 => HashSet::from([vec![*node]]),
            _ => node
                .ascending_neighbors()
                .into_iter()
                .filter(|n| self.map.contains(n))
                .fold(HashSet::<Vec<Topogaphy>>::new(), |mut acc, n| {
                    self.routes(&n).into_iter().for_each(|v| {
                        let mut path = v.clone();
                        path.push(*node);
                        acc.insert(path);
                    });
                    acc
                }),
        }
    }
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let map: LavaProductionFacility = LavaProductionFacility::from_input(&input);

    println!("trailheads: {}", map.starting_positions.len());
    let result: usize = map.starting_positions[..]
        .into_iter()
        .map(|p| {
            let result = map.routes(p);
            let distinct_9s: HashSet<Topogaphy> =
                result.iter().fold(HashSet::new(), |mut acc, y| {
                    assert!(y[0].height == 9);
                    acc.insert(y[0]);
                    acc
                });
            distinct_9s.len()
        })
        .sum();
    Ok(result.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let map: LavaProductionFacility = LavaProductionFacility::from_input(&input);

    println!("trailheads: {}", map.starting_positions.len());
    let result: usize = map.starting_positions[..]
        .into_iter()
        .map(|p| {
            let result = map.routes(p);
            result.len()
        })
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    #[test]
    fn test_part1() {
        let input = INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "36")
    }
    #[test]
    fn test_part2() {
        let input = INPUT.to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "81")
    }
}
