use std::fs;
use crate::{error, days};

#[derive(Debug, Clone)]
pub struct SolutionRunner {
    day: u8,
    part: u8
}

impl SolutionRunner {
    pub fn solve(&self) -> Result<String, error::Error> {
        let filename = match self.day {
            x if x < 10 => format!("day0{}.txt", x),
            x => format!("day{}.txt", x)
        };
        let crate_root = env!("CARGO_MANIFEST_DIR");
        let data_dir = format!("{}/data/{}", &crate_root, &filename);
        let data = fs::read_to_string(data_dir);
        match data {
            Ok(d) => match (self.day, self.part) {
                // uncomment after running ./makeday.sh
                (1, 1) => days::day01::part1(d),
                (1, 2) => days::day01::part2(d),
                (2, 1) => days::day02::part1(d),
                (2, 2) => days::day02::part2(d),
                (3, 1) => days::day03::part1(d),
                (3, 2) => days::day03::part2(d),
                (4, 1) => days::day04::part1(d),
                (4, 2) => days::day04::part2(d),
                (5, 1) => days::day05::part1(d),
                (5, 2) => days::day05::part2(d),
                (6, 1) => days::day06::part1(d),
                (6, 2) => days::day06::part2(d),
                (7, 1) => days::day07::part1(d),
                (7, 2) => days::day07::part2(d),
                (8, 1) => days::day08::part1(d),
                (8, 2) => days::day08::part2(d),
                (9, 1) => days::day09::part1(d),
                (9, 2) => days::day09::part2(d),
                (10, 1) => days::day10::part1(d),
                (10, 2) => days::day10::part2(d),
                (11, 1) => days::day11::part1(d),
                (11, 2) => days::day11::part2(d),
                (12, 1) => days::day12::part1(d),
                (12, 2) => days::day12::part2(d),
                (13, 1) => days::day13::part1(d),
                (13, 2) => days::day13::part2(d),
                (14, 1) => days::day14::part1(d),
                (14, 2) => days::day14::part2(d),
                (15, 1) => days::day15::part1(d),
                (15, 2) => days::day15::part2(d),
                (16, 1) => days::day16::part1(d),
                (16, 2) => days::day16::part2(d),
                (17, 1) => days::day17::part1(d),
                (17, 2) => days::day17::part2(d),
                (18, 1) => days::day18::part1(d),
                (18, 2) => days::day18::part2(d),
                (19, 1) => days::day19::part1(d),
                (19, 2) => days::day19::part2(d),
                (20, 1) => days::day20::part1(d),
                (20, 2) => days::day20::part2(d),
                (21, 1) => days::day21::part1(d),
                (21, 2) => days::day21::part2(d),
                (22, 1) => days::day22::part1(d),
                (22, 2) => days::day22::part2(d),
                (23, 1) => days::day23::part1(d),
                (23, 2) => days::day23::part2(d),
                (24, 1) => days::day24::part1(d),
                (24, 2) => days::day24::part2(d),
                (25, 1) => days::day25::part1(d),
                (25, 2) => days::day25::part2(d),
                (day, part) => panic!("day {day} part {part} should have been excluded from command line input")
            }
            Err(e) => Err(error::Error::FileNotFound(e))
        }
    }
    pub fn new(day: u8, part: u8) -> SolutionRunner {
        SolutionRunner { day, part }

    }
}