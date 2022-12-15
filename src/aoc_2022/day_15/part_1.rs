use std::collections::HashSet;


#[derive(Debug)]
struct Sensor {
    sensor: (i32, i32),
    beacon: (i32, i32)
}

impl Sensor {
    fn get_sensors(input_str: &str) -> Vec<Sensor> {
        let mut sensors = vec![];
        for line in input_str.lines() {
            let (sensor_str, beacon_str) = line.split_once(":").unwrap();
            let points = sensor_str.split("at ").nth(1).unwrap();
            let (x_str, y_str) = points.split_once(", ").unwrap();
            let sensor = (
                x_str.split('=').nth(1).unwrap().parse::<i32>().unwrap(), 
                y_str.split('=').nth(1).unwrap().parse::<i32>().unwrap()
            );

            let points = beacon_str.split("at ").nth(1).unwrap();
            let (x_str, y_str) = points.split_once(", ").unwrap();
            let beacon = (
                x_str.split('=').nth(1).unwrap().parse::<i32>().unwrap(), 
                y_str.split('=').nth(1).unwrap().parse::<i32>().unwrap()
            );
            sensors.push(Sensor {sensor, beacon});
        }
        sensors
    }
}


fn beacon_exclusion_zone(input_str: &str, row_y: i32) -> usize {
    let mut coverage = HashSet::new();
    let sensors = Sensor::get_sensors(input_str);
    for sensor in sensors.iter() {
        let radius = (sensor.sensor.0 - sensor.beacon.0).abs() + (sensor.sensor.1 - sensor.beacon.1).abs();
        let dist = (sensor.sensor.1 - row_y).abs();
        if dist > radius {
            continue;
        }
        let target_line = radius - dist;
        let x_min = sensor.sensor.0 - target_line;
        let x_max = sensor.sensor.0 + target_line;
        coverage.extend(x_min..=x_max);
    }
    let beacon_count = sensors.iter().filter_map(|num| {
        if num.beacon.1 == row_y {
            Some(num.beacon.1)
        } else {
            None
        }}).collect::<HashSet<_>>().iter().count();
    coverage.len() - beacon_count
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_beacon_exclusion_zone() {
        let input_str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert_eq!(26, beacon_exclusion_zone(input_str, 10));
    }

    #[test]
    fn test_beacon_exclusion_zone_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(5176944, beacon_exclusion_zone(input_str, 2000000));
    }

    #[bench]
    fn bench_beacon_exclusion_zone(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            beacon_exclusion_zone(input_str, 2000000)
        })
    }
}
