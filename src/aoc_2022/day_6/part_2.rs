use std::collections::HashSet;

fn tuning_trouble(input_str: &str) -> usize {
    let slice_length = 14;
    let mut start_pos = 0;
    let mut end_pos = slice_length;
    loop {
        let sub_string = &input_str[start_pos..end_pos];
        let chars: HashSet<char> = sub_string.chars().collect();

        if sub_string.len() == chars.len() {
            break end_pos;
        }
        start_pos += 1;
        end_pos +=1;
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_tuning_trouble() {
        let input_str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, tuning_trouble(input_str));

        let input_str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(23, tuning_trouble(input_str));

        let input_str = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(23, tuning_trouble(input_str));

        let input_str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(29, tuning_trouble(input_str));

        let input_str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(26, tuning_trouble(input_str));
    }

    #[test]
    fn test_tuning_trouble_from_file() {
        let input_str = fs::read_to_string(".\\src\\aoc_2022\\day_6\\input.txt").unwrap();
        assert_eq!(3444, tuning_trouble(&input_str))
    }
}
