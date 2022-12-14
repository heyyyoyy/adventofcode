fn treetop_tree_house(input_str: &str) -> u64 {
    let arr: Vec<&str> = input_str
    .lines()
    .collect();

    arr
    .iter()
    .enumerate()
    .map(|(idx, &line)| {
        line.chars().enumerate().map(|(in_idx, ch)| {
            let number = ch.to_digit(10).unwrap() as u64;

            let left_range = 0..in_idx;
            let right_range = in_idx + 1..line.len();
            let top_range = 0..idx;
            let bottom_range = idx + 1..arr.len();

            let mut left = 0;
            for n in line[left_range].chars().rev() {
                let n = n.to_digit(10).unwrap() as u64;
                if number > n {
                    left += 1;
                } else {
                    left += 1;
                    break;
                }

            }
            if left == 0 {left = 1}

            let mut right = 0;
            for n in line[right_range].chars() {
                let n = n.to_digit(10).unwrap() as u64;
                if number > n {
                    right += 1;
                } else {
                    right += 1;
                    break;
                }
            }
            if right == 0 {right = 1}

            let mut top = 0;
            for i in top_range.rev() {
                let n = arr[i].chars().nth(in_idx).unwrap().to_digit(10).unwrap() as u64;
                if number > n {
                    top += 1;
                } else {
                    top += 1;
                    break;
                }
            }
            if top == 0 {top = 1}

            let mut bottom = 0;
            for i in bottom_range {
                let n = arr[i].chars().nth(in_idx).unwrap().to_digit(10).unwrap() as u64;
                if number > n {
                    bottom += 1;
                } else {
                    bottom += 1;
                    break;
                }
            }
            if bottom == 0 {bottom = 1}

            left * right * top * bottom as u64
        })
        .collect()

    })
    .collect::<Vec<Vec<u64>>>()
    .into_iter()
    .flatten()
    .max()
    .unwrap()
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
        
        assert_eq!(16, treetop_tree_house(input_str));
    }

    #[test]
    fn test_treetop_tree_house_from_file() {
        let input_str = include_str!(r"input.txt");
        assert_eq!(504000, treetop_tree_house(input_str));
    }
}
