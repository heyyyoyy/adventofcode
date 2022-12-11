use std::{cell::RefCell, str::FromStr, collections::VecDeque};


#[derive(Debug)]
struct TestWorry {
    div_by: u64,
    if_true: usize,
    if_false: usize
}

#[derive(Debug)]
enum OpEnum {
    Old,
    Digit(u64)
}

impl FromStr for OpEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(res) = s.parse::<u64>() {
            Ok(OpEnum::Digit(res))
        } else {
            Ok(OpEnum::Old)
        }
    }
}

#[derive(Debug)]
enum Operand {
    Add,
    Mul
}

impl From<char> for Operand {
    fn from(ch: char) -> Self {
        match ch {
            '*' => Self::Mul,
            '+' => Self::Add,
            _ => panic!("wrong operand")
        }
    }
}

#[derive(Debug)]
struct Operation {
    operand: Operand,
    second: OpEnum
}

#[derive(Debug)]
struct Monkey {
    items: RefCell<VecDeque<u64>>,
    operation: Operation,
    test_worry: TestWorry,
    total_check: RefCell<u64>
}

impl Monkey {
    fn parse_input(input_str: &str) -> Self {
        let mut lines = input_str.lines();
        lines.next().unwrap();
        
        let arr_str = lines.next().unwrap().split(": ").last().unwrap();
        let arr = arr_str.split(", ").map(|number| number.parse::<u64>().unwrap()).collect::<VecDeque<_>>();

        let mut op_str = lines.next().unwrap().split("old ").last().unwrap().split_whitespace();
        let operand = op_str.next().unwrap().chars().last().unwrap().into();
        let second = op_str.next().unwrap().parse::<OpEnum>().unwrap();

        let div_by = lines.next().unwrap().split("by ").last().unwrap().parse::<u64>().unwrap();
        let if_true = lines.next().unwrap().split("monkey ").last().unwrap().parse::<usize>().unwrap();
        let if_false = lines.next().unwrap().split("monkey ").last().unwrap().parse::<usize>().unwrap();
        
        Self { 
            items: RefCell::new(arr), 
            operation: Operation {operand, second}, 
            test_worry: TestWorry { div_by, if_true, if_false },
            total_check: RefCell::new(0)
        }
    }

    fn analyze_item(&self, monkeys: &Vec<Monkey>) {
        while let Some(old) = self.items.borrow_mut().pop_front() {
            *self.total_check.borrow_mut() += 1;
            let new = match self.operation.operand {
                Operand::Add => {
                    match self.operation.second {
                        OpEnum::Digit(num) => {
                            old + num
                        },
                        OpEnum::Old => {
                            old + old
                        }
                    }
                },
                Operand::Mul => {
                    match self.operation.second {
                        OpEnum::Digit(num) => {
                            old * num
                        },
                        OpEnum::Old => {
                            old * old
                        }
                    }
                }
            };

            let result = new / 3;

            let monkey_index = if result % self.test_worry.div_by == 0 {
                self.test_worry.if_true
            } else {
                self.test_worry.if_false
            };

            let monkey = monkeys.get(monkey_index).unwrap();
            monkey.items.borrow_mut().push_back(result);
        }
    }
}

fn monkey_in_the_middle(input_str: &str) -> u64 {

    let monkeys = input_str.split("\n\n").map(Monkey::parse_input).collect::<Vec<_>>();

    for _ in 0..20 {
        for monkey in monkeys.iter() {
            monkey.analyze_item(&monkeys);
        }
    }

    let mut total_values = monkeys
    .iter()
    .map(|m| *m.total_check.borrow())
    .collect::<Vec<_>>();

    total_values.sort_by(|a,b| b.cmp(a));

    total_values[0] * total_values[1]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_in_the_middle() {
        let input_str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1
";
        assert_eq!(10605, monkey_in_the_middle(input_str));
    }

    #[test]
    fn test_monkey_in_the_middle_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(69918, monkey_in_the_middle(input_str));
    }
}
