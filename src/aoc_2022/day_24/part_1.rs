use std::collections::{HashSet, VecDeque};
use num::integer::lcm;


const ARROW: &'static str = "^v<>";
const DIR: [(i64,i64); 4] = [(-1,0), (1,0), (0,-1), (0,1)];

fn parse(input_str: &str) -> HashSet<(i64,i64,i64)> {
    input_str
    .lines()
    .enumerate()
    .flat_map(|(row, line)| {
        line.chars().enumerate().filter_map(|(col, ch)| {
            if let Some(idx) = ARROW.find(ch) {
                Some((row as i64, col as i64, idx as i64))
            } else {
                None
            }
        }).collect::<HashSet<_>>()
    })
    .collect::<HashSet<_>>()
}

fn is_bliz(new_pos: &(i64,i64), state: &HashSet<(i64,i64,i64)>) -> bool {
    for dir in 0..4 {
        if state.contains(&(new_pos.0, new_pos.1, dir)) {
            return true;
        }
    }
    false
}

fn draw(
    state: &HashSet<(i64,i64,i64)>, 
    row_len: i64,
    col_len: i64,
    start_pos: &(i64,i64),
    end_pos: &(i64,i64)
) {
    for row in 0..row_len {
        for col in 0..col_len {
            if (row, col) == *start_pos {
                print!("E");
            }
            else if (row, col) == *end_pos {
                print!(".");
            }
            else if  row == 0 
                || row == row_len - 1
                || col == 0
                || col == col_len - 1
            {
                print!("#");    
                    
            } else {
                let mut draw = false;
                let mut arrow = ' ';
                let mut count = 0;
                for d in 0..4 {
                    if state.contains(&(row, col, d)) {
                        count += 1;
                        arrow = ARROW.chars().nth(d as usize).unwrap();
                        draw = true;
                    }
                    
                }
                if draw {
                    if count == 1 {
                        print!("{arrow}");
                    } else {
                        print!("{count}");
                    }
                } else {
                    print!(".");
                }
                
                
            }
        }
        println!();
    }
}

fn blizzard_basin(input_str: &str) -> usize {
    let mut blizzards = parse(input_str);

    let row_len = input_str.lines().count();
    let col_len = input_str.lines().next().unwrap().len();

    let start_pos = (0,1);
    let end_pos = (row_len as i64 - 1, col_len as i64 - 2);

    let period = lcm(row_len, col_len);

    let mut states = Vec::with_capacity(period);
    states.push(blizzards.clone());

    for _ in 1..period {
        let mut new_bliz = HashSet::new();

        for bliz in blizzards.iter() {
            let (row, col, dir) = *bliz;
            let (delta_row, delta_col) = DIR[dir as usize];
            let (mut new_row, mut new_col) = (row + delta_row, col + delta_col);

            if new_row == 0 {
                new_row = row_len as i64 - 2;
            } else if new_row == row_len as i64 - 1 {
                new_row = 1;
            }

            if new_col == 0 {
                new_col = col_len as i64 - 2;
            } else if new_col == col_len as i64 - 1 {
                new_col = 1;
            }

            new_bliz.insert((new_row, new_col, dir));
            
        }
        states.push(new_bliz.clone());
        blizzards = new_bliz;
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut count = 0;

    queue.push_back((0, start_pos));
    while let Some(state) = queue.pop_front() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        let (time, cur_pos) = state;

        if (cur_pos.0, cur_pos.1) == end_pos {
            count = time;
            break;
        }
        for (delta_row, delta_col) in DIR.iter().chain([&(0,0)]) {
            let new_pos = (cur_pos.0 + delta_row, cur_pos.1 + delta_col);

            if !(new_pos == start_pos || new_pos == end_pos) &&
            !(new_pos.0 >= 1
            && new_pos.0 <= row_len as i64 - 2
            && new_pos.1 >= 1 
            && new_pos.1 <= col_len as i64 - 2)
            {
                continue;
            }

            let st = states.get((time + 1) % period).unwrap();
            if is_bliz(&new_pos, st) {
                continue;
            }
            queue.push_back((time + 1, new_pos));
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
    fn test_blizzard_basin() {
        let input_str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
        assert_eq!(18, blizzard_basin(input_str));
    }

    #[test]
    fn test_blizzard_basin_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(326, blizzard_basin(input_str));
    }

    #[bench]
    fn bench_blizzard_basin(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            blizzard_basin(input_str)
        })
    }
}
