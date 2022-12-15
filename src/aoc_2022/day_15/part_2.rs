
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


fn beacon_exclusion_zone(input_str: &str, max_row: i32) -> i64 {
    let sensors = Sensor::get_sensors(input_str);
    for y in 0..=max_row {
        let mut ranges = vec![0..=max_row];
        for sensor in sensors.iter() {
            let radius = (sensor.sensor.0 - sensor.beacon.0).abs() + (sensor.sensor.1 - sensor.beacon.1).abs();
            let top = 0.max(sensor.sensor.1 - radius);
            let bottom = max_row.min(sensor.sensor.1 + radius);

            if top > y || bottom < y {
                continue;
            }
            let dist = (sensor.sensor.1 - y).abs();
            let target_line = radius - dist;
            let x_min = 0.max(sensor.sensor.0 - target_line);
            let x_max = max_row.min(sensor.sensor.0 + target_line);

            let mut new_ranges = vec![];
            for rng in ranges.iter() {
                let start = *rng.start();
                if start > x_max {
                    new_ranges.push(rng.clone());
                    continue;
                }
                let end = *rng.end();
                if end < x_min {
                    new_ranges.push(rng.clone());
                    continue;
                }

                if start < x_min {
                    new_ranges.push(start..=x_min - 1);
                }
                if end > x_max {
                    new_ranges.push(x_max + 1..=end);
                }
            }
            ranges = new_ranges;
        }
        if !ranges.is_empty() {
            let x = *ranges[0].start() as i64;
            return x * 4000000 + y as i64;
        }
    }
    todo!()
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
        assert_eq!(56000011, beacon_exclusion_zone(input_str, 20));
    }

    #[test]
    fn test_beacon_exclusion_zone_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(13350458933732, beacon_exclusion_zone(input_str, 4000000));
    }

    #[bench]
    fn bench_beacon_exclusion_zone(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            beacon_exclusion_zone(input_str, 4000000)
        })
    }
}
