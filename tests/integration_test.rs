use aoc_2024::module_runner::SolutionRunner;
use test_case::test_case;


#[test_case(1, 1, "1151792")]
#[test_case(1, 2, "21790168")]
fn completed_solutions(day: u8, part: u8, expected: &str) {
    let actual = SolutionRunner::new(day, part)
        .solve();
    assert!(actual.is_ok());
    assert_eq!(actual.unwrap(), expected);
}