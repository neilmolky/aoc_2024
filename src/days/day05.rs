use crate::error;

use std::{collections::HashMap, iter};

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    sequence::separated_pair, IResult,
};

fn number_pair(s: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(
        map_res(digit1, str::parse),
        tag("|"),
        map_res(digit1, str::parse),
    )(s)
}

fn book_pages(s: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), map_res(digit1, str::parse::<u32>))(s)
}

struct PageOrder {
    order_map: HashMap<u32, Vec<u32>>,
}

impl PageOrder {
    pub fn build_from_input(input: &String) -> PageOrder {
        let order_map: HashMap<u32, Vec<u32>> = input
            .lines()
            .take_while(|l| !l.is_empty())
            .map(number_pair)
            .map(|r| r.unwrap().1)
            .fold(HashMap::new(), |mut acc, x| {
                acc.entry(x.0)
                    .and_modify(|v| v.push(x.1))
                    .or_insert(vec![x.1]);
                acc
            });
        PageOrder { order_map }
    }

    pub fn is_before(&self, prior_page: &u32, page: &u32) -> bool {
        self.order_map.get(page).map_or(true, |must_be_beffore| {
            !must_be_beffore.contains(prior_page)
        })
    }

    pub fn is_ordered(&self, history: &Vec<u32>, page: &u32) -> bool {
        history
            .iter()
            .all(|prior_page| self.is_before(prior_page, page))
    }

    pub fn insert(&self, history: &Vec<u32>, page: &u32) -> Vec<u32> {
        let prior = history
            .iter()
            .take_while(|prior_page| self.is_before(*prior_page, page));
        let i = iter::once(page);
        let post = history
            .iter()
            .skip_while(|prior_page| self.is_before(*prior_page, page));
        prior.chain(i).chain(post).map(|x| *x).collect()
    }
}

pub fn part1(input: String) -> Result<String, error::Error> {
    let p = PageOrder::build_from_input(&input);

    let middle_page_numbers: u32 = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(book_pages)
        .map(|r| r.unwrap().1)
        .map(|pages| {
            pages.iter().fold(
                Some(Vec::<u32>::new()),
                |previous_pages, page| match previous_pages {
                    None => None,
                    Some(history) => {
                        if p.is_ordered(&history, page) {
                            Some(
                                history
                                    .iter()
                                    .chain(std::iter::once(page))
                                    .map(|x| *x)
                                    .collect(),
                            )
                        } else {
                            None
                        }
                    }
                },
            )
        })
        .flat_map(|s| s)
        .map(|s| s[s.len() / 2])
        .sum();

    Ok(middle_page_numbers.to_string())
}

pub fn part2(input: String) -> Result<String, error::Error> {
    let p = PageOrder::build_from_input(&input);

    let middle_page_numbers: u32 = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(book_pages)
        .map(|r| r.unwrap().1)
        .map(|pages| {
            (
                pages.clone(),
                pages
                    .iter()
                    .fold(Vec::<u32>::new(), |history, page| p.insert(&history, &page)),
            )
        })
        .filter(|(original_pages, sorted_pages)| original_pages != sorted_pages)
        .map(|(_, sorted_pages)| sorted_pages[sorted_pages.len() / 2])
        .sum();

    Ok(middle_page_numbers.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    #[test]
    fn test_part1() {
        let input = INPUT.to_string();
        let result = part1(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "143")
    }
    #[test]
    fn test_part2() {
        let input = INPUT.to_string();
        let result = part2(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "123")
    }
}
