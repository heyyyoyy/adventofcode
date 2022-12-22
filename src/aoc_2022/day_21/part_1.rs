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


fn monkey_math(input_str: &str) -> i64 {
    let monkeys = parse(input_str);

    let result = calc(&"root".to_owned(), &monkeys);

    result
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
        assert_eq!(152, monkey_math(input_str));
    }

    #[test]
    fn test_monkey_math_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(21120928600114, monkey_math(input_str));
    }

    #[bench]
    fn bench_monkey_math(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            monkey_math(input_str)
        })
    }
}
