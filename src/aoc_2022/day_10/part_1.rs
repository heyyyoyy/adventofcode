
enum Instruction {
    Noop = 1,
    Addx 
}

impl<'a> From<&'a str> for Instruction {
    fn from(instruction: &'a str) -> Self {
        match instruction {
            "noop" => {Instruction::Noop},
            "addx" => {Instruction::Addx},
            _ => unreachable!()
        }
    }
}


fn cathode_ray_tube(input_str: &str) -> i32 {

    let mut reg_x = 1;
    let mut signul = 20;
    let mut cycle = 0;
    let mut total = 0;

    for line in input_str.lines() {
        let mut line_iter = line.split_whitespace();

        match line_iter.next().unwrap().into() {
            Instruction::Noop => {
                cycle += Instruction::Noop as i32;
                if signul == cycle {
                    total += cycle * reg_x;
                    signul += 40;
                }
            },
            Instruction::Addx => {
                for _ in 0..Instruction::Addx as i32 {
                    cycle += 1;
                    if signul == cycle {
                        total += cycle * reg_x;
                        signul += 40;
                    }
                }

                reg_x += line_iter.next().unwrap().parse::<i32>().unwrap();
            }
        }
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cathode_ray_tube() {
        let input_str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(13140, cathode_ray_tube(input_str));
    }

    #[test]
    fn test_cathode_ray_tube_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(13920, cathode_ray_tube(input_str));
    }
}

