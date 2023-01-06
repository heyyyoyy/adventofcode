fn part_1(input_str: &str) -> i64 {
    input_str.lines().fold(0, |mut acc, line| {
        let mut s = line.split_ascii_whitespace();
        let (a, b, c) = (
            s.next().unwrap().parse::<i64>().unwrap(),
            s.next().unwrap().parse::<i64>().unwrap(),
            s.next().unwrap().parse::<i64>().unwrap(),
        );
        if (a + b) > c && (a + c) > b && (b + c) > a {
            acc += 1;
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_part_1_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(917, part_1(input_str));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| part_1(input_str))
    }
}
