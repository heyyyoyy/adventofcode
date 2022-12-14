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

    let rope = (0..2).map(|_| RefCell::new((0,0))).collect::<Vec<RefCell<(i32,i32)>>>();
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
        let input_str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, rope_bridge(input_str));
    }

    #[test]
    fn test_rope_bridge_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(5683, rope_bridge(input_str));
    }
}

