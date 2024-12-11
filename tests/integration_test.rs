use aoc_2024::module_runner::SolutionRunner;
use test_case::test_case;
use std::fmt;
use std::process::ExitCode;
use std::process::Termination;

#[derive(Debug, PartialEq, Eq)]
pub struct TestCode(pub String);

impl fmt::Display for TestCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {

            TestCode(e) => write!(f, "{e}")
        }
    }
}
impl Termination for TestCode {
    fn report(self) -> std::process::ExitCode {
        ExitCode::SUCCESS
    }
}

#[test_case(1, 1 => TestCode("1151792".to_string()))]
#[test_case(1, 2 => TestCode("21790168".to_string()))]
#[test_case(2, 1 => TestCode("383".to_string()))]
#[test_case(2, 2 => TestCode("436".to_string()))]
#[test_case(3, 1 => TestCode("170778545".to_string()))]
#[test_case(3, 2 => TestCode("82868252".to_string()))]
#[test_case(4, 1 => TestCode("2524".to_string()))]
#[test_case(4, 2 => TestCode("1873".to_string()))]
#[test_case(5, 1 => TestCode("7198".to_string()))]
#[test_case(5, 2 => TestCode("4230".to_string()))]
#[test_case(6, 1 => TestCode("4988".to_string()))]
#[test_case(6, 2 => ignore TestCode("1697".to_string()); "6_2_Requires optimization, long running test")]
#[test_case(7, 1 => TestCode("465126289353".to_string()))]
#[test_case(7, 2 => TestCode("70597497486371".to_string()))]
#[test_case(8, 1 => TestCode("256".to_string()))]
#[test_case(8, 2 => TestCode("1005".to_string()))]
#[test_case(9, 1 => TestCode("6401092019345".to_string()))]
#[test_case(9, 2 => TestCode("6431472344710".to_string()))]
#[test_case(10, 1 => TestCode("776".to_string()))]
#[test_case(10, 2 => TestCode("1657".to_string()))]
fn completed_solutions(day: u8, part: u8) -> TestCode {
    TestCode(SolutionRunner::new(day, part).solve().unwrap())
}
