use std::collections::HashMap;

fn supply_stacks(input_str: &str) -> String {
    let mut split = input_str.split("\n\n");
    let stacks = split.next().unwrap();
    let commands = split.next().unwrap();

    let mut stacks_map: HashMap<i32, Vec<char>> = stacks
    .lines()
    .fold(HashMap::new(), |mut acc, line| {
        let mut column = 1;
        let mut last_pos = 1;
        let mut arr = vec![];
        for (idx, ch) in line.chars().enumerate() {
            if ch.is_alphabetic() {
                arr.push(ch);

            }
            if idx == last_pos {
                acc
                .entry(column)
                .and_modify(|val| val.extend(arr.clone()))
                .or_insert(arr.clone());
                column += 1;
                last_pos += 4;
                arr.clear();
            }
        }
        acc
    });

    let commands_vec: Vec<Vec<i32>> = commands
    .lines()
    .map(|line| {
        line
        .split_whitespace()
        .filter_map(|str| str.parse::<i32>().ok())
        .collect()
    })
    .collect();

    for command in commands_vec.iter() {
        let mut push_chars: Vec<char> = {
            let from_arr = stacks_map.get_mut(&command[1]).unwrap();
            from_arr.drain(..command[0] as usize).collect()
        };
        push_chars.extend(stacks_map.get(&command[2]).unwrap());
        let to_arr = stacks_map.get_mut(&command[2]).unwrap();
        *to_arr = push_chars;
    }

    (1..stacks_map.len() + 1)
    .fold(String::new(), |mut acc, key| {
        let val = stacks_map.get(&(key as i32)).unwrap();
        acc.push(*val.first().unwrap());
        acc
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_supply_stacks() {
        let input_str = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!("MCD", supply_stacks(input_str));
    }

    #[test]
    // #[ignore]
    fn test_supply_stacks_from_file() {
        let input_str = fs::read_to_string(".\\src\\aoc_2022\\day_5\\input.txt").unwrap();
        assert_eq!("LCTQFBVZV", supply_stacks(&input_str))
    }
}
