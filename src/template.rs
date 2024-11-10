use crate::error;


pub fn part1(_input: String) -> Result<String, error::Error> {
    Err(error::Error::SolutionNotExists)
}

pub fn part2(_input: String) -> Result<String, error::Error> {
    Err(error::Error::SolutionNotExists)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    #[test]
    fn test_part1() {
        let input = "".to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "")

    }
    #[ignore]
    #[test]
    fn test_part2() {
        let input = "".to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "")

    }
}