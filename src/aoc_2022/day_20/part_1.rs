
fn grove_positioning_system(input_str: &str) -> i32 {
    
    let numbers = input_str
    .lines()
    .enumerate()
    .map(|(idx, line)| (idx, line.parse().unwrap()))
    .collect::<Vec<(usize, i32)>>();

    let mut mixed = numbers.clone();

    for (idx, _) in numbers.iter() {
        let index = mixed.iter().position(|&num| num.0 == *idx).unwrap();
        let rem_value = mixed.remove(index);
        let shift = index as i32 + rem_value.1;
        let new_index = shift.rem_euclid(mixed.len() as i32);
        mixed.insert(new_index as usize, rem_value);
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
        assert_eq!(3, grove_positioning_system(input_str));
    }

    #[test]
    fn test_grove_positioning_system_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(11073, grove_positioning_system(input_str));
    }

    #[bench]
    fn bench_grove_positioning_system(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            grove_positioning_system(input_str)
        })
    }
}