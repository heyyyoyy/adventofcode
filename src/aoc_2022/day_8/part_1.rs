fn treetop_tree_house(input_str: &str) -> u64 {
    let arr: Vec<&str> = input_str
    .lines()
    .collect();

    arr
    .iter()
    .enumerate()
    .map(|(idx, &line)| {
        if idx == 0 || idx == arr.len() - 1 {
            return line.chars().map(|ch| ch.to_digit(10).unwrap() as u64).collect::<Vec<u64>>();
        }
        line
        .chars()
        .enumerate()
        .filter_map(|(in_idx, ch)| {
            let number = ch.to_digit(10).unwrap() as u64;
            if in_idx == 0 || in_idx == line.len() - 1 {
                return Some(number);
            }
            let left_range = 0..in_idx;
            let right_range = in_idx + 1..line.len();
            let top_range = 0..idx;
            let bottom_range = idx + 1..arr.len();

            let mut is_visible = false;
            for n in line[left_range].chars().rev() {
                let n = n.to_digit(10).unwrap() as u64;
                if number > n {
                    is_visible = true;
                } else {
                    is_visible = false;
                    break;
                }
            }

            if is_visible {
                return Some(number);
            }
            

            for n in line[right_range].chars() {
                let n = n.to_digit(10).unwrap() as u64;
                if number > n {
                    is_visible = true;
                } else {
                    is_visible = false;
                    break;
                }
            }

            if is_visible {
                return Some(number);
            }
            
            for i in top_range {
                let n = arr[i].chars().nth(in_idx).unwrap().to_digit(10).unwrap() as u64;
                if number > n {
                    is_visible = true;
                } else {
                    is_visible = false;
                    break;
                }
            }

            if is_visible {
                return Some(number);
            }
            

            for i in bottom_range {
                let n = arr[i].chars().nth(in_idx).unwrap().to_digit(10).unwrap() as u64;
                if number > n {
                    is_visible = true;
                } else {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                return Some(number);
            }
            None
        })
        .collect::<Vec<u64>>()

    })
    .collect::<Vec<Vec<u64>>>()
    .into_iter()
    .flatten()
    .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treetop_tree_house() {
        let input_str = "30373
25512
65332
33549
35390";
        assert_eq!(21, treetop_tree_house(input_str));
    }

    #[test]
    fn test_treetop_tree_house_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(1708, treetop_tree_house(input_str));
    }
}
