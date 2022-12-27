use std::collections::HashMap;

const DIR: [(i64,i64); 4] = [(0,1), (1,0), (0,-1), (-1,0)];


#[derive(Debug)]
enum Tile {
    Space,
    Wall
}

impl TryFrom<char> for Tile {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Space),
            '#' => Ok(Tile::Wall),
            _ => Err("Unsupported char")
        }
    }
}


#[derive(Debug)]
enum Move {
    Forward(i64),
    Left,
    Right
}


#[derive(Debug, Default)]
struct Map {
    tiles: HashMap<(i64,i64), Tile>,
    width: i64,
    height: i64,
    steps: Vec<Move>
}

impl Map {
    fn parse_map(&mut self, map: &str) {
        let (tiles, m) = map.lines().enumerate().fold((HashMap::new(), (i64::MIN, i64::MIN)), |(mut acc, mut pos), (row, line)| {
            for (col, ch) in line.chars().enumerate() {
                if let Ok(tile) = <char as TryInto<Tile>>::try_into(ch) {
                    acc.insert((row as i64, col as i64), tile);
                    pos = (pos.0.max(row as i64), pos.1.max(col as i64));
                }
            }
            (acc, pos)
        });

        self.tiles = tiles;
        self.height = m.0;
        self.width = m.1;
    }

    fn parse_steps(&mut self, steps: &str) {
        let mut move_steps = vec![];
        let mut st = steps.chars().peekable();
        while let Some(movement) = st.peek() {
            match movement {
                '0'..='9' => {
                    let mut num_str = String::new();
                    while let Some(dig) = st.peek() {
                        if dig.is_ascii_digit() {
                            num_str.push(st.next().unwrap());
                        } else {
                            break;
                        }
                    };
                    move_steps.push(Move::Forward(num_str.parse::<i64>().expect("Convert to digit")));
                },
                'R' => {
                    st.next();
                    move_steps.push(Move::Right)
                },
                'L' => {
                    st.next();
                    move_steps.push(Move::Left)
                },
                _ => unreachable!("Wrong move")
            }
            

        }
        self.steps = move_steps;
    }

    fn wrap_around(&self, pos: &(i64, i64), facing: usize) -> (i64, i64) {
        let mut res = *pos;
        match facing {
            0 => res.1 = 0,
            1 => res.0 = 0,
            2 => res.1 = self.width,
            3 => res.0 = self.height,
            _ => unreachable!("Wrong facing")
        }

        let dir = DIR[facing];
        while !self.tiles.contains_key(&res) {
            res = (res.0 + dir.0, res.1 + dir.1); 
        }
        match self.tiles.get(&res).unwrap() {
            Tile::Space => res,
            Tile::Wall => *pos
        }
    }
}

fn monkey_map(input_str: &str) -> i64 {
    let (map_str, steps) = input_str.split_once("\n\n").unwrap();
    let mut map = Map::default();
    map.parse_map(map_str);
    map.parse_steps(steps);

    let start_col = (0..map.width).find(|col| map.tiles.contains_key(&(0, *col))).unwrap();
    let mut pos = (0, start_col);
    let mut facing = 0;

    for step in map.steps.iter() {
        match step {
            Move::Forward(num) => {
                for _ in 0..*num {
                    let dir = DIR[facing];
                    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

                    if let Some(tile) = map.tiles.get(&next_pos) {
                        match tile {
                            Tile::Space => pos = next_pos,
                            Tile::Wall => break,
                        }
                    } else {
                        pos = map.wrap_around(&pos, facing);
                    }
                }
            },
            Move::Left => facing = (facing + DIR.len() - 1) % DIR.len(),
            Move::Right => facing = (facing + 1) % DIR.len(),
        }
    }

    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + facing as i64
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_monkey_map() {
        let input_str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        assert_eq!(6032, monkey_map(input_str));
    }

    #[test]
    fn test_monkey_map_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(133174, monkey_map(input_str));
    }

    #[bench]
    fn bench_monkey_map(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            monkey_map(input_str)
        })
    }
}
