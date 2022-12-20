use std::collections::{HashSet, VecDeque};


fn not_enough_minerals(input_str: &str) -> i32 {
    let mut total = 1;
    for blueprint in input_str.lines().take(3) {
        let mut blueprint_iter = blueprint.split(':');
        
        let mut blueprint_iter = blueprint_iter.nth(1).unwrap().split('.');

        let ore_cost = blueprint_iter.next().unwrap().trim().split(' ').nth(4).unwrap().parse::<i32>().unwrap();

        let clay_cost = blueprint_iter.next().unwrap().trim().split(' ').nth(4).unwrap().parse::<i32>().unwrap();

        let mut obsidian = blueprint_iter.next().unwrap().trim().split(' ');
        let obsidian_cost_ore = obsidian.nth(4).unwrap().parse::<i32>().unwrap();
        let obsidian_cost_clay = obsidian.nth(2).unwrap().parse::<i32>().unwrap();

        let mut geode = blueprint_iter.next().unwrap().trim().split(' ');
        let geode_cost_ore = geode.nth(4).unwrap().parse::<i32>().unwrap();
        let geode_cost_obs = geode.nth(2).unwrap().parse::<i32>().unwrap();

        let max_time = 32;
        let mut best_geode_count = 0;
        let mut visited = HashSet::new();
        // ore, clay, obsidian, geodes, robot_ore, robot_clay, robot_obs, robot_geodes, time
        let start = (0,0,0,0,1,0,0,0,max_time);
        let mut queue = VecDeque::from([start]);

        while let Some(mut state) = queue.pop_front() {
            let (mut ore, mut clay, mut obsidian, geodes, mut robot_ore, mut robot_clay, mut robot_obs, robot_geodes, time) = state;
            
            best_geode_count = best_geode_count.max(geodes);
            
            if time == 0 {
                continue;
            }

            let max_ore_cost = [ore_cost, clay_cost, obsidian_cost_ore, geode_cost_ore].into_iter().max().unwrap();
            if robot_ore >= max_ore_cost {
                robot_ore = max_ore_cost;
            }
            if robot_clay >= obsidian_cost_clay {
                robot_clay = obsidian_cost_clay;
            }
            if robot_obs >= geode_cost_obs {
                robot_obs = geode_cost_obs;
            }
            if ore >= time * max_ore_cost - robot_ore * (time - 1) {
                ore = time * max_ore_cost - robot_ore * (time - 1);
            }
            if clay >= time * obsidian_cost_clay - robot_clay * (time - 1) {
                clay = time * obsidian_cost_clay - robot_clay * (time - 1);
            }
            if obsidian >= time * geode_cost_obs - robot_obs * (time - 1) {
                obsidian = time * geode_cost_obs - robot_obs * (time - 1);
            }
            state = (ore, clay, obsidian, geodes, robot_ore, robot_clay, robot_obs, robot_geodes, time);

            if visited.contains(&state) {
                continue;
            }
            visited.insert(state);

            queue.push_back((
                ore+robot_ore, 
                clay+robot_clay, 
                obsidian+robot_obs, 
                geodes+robot_geodes, 
                robot_ore, 
                robot_clay, 
                robot_obs, 
                robot_geodes, 
                time - 1
            ));
            if ore >= ore_cost {
                queue.push_back((
                    ore-ore_cost+robot_ore, 
                    clay+robot_clay,
                    obsidian+robot_obs, 
                    geodes+robot_geodes, 
                    robot_ore+1, 
                    robot_clay, 
                    robot_obs, 
                    robot_geodes, 
                    time - 1
                ));
            }
            if ore >= clay_cost {
                queue.push_back((
                    ore-clay_cost+robot_ore, 
                    clay+robot_clay, 
                    obsidian+robot_obs, 
                    geodes+robot_geodes,
                    robot_ore, 
                    robot_clay+1, 
                    robot_obs, 
                    robot_geodes, 
                    time - 1
                ));
            }
            if ore >= obsidian_cost_ore && clay >= obsidian_cost_clay {
                queue.push_back((
                    ore-obsidian_cost_ore+robot_ore, 
                    clay-obsidian_cost_clay+robot_clay, 
                    obsidian+robot_obs, 
                    geodes+robot_geodes, 
                    robot_ore, 
                    robot_clay, 
                    robot_obs+1, 
                    robot_geodes, 
                    time - 1
                ));
            }
            if ore >= geode_cost_ore && obsidian >= geode_cost_obs {
                queue.push_back((
                    ore-geode_cost_ore+robot_ore, 
                    clay+robot_clay, 
                    obsidian-geode_cost_obs+robot_obs, 
                    geodes+robot_geodes, 
                    robot_ore, 
                    robot_clay, 
                    robot_obs, 
                    robot_geodes+1, 
                    time - 1
                ));
            }
        }
        
        total *= best_geode_count;
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_not_enough_minerals() {
        let input_str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(3472, not_enough_minerals(input_str));
    }

    #[test]
    fn test_not_enough_minerals_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(13475, not_enough_minerals(input_str));
    }

    #[bench]
    fn bench_not_enough_minerals(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            not_enough_minerals(input_str)
        })
    }
}
