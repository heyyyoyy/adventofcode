use std::collections::HashSet;


fn get_cubes(input_str: &str) -> HashSet<(i32,i32,i32)> {
    input_str
    .lines()
    .map(|line| {
        let mut s = line.split(',');
        (
            s.next().unwrap().parse().unwrap(),
            s.next().unwrap().parse().unwrap(),
            s.next().unwrap().parse().unwrap(),
        )
    })
    .collect()
}

fn boiling_boulders(input_str: &str) -> i32 {
    let deltas = [
        (-1,0,0),
        (1,0,0),
        (0,-1,0),
        (0,1,0),
        (0,0,-1),
        (0,0,1)
    ];
    let cubes = get_cubes(input_str);
    let mut count = 0;
    for cube in cubes.iter() {
        for delta in deltas.iter() {
            let position = (cube.0 + delta.0, cube.1 + delta.1, cube.2 + delta.2);
            if !cubes.contains(&position) {
                count += 1;
            }
        }
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_boiling_boulders() {
        let input_str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(64, boiling_boulders(input_str));
    }

    #[test]
    fn test_boiling_boulders_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(4608, boiling_boulders(input_str));
    }

    #[bench]
    fn bench_boiling_boulders(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            boiling_boulders(input_str)
        })
    }
}
