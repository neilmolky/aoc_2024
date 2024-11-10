use aoc_2023::module_runner::SolutionRunner;
use test_case::test_case;

#[test_case(1, 2, "55358")]
#[test_case(1, 1, "56042")]
fn completed_solutions(day: u8, part: u8, expected: &str) {
    let actual = SolutionRunner::new(day, part)
        .solve();
    assert!(actual.is_ok());
    assert_eq!(actual.unwrap(), expected);
}