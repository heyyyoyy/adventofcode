use std::collections::HashMap;

#[derive(Debug)]
enum Job {
    Number(i64),
    Operation((String, char, String))

}

fn parse(input_str: &str) -> HashMap<String, Job> {
    input_str
    .lines()
    .fold(HashMap::new(), |mut acc, line|{
        let (monkey, job) = line.split_once(": ").unwrap();
        let job_enum = if let Ok(num)  = job.parse::<i64>() {
            Job::Number(num)
        } else {
            let mut op = job.split(' ');
            Job::Operation((
                op.next().unwrap().to_owned(),
                op.next().unwrap().chars().next().unwrap(),
                op.next().unwrap().to_owned(),
            ))
        };
        acc.insert(monkey.to_owned(), job_enum);
        acc
    })
}

fn calc(monkey_name: &String, monkeys: &HashMap<String, Job>) -> i64 {

    let job = monkeys.get(monkey_name).unwrap();
    match job {
        Job::Number(num) => {return *num;},
        Job::Operation((lhs, operation, rhs)) => {
            let left = calc(&lhs, monkeys);
            let right = calc(&rhs, monkeys);
            match operation {
                '+' => {
                    return left + right;
                },
                '-' => {
                    return left - right;
                },
                '*' => {
                    return left * right;
                },
                '/' => {
                    return left / right;
                },
                _ => unreachable!("wrong operation")
            }
        }
    }
}

fn get_humn_graf<'a>(monkey_name: &'a String, monkeys: &'a HashMap<String, Job>) -> Option<Vec<&'a String>>  {

    if monkey_name == "humn" {
        return Some(vec![monkey_name]);
    }
    if let Some(Job::Operation((left, _, right))) = monkeys.get(monkey_name) {
        if let Some(mut vec) = get_humn_graf(left, monkeys) {
            vec.push(monkey_name);
            return Some(vec);
        }
        if let Some(mut vec) = get_humn_graf(right, monkeys) {
            vec.push(monkey_name);
            return Some(vec);
        }
    }
    None
}

fn calc_humn_number(
    path: &Vec<&String>,
    index: usize,
    monkeys: &HashMap<String, Job>,
    current_value: i64
) -> i64 {
    match monkeys.get(path[index]).unwrap() {
        Job::Number(_) => current_value,
        Job::Operation((lhs, operation, rhs)) => {
            let left = calc(&lhs, monkeys);
            let right = calc(&rhs, monkeys);
            let new_value = if lhs == path[index + 1] {
                match operation {
                    '+' => current_value - right,
                    '-' => current_value + right,
                    '*' => current_value / right,
                    '/' => current_value * right,
                    _ => unreachable!("wrong operation")
                }
            } else {
                match operation {
                    '+' => current_value - left,
                    '-' => left - current_value,
                    '*' => current_value / left,
                    '/' => left / current_value,
                    _ => unreachable!("wrong operation")
                }
            };
            calc_humn_number(path, index + 1, monkeys, new_value)
        }
    }
}


fn monkey_math(input_str: &str) -> i64 {
    let monkeys = parse(input_str);
    let root = "root".to_string();

    let path = get_humn_graf(&root, &monkeys).unwrap();
    let path = path.into_iter().rev().collect::<Vec<_>>();

    let Some(Job::Operation((left, _, right))) = monkeys.get(&root) else {
        unreachable!("number is wrong");
    };
    
    let target = if left == path[1] {
        calc(right, &monkeys)
    } else {
        calc(left, &monkeys)
    };

    calc_humn_number(&path, 1, &monkeys, target)
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_monkey_math() {
        let input_str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        assert_eq!(301, monkey_math(input_str));
    }

    #[test]
    fn test_monkey_math_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(3453748220116, monkey_math(input_str));
    }

    #[bench]
    fn bench_monkey_math(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            monkey_math(input_str)
        })
    }
}
