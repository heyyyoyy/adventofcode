use std::{str::{FromStr, Split}, collections::HashSet};
use std::ops;

struct Range(HashSet<u64>);


impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let mut start_end = s.split('-').map(|num_str| {
            num_str.parse::<u64>().unwrap()
        });

        Range(ops::Range{
            start: start_end.next().unwrap(), end: start_end.next().unwrap() + 1
        }.collect::<HashSet<u64>>())
    }
}

struct Pair(Range, Range);

impl From<Split<'_, char>> for Pair {
    fn from(mut s: Split<'_, char>) -> Self {
        Pair(s.next().unwrap().into(), s.next().unwrap().into())
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(',').into())
    }
}


impl Pair {
    fn is_intersection(&self) -> bool {
        self.0.0.intersection(&self.1.0).count() > 0 || self.1.0.intersection(&self.0.0).count() > 0
    }
}


fn camp_cleanup(input_str: &str) -> u64 {
    input_str
    .lines()
    .filter(|pairs| {
        let pair: Pair = pairs.parse().unwrap();
        pair.is_intersection()
    }).count() as u64
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_camp_cleanup() {
        let input_str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(4, camp_cleanup(input_str))
    }

    #[test]
    fn test_camp_cleanup_from_file() {
        let input_str = fs::read_to_string(".\\src\\aoc_2022\\day_4\\input.txt").unwrap();
        assert_eq!(821, camp_cleanup(&input_str))
    }
}
