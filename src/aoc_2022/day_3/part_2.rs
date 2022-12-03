fn rucksack(input_str: &str) -> u64 {
    // build vector with chars a-zA-Z
    let mut chars: Vec<char> = ('a'..='z').collect();
    chars.extend(('A'..='Z').collect::<Vec<char>>());

    // grouping by three lines 
    let groupes: Vec<Vec<&str>> = input_str
    .lines()
    .fold(Vec::new(), |mut acc, line| {
        if acc.last().is_some() && acc.last().unwrap().len() < 3 {
            acc.last_mut().unwrap().push(line);
        } else {
            acc.push(vec![line]);
        }
        acc
    });

    groupes
    .iter()
    .flat_map(|arr| {
        // sort and remove duplicates
        let mut chars: Vec<char> = arr[0].chars().collect();
        chars.sort();
        chars.dedup();

        chars.into_iter().filter_map(|ch| {
            if arr[1].contains(ch) && arr[2].contains(ch) {
                Some(ch)
            } else {
                None
            }
        })
    })
    .map(|uniq_ch| {
        chars.iter().position(|&ch| ch == uniq_ch).unwrap() as u64 + 1
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
        assert_eq!(70, rucksack(input_str));
    }

    #[test]
    fn test_rucksack_from_file() {
        let input_str = fs::read_to_string(".\\src\\aoc_2022\\day_3\\input.txt").unwrap();
        assert_eq!(2825, rucksack(&input_str));
    }
}
