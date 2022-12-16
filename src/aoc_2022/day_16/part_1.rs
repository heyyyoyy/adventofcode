use std::collections::HashMap;


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

fn proboscidea_volcanium(input_str: &str) -> usize {
    let (flows, tunnels) = get_flow_and_tunnels(input_str);

    dbg!(&flows, &tunnels);
    todo!()
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
        assert_eq!(0, proboscidea_volcanium(input_str));
    }

    #[test]
    #[ignore = "reason"]
    fn test_beacon_exclusion_zone_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(5176944, proboscidea_volcanium(input_str));
    }

    #[bench]
    #[ignore = "reason"]
    fn bench_beacon_exclusion_zone(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            proboscidea_volcanium(input_str)
        })
    }
}
