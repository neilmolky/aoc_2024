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
#[test_case(6, 1, "4988")]
#[test_case(6, 2, "1697")]
#[test_case(7, 1, "465126289353")]
#[test_case(7, 2, "70597497486371")]
#[test_case(8, 1, "256")]
#[test_case(8, 2, "1005")]
#[test_case(9, 1, "6401092019345")]
#[test_case(9, 2, "6431472344710")]
fn completed_solutions(day: u8, part: u8, expected: &str) {
    let actual = SolutionRunner::new(day, part).solve();
    assert!(actual.is_ok());
    assert_eq!(actual.unwrap(), expected);
}
