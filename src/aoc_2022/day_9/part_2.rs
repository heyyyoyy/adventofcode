use std::{collections::HashSet, cell::RefCell};


#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl<'a> From<&'a str> for Direction {
    fn from(input: &'a str) -> Self {
        match input {
            "U" => {Direction::Up},
            "D" => {Direction::Down},
            "L" => {Direction::Left},
            "R" => {Direction::Right},
            _ => unreachable!()
        }
    }
}


fn rope_bridge(input_str: &str) -> u64 {
    let directions = input_str.lines().flat_map(|line| {
        let mut command = line.split_whitespace();
        std::iter::repeat(Direction::from(command.next().unwrap())).take(command.next().unwrap().parse().unwrap())
    })
    .collect::<Vec<Direction>>();

    let rope = (0..10).map(|_| RefCell::new((0,0))).collect::<Vec<RefCell<(i32,i32)>>>();
    let mut total = HashSet::from([(0,0)]);

    for direction in directions {
        let mut window = rope.windows(2);
        match direction {
            Direction::Left => {
                rope[0].borrow_mut().0 -= 1;
            },
            Direction::Right => {
                rope[0].borrow_mut().0 += 1;
            },
            Direction::Up => {
                rope[0].borrow_mut().1 += 1;
            },
            Direction::Down => {
                rope[0].borrow_mut().1 -= 1;
            },
        }
        while let Some([head, tail]) = window.next() {
            let (x_step, y_step) = {
                let head_ref = head.borrow();
                let tail_ref = tail.borrow();

                let x_step = head_ref.0 - tail_ref.0;
                let y_step = head_ref.1 - tail_ref.1;
                (x_step, y_step)
            };

            match (x_step, y_step) {
                (0, y) if y.abs() > 1 => {
                    tail.borrow_mut().1 += y_step.signum();
                },
                (x, 0) if x.abs() > 1 => {
                    tail.borrow_mut().0 += x_step.signum();
                },
                (x, y) if x.abs() > 1 || y.abs() > 1 => {
                    tail.borrow_mut().0 += x_step.signum();
                    tail.borrow_mut().1 += y_step.signum();
                },
                _ => {}
            }

        }
        total.insert(*rope.last().unwrap().borrow());
    }
    total.len() as u64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope_bridge() {
        let input_str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, rope_bridge(input_str));
    }

    #[test]
    fn test_rope_bridge_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(2372, rope_bridge(input_str));
    }
}

