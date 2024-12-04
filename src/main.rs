use aoc_2024::module_runner::SolutionRunner;
use std::env;

fn help() {
    println!(
        "usage:
day {{1..25}} <integer>
    The day to solve
part {{1|2}} <integer>
    The part to solve"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_, day, part, ..] => {
            let day: u8 = match day.parse() {
                Ok(x) if 1 <= x && x <= 25 => x,
                _ => {
                    help();
                    return;
                }
            };
            let part: u8 = match part.parse() {
                Ok(x) if x == 1 || x == 2 => x,
                _ => {
                    help();
                    return;
                }
            };
            let runner = SolutionRunner::new(day, part);
            match runner.solve() {
                Ok(x) => println!("{x}"),
                Err(e) => eprint!("{e}"),
            }
        }
        _ => help(),
    }
    return;
}
