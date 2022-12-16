use std::collections::{HashMap, BTreeSet};


fn get_flow_and_tunnels(input_str: &str) -> (HashMap<String, i32>, HashMap<String, Vec<String>>) {
    let mut flows = HashMap::new();
    let mut tunnels = HashMap::new();
    for line in input_str.lines() {
        let (valve_str, lead_to_str) = line.split_once("; ").unwrap();
        let valve_name = valve_str[6..8].to_string();
        let flow_rate = valve_str.split('=').nth(1).unwrap().parse::<i32>().unwrap();
        flows.insert(valve_name.clone(), flow_rate);

        let tunnel = if let Some(tunnels_str) = lead_to_str.split("valves ").nth(1) {
            tunnels_str.split(", ").map(|s| s.to_string()).collect::<Vec<String>>()
        } else {
            lead_to_str.split("valve ").nth(1).unwrap().split(", ").map(|s| s.to_string()).collect::<Vec<String>>()
        };

        tunnels.insert(valve_name, tunnel);
    }
    (flows, tunnels)
}

fn dfs(
    valve: String,
    flows: &HashMap<String, i32>,
    tunnels: &HashMap<String, Vec<String>>,
    time: i32,
    visited: &mut BTreeSet<String>,
    cache: &mut HashMap<(String, BTreeSet<String>, i32), i32>
) -> i32 {
    if time == 0 {
        return 0;
    }
    let key = (valve.clone(), visited.clone(), time);
    let v = cache.get(&key);
    
    if v.is_some() {
        return *v.unwrap();
    }

    let mut max_pressure = 0;

    let flow = *flows.get(&valve).unwrap();
    let tun = tunnels.get(&valve).unwrap();

    if !visited.contains(&valve) && flow > 0 {
        let mut cur_visited = visited.clone();
        cur_visited.insert(valve.clone());
       
        max_pressure = max_pressure.max(
            visited.iter().map(|v| *flows.get(v).unwrap()).sum::<i32>() + dfs(
                valve.clone(), flows, tunnels, time - 1, &mut cur_visited, cache
            )
        );
    }
    for t in tun {
        max_pressure = max_pressure.max(
            visited.iter().map(|v| *flows.get(v).unwrap()).sum::<i32>() + dfs(
                t.clone(), flows, tunnels, time - 1, visited, cache
            )
        );
    }
    cache.insert(key, max_pressure);

    max_pressure
}


fn proboscidea_volcanium(input_str: &str) -> i32 {
    let time = 30;
    let mut visited: BTreeSet<String> = BTreeSet::new();
    let mut cache: HashMap<(String, BTreeSet<String>, i32), i32> = HashMap::new();
    let (flows, tunnels) = get_flow_and_tunnels(input_str);

    let max_pressure = dfs("AA".to_owned(), &flows, &tunnels, time, &mut visited, &mut cache);

    max_pressure
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_proboscidea_volcanium() {
        let input_str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(1651, proboscidea_volcanium(input_str));
    }

    #[test]
    fn test_proboscidea_volcanium_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(1751, proboscidea_volcanium(input_str));
    }

    #[bench]
    fn bench_proboscidea_volcanium(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            proboscidea_volcanium(input_str)
        })
    }
}
