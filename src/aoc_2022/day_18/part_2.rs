use std::collections::{HashSet, VecDeque};


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
    
    let x_min = cubes.iter().map(|&(x,_,_)| x).min().unwrap() - 1;
    let x_max = cubes.iter().map(|&(x,_,_)| x).max().unwrap() + 1;
    let y_min = cubes.iter().map(|&(_,y,_)| y).min().unwrap() - 1;
    let y_max = cubes.iter().map(|&(_,y,_)| y).max().unwrap() + 1;
    let z_min = cubes.iter().map(|&(_,_,z)| z).min().unwrap() - 1;
    let z_max = cubes.iter().map(|&(_,_,z)| z).max().unwrap() + 1;

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut count = 0;
    let start = (x_min, y_min, z_min);
    queue.push_back(start);

    while let Some(cur_pos) = queue.pop_front() {
        if !visited.insert(cur_pos) {
            continue;
        }
        
        for delta in deltas.iter() {
            let next_pos = (cur_pos.0 + delta.0, cur_pos.1 + delta.1, cur_pos.2 + delta.2);

            if next_pos.0 < x_min
            || next_pos.0 > x_max
            || next_pos.1 < y_min
            || next_pos.1 > y_max
            || next_pos.2 < z_min
            || next_pos.2 > z_max {
                continue;
            }

            if cubes.contains(&next_pos) {
                count += 1;
            } else {
                queue.push_back(next_pos);
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
        assert_eq!(58, boiling_boulders(input_str));
    }

    #[test]
    fn test_boiling_boulders_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(2652, boiling_boulders(input_str));
    }

    #[bench]
    fn bench_boiling_boulders(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            boiling_boulders(input_str)
        })
    }
}
