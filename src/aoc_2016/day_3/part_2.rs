fn part_2(input_str: &str) -> i64 {
    let mut triangles = vec![];
    for (col, line) in input_str.lines().enumerate() {
        let mut sp = line.split_ascii_whitespace();
        let (a, b, c) = (
            sp.next().unwrap().parse::<i64>().unwrap(),
            sp.next().unwrap().parse::<i64>().unwrap(),
            sp.next().unwrap().parse::<i64>().unwrap(),
        );
        if col % 3 == 0 {
            triangles.push(vec![a]);
            triangles.push(vec![b]);
            triangles.push(vec![c]);
        } else {
            let len = triangles.len();
            triangles[len - 3].push(a);
            triangles[len - 2].push(b);
            triangles[len - 1].push(c);
        }
    }
    triangles.iter().fold(0, |mut acc, tr| {
        let (a, b, c) = (tr[0], tr[1], tr[2]);
        if a + b > c && a + c > b && b + c > a {
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
    fn test_part_2_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(1649, part_2(input_str));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| part_2(input_str))
    }
}
