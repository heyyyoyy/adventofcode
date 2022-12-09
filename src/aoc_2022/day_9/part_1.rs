use std::collections::HashSet;

fn t(input_str: &str) -> u64 {
    let mut prev_position = (0, 0);
    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut steps = HashSet::new();
    steps.insert(tail);

    for line in input_str.lines() {
        let mut command = line.split_whitespace();
        match (command.next().unwrap(), command.next().unwrap()) {
            ("R", step) => {
                let mut step = step.parse::<u64>().unwrap();
                while step > 0 {
                    prev_position = head;
                    head.1 += 1;
                    match (i32::abs(head.0 - tail.0), i32::abs(head.1 - tail.1)) {
                        (0,0) | (0,1) | (1,0) | (1,1) => {},
                        _ => {
                            tail = prev_position;
                            steps.insert(tail);
                        }
                    }
                    step -= 1;
                }
            },
            ("L", step) => {
                let mut step = step.parse::<u64>().unwrap();
                while step > 0 {
                    prev_position = head;
                    head.1 -= 1;
                    match (i32::abs(head.0 - tail.0), i32::abs(head.1 - tail.1)) {
                        (0,0) | (0,1) | (1,0) | (1,1) => {},
                        _ => {
                            tail = prev_position;
                            steps.insert(tail);
                        }
                    }
                    step -= 1;
                }
            },
            ("U", step) => {
                let mut step = step.parse::<u64>().unwrap();
                while step > 0 {
                    prev_position = head;
                    head.0 += 1;
                    match (i32::abs(head.0 - tail.0), i32::abs(head.1 - tail.1)) {
                        (0,0) | (0,1) | (1,0) | (1,1) => {},
                        _ => {
                            tail = prev_position;
                            steps.insert(tail);
                        }
                    }
                    step -= 1;
                }
            },
            ("D", step) => {
                let mut step = step.parse::<u64>().unwrap();
                while step > 0 {
                    prev_position = head;
                    head.0 -= 1;
                    match (i32::abs(head.0 - tail.0), i32::abs(head.1 - tail.1)) {
                        (0,0) | (0,1) | (1,0) | (1,1) => {},
                        _ => {
                            tail = prev_position;
                            steps.insert(tail);
                        }
                    }
                    step -= 1;
                }
            },
            _ => {unreachable!()},
        }
        
    }
    steps.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_counter() {
        let input_str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, t(input_str));
    }

    #[test]
    fn test_size_counter_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(5683, t(input_str));
    }
}

