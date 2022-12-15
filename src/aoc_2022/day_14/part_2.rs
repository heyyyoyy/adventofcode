use std::collections::HashMap;

use itertools::Itertools;


fn step(pos: (i32,i32), cave: &HashMap<(i32,i32), char>) -> Option<(i32,i32)> {
    for delta in [0, -1, 1] {
        let new_pos = (pos.0 + delta, pos.1 + 1);
        if !cave.contains_key(&new_pos) {
            return Some(new_pos);
        }
    }
    None
}

fn drop_sand(cave: &mut HashMap<(i32,i32), char>, bottom: i32) -> bool {
    let mut cur_pos = (500, 0);

    if cave.contains_key(&cur_pos) {
        return false;
    }

    while let Some(next_pos) = step(cur_pos, &cave) {
        cur_pos = next_pos;
        if cur_pos.1 == bottom + 1 {
            break;
        }
    }
    cave.insert(cur_pos, 'o');
    true
}


fn regolith_reservoir(input_str: &str) -> usize {

    let mut cave: HashMap<(i32,i32), char> = HashMap::new();

    for line in input_str.lines() {
        let ranges = line.split(" -> ")
        .collect::<Vec<_>>();

        let points = ranges
        .windows(2)
        .flat_map(|rng| {
            let first = rng[0].split_once(',').unwrap();
            let second = rng[1].split_once(',').unwrap();
            let (fx, fy) = (first.0.parse::<i32>().unwrap(), first.1.parse::<i32>().unwrap());
            let (sx, sy) = (second.0.parse::<i32>().unwrap(), second.1.parse::<i32>().unwrap());

            let x_range = if fx > sx {sx..=fx} else {fx..=sx};
            let y_range = if fy > sy {sy..=fy} else {fy..=sy};

            x_range.cartesian_product(y_range).collect::<Vec<_>>()

        })
        .collect::<Vec<_>>();

        for point in points {
            cave.insert(point, '#');
        }
    }

    let bottom = cave.keys().map(|&point| point.1).max().unwrap();
    let mut count = 0;
    while drop_sand(&mut cave, bottom) {
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_regolith_reservoir() {
        let input_str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(93, regolith_reservoir(input_str));
    }

    #[test]
    fn test_regolith_reservoir_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(23610, regolith_reservoir(input_str));
    }

    #[bench]
    fn bench_regolith_reservoir(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            regolith_reservoir(input_str)
        })
    }
}
