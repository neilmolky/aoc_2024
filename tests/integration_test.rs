use aoc_2024::module_runner::SolutionRunner;
use test_case::test_case;

#[test_case(1, 1, "1151792")]
#[test_case(1, 2, "21790168")]
#[test_case(2, 1, "383")]
#[test_case(2, 2, "436")]
#[test_case(3, 1, "170778545")]
#[test_case(3, 2, "82868252")]
#[test_case(4, 1, "2524")]
#[test_case(4, 2, "1873")]
#[test_case(5, 1, "7198")]
#[test_case(5, 2, "4230")]
fn completed_solutions(day: u8, part: u8, expected: &str) {
    let actual = SolutionRunner::new(day, part).solve();
    assert!(actual.is_ok());
    assert_eq!(actual.unwrap(), expected);
}
