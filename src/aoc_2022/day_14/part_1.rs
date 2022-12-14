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

fn drop_sand(sand_point: (i32,i32), cave: &mut HashMap<(i32,i32), char>, bottom: i32) -> bool {
    let mut cur_pos = sand_point;

    while let Some(next_pos) = step(cur_pos, &cave) {
        if next_pos.1 > bottom {
            return false;
        }
        cur_pos = next_pos;
    }
    cave.insert(cur_pos, 'o');
    true
}


fn regolith_reservoir(input_str: &str) -> usize {

    let mut cave: HashMap<(i32,i32), char> = HashMap::new();

    let sand_point = (500, 0);
    cave.insert(sand_point, '+');

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
    while drop_sand(sand_point, &mut cave, bottom) {
        count += 1;
    }

    // let x = cave.keys().map(|&point| point.0);
    // let y = cave.keys().map(|&point| point.1);

    // let (x_min, x_max) = (x.clone().min().unwrap(), x.max().unwrap());
    // let (y_min, y_max) = (y.clone().min().unwrap(), y.max().unwrap());

    // for y in y_min..=y_max {
    //     for x in x_min..=x_max {
    //         if let Some(point) = cave.get(&(x,y)) {
    //             print!("{point}")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }

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
        assert_eq!(24, regolith_reservoir(input_str));
    }

    #[test]
    fn test_distress_signal_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(696, regolith_reservoir(input_str));
    }

    #[bench]
    fn bench_regolith_reservoir(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            regolith_reservoir(input_str)
        })
    }
}
