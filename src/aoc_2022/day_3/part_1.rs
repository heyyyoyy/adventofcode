fn rucksack(input_str: &str) -> u64 {
    let mut chars: Vec<char> = ('a'..='z').collect();
    chars.extend(('A'..='Z').collect::<Vec<char>>());

    input_str
    .lines()
    .map(|line| {
        let (first, second) = line.split_at(line.len() / 2);
        let uniq_ch = first.chars().filter(|&ch| second.contains(ch)).next().unwrap();
        chars.iter().position(|&ch| ch == uniq_ch ).unwrap() as u64 + 1
    })
    .sum()
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_rucksack() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(157, rucksack(input_str));
    }

    #[test]
    fn test_rucksack_from_file() {
        let input_str = fs::read_to_string(".\\src\\aoc_2022\\day_3\\input.txt").unwrap();
        assert_eq!(7889, rucksack(&input_str));
    }
}
