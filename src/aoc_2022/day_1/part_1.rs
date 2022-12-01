fn calorie_counting_part_one(input_str: &str) -> u64 {
    *input_str
    .lines()
    .fold(Vec::new(), |mut acc, number_str| {
        if let Ok(number) = number_str.trim().parse::<u64>() {
            if acc.last().is_some() {
                let last = acc.last_mut().unwrap();
                *last += number;
            } else {
                acc.push(number);
            }
        } else {
            acc.push(0);
        }
        acc
    }).iter().max().unwrap()
}


#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_calorie_counting() {
        let input_str = r"#
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        #";
        assert_eq!(24000, calorie_counting_part_one(input_str));
    }

    #[test]
    fn test_calorie_counting_from_file() {
        let input_str = fs::read_to_string(".\\src\\aoc_2022\\day_1\\input.txt").unwrap();
        assert_eq!(69206, calorie_counting_part_one(&input_str));
    }
}
