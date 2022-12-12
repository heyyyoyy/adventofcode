use std::collections::VecDeque;

use itertools::Itertools;


fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let delta: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited[start.0][start.1] = true;

    while let Some(((x, y), distance)) = queue.pop_front()  {
        if (x, y) == end {
            return Some(distance);
        }
        for (delta_x, delta_y) in delta.iter() {
            let Some(new_x) = (x as i32).checked_add(*delta_x) else{
                continue;
            };
            let Some(new_y) = (y as i32).checked_add(*delta_y) else{
                continue;
            };
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            let Some(&ch) = grid.get(new_x).and_then(|row| row.get(new_y as usize)) else {
                continue;
            };

            if (grid[x][y] as u8 + 1) as char >= ch && !visited[new_x][new_y] {
                visited[new_x][new_y] = true;
                queue.push_back(((new_x, new_y), distance + 1));
            }
        }
    }

    None
}

fn hill_climbing(input_str: &str) -> usize {

    let mut grid: Vec<Vec<char>> = input_str.lines().map(|line| line.chars().collect()).collect();

    let (x_start, y_start) = (0..grid.len()).cartesian_product(0..grid[0].len()).find(|&(x, y)| grid[x][y] == 'S').unwrap();
    let (x_end, y_end) = (0..grid.len()).cartesian_product(0..grid[0].len()).find(|&(x, y)| grid[x][y] == 'E').unwrap();
    grid[x_start][y_start] = 'a';
    grid[x_end][y_end] = 'z';

    bfs(&grid, (x_start, y_start), (x_end, y_end)).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hill_climbing() {
        let input_str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(31, hill_climbing(input_str));
    }

    #[test]
    fn test_hill_climbing_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(412, hill_climbing(input_str));
    }
}
