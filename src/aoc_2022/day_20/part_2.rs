
fn grove_positioning_system(input_str: &str) -> i64 {
    
    let numbers = input_str
    .lines()
    .enumerate()
    .map(|(idx, line)| (idx, line.parse::<i64>().unwrap() * 811_589_153))
    .collect::<Vec<(usize, i64)>>();

    let mut mixed = numbers.clone();

    for _ in 0..10 {
        for (idx, _) in numbers.iter() {
            let index = mixed.iter().position(|&num| num.0 == *idx).unwrap();
            let rem_value = mixed.remove(index);
            let shift = index as i64 + rem_value.1;
            if shift == 0 {
                mixed.push(rem_value);
            } else {
                let new_index = shift.rem_euclid(mixed.len() as i64) as usize;
                mixed.insert(new_index, rem_value);
            }
        }
    }

    let zero_index = mixed.iter().position(|&num| num.1 == 0).unwrap();
    let first = mixed[(1000 + zero_index) % mixed.len()].1;
    let second = mixed[(2000 + zero_index) % mixed.len()].1;
    let third = mixed[(3000 + zero_index) % mixed.len()].1;

    first + second + third
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_grove_positioning_system() {
        let input_str = "1
2
-3
3
-2
0
4";
        assert_eq!(1623178306, grove_positioning_system(input_str));
    }

    #[test]
    fn test_grove_positioning_system_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(11102539613040, grove_positioning_system(input_str));
    }

    #[bench]
    fn bench_grove_positioning_system(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            grove_positioning_system(input_str)
        })
    }
}
