use array2d::Array2D;

use crate::{
    error,
    utils::{ArrayUtils2D, Gradient},
};

pub fn part1(input: String) -> Result<String, error::Error> {
    let arr: Array2D<char> = Array2D::from_rows(
        input
            .lines()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
            .as_slice(),
    )
    .unwrap();
    let a_lim = ArrayUtils2D::from_array(&arr);
    let search_term = "XMAS";
    let rev_search_term: String = search_term.chars().rev().collect();

    let line_count: usize = a_lim
        .lines(Gradient::NegativeDiagonal)
        .iter()
        .chain(a_lim.lines(Gradient::PositiveDiagonal).iter())
        .chain(a_lim.lines(Gradient::Horizontal).iter())
        .chain(a_lim.lines(Gradient::Vertical).iter())
        .filter(|x| x.len() >= search_term.len())
        .map(|x| {
            x.iter()
                .map(|y| arr.get(y.0, y.1).unwrap())
                .collect::<String>()
        })
        .map(|x| {
            x.match_indices(search_term).count() + x.match_indices(rev_search_term.as_str()).count()
        })
        .sum();
    Ok(line_count.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let arr: Array2D<char> = Array2D::from_rows(
        input
            .lines()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
            .as_slice(),
    )
    .unwrap();
    let search_term = "MAS";
    let rev_search_term: String = search_term.chars().rev().collect();

    let x_count = (0..=(arr.num_rows() - search_term.len()))
        .flat_map(|i| (0..=(arr.num_columns() - search_term.len())).map(move |j| (i, j)))
        .filter(|(i, j)| {
            let d1 = (0..search_term.len())
                .map(|n| arr.get(i + n, j + n).unwrap())
                .collect::<String>();
            let d2 = (0..search_term.len())
                .map(|n| {
                    arr.get(i + search_term.len().abs_diff(n + 1), j + n)
                        .unwrap()
                })
                .collect::<String>();
            (d1 == search_term || d1 == rev_search_term)
                && (d2 == search_term || d2 == rev_search_term)
        })
        .count();
    Ok(x_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        let input = INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "18")
    }
    #[test]
    fn test_part2() {
        let input = INPUT.to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "9")
    }
}
