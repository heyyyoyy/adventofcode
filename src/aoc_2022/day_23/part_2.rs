use std::collections::{HashSet, HashMap};

const DIR: [(i64,i64); 8] = [
        (0,-1), (0,1), (-1,0), (1,0), (-1,-1), (1,-1), (-1,1), (1,1)
];

const CHECK_DIR: [[(i64,i64); 3]; 4] = [
    [(-1,0), (-1,-1), (-1,1)],
    [(1,0), (1,-1), (1,1)],
    [(0,-1), (-1,-1), (1,-1)],
    [(0,1), (-1,1), (1,1)]
];


#[derive(Debug, Default)]
struct Grove {
    elves : Vec<(i64,i64)>,
    dir: i64,
    moved: bool
}

impl Grove {

    fn parse(input_str: &str) -> Self {
        let elves = input_str
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, ch)| {
                if ch == '#' {
                    Some((row as i64, col as i64))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<(i64,i64)>>();
        Self { elves, ..Default::default() }
    }

    fn get_coordinates(&self) -> (i64,i64,i64,i64) {
        self.elves.iter().fold((i64::MAX, i64::MIN, i64::MAX, i64::MIN), |coords, &cur| {
            (coords.0.min(cur.0), coords.1.max(cur.0), coords.2.min(cur.1), coords.3.max(cur.1))
        })
    }

    fn draw(&self) {
        let (rmin, rmax, col_min, col_max) = self.get_coordinates();
        for r in rmin..=rmax {
            for c in col_min..=col_max {
                if self.elves.contains(&(r,c)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn round(&mut self) {
        let mut round_res = HashSet::new();
        let mut visited = HashMap::new();
        for el in self.elves.iter() {
            let count = DIR
            .iter()
            .filter(|&&delta| self.elves.contains(&(el.0 + delta.0, el.1 + delta.1)))
            .count();

            if count == 0 {
                round_res.insert(*el);
            } else {
                self.moved = true;
                let mut is_sibling = false;
                for i in 0..4 {
                    let count = CHECK_DIR[(i + self.dir) as usize % CHECK_DIR.len()]
                    .iter()
                    .filter(|&&delta| self.elves.contains(&(el.0 + delta.0, el.1 + delta.1)))
                    .count();
                    if count == 0 {
                        let delta = CHECK_DIR[(i + self.dir) as usize % CHECK_DIR.len()][0];
                        let new_position = (el.0 + delta.0, el.1 + delta.1);

                        let old = visited.entry(new_position).or_insert((*el, 0));
                        old.1 += 1;
                        if old.1 != 1 {
                            round_res.insert(*el);
                        }
                        
                        is_sibling = true;
                        break;
                    }
                }

                if !is_sibling {
                    round_res.insert(*el);
                }
            }
        }
        for (new_pos, (old_pos, count)) in visited {
            if count == 1 {
                round_res.insert(new_pos);
            } else {
                round_res.insert(old_pos);
            }
        }
        self.elves = round_res.into_iter().collect::<Vec<_>>();
        self.dir = (self.dir + 1) % DIR.len() as i64;
    }
}


fn unstable_diffusion(input_str: &str) -> usize {
    let mut grove = Grove::parse(input_str);
    let mut count = 0;
    loop {
        println!("Count {count}");
        count += 1;
        grove.round();
        if !grove.moved {break;}
        grove.moved = false;
        
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_unstable_diffusion() {
        let input_str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        assert_eq!(20, unstable_diffusion(input_str));
    }

    #[test]
    #[ignore = "slow"]
    fn test_unstable_diffusion_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(1057, unstable_diffusion(input_str));
    }

    #[bench]
    fn bench_unstable_diffusion(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            unstable_diffusion(input_str)
        })
    }
}
