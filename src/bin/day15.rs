use regex::Regex;

static DAY: u8 = 15;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", positions_without_beacon(&input, 2000000));
    println!("{DAY}b: {}", tuning_frequency(&input, 0, 4000000));
}

#[derive(PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn distance(&self, other: &Self) -> isize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as isize
    }
}

struct SensorReading {
    sensor: Position,
    closest: Position,
    range: isize,
}

impl SensorReading {
    fn new(input: &str) -> SensorReading {
        let re = Regex::new("Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();

        let cap = re.captures(input).expect("input should match regex");
        let sensor = Position {
            x: cap[1].parse::<isize>().unwrap(),
            y: cap[2].parse::<isize>().unwrap(),
        };
        let closest = Position {
            x: cap[3].parse::<isize>().unwrap(),
            y: cap[4].parse::<isize>().unwrap(),
        };
        let range = sensor.distance(&closest);
        SensorReading { sensor, closest, range }
    }

    fn signal_in_range(&self, pos: &Position) -> bool {
        self.sensor.distance(pos) <= self.range
    }
}

fn positions_without_beacon(input: &[String], y: isize) -> usize {
    let readings = input.iter()
                        .map(|x| SensorReading::new(x))
                        .collect::<Vec<_>>();

    let min_x = readings.iter()
                        .map(|r| r.sensor.x - r.range)
                        .min().unwrap();
    let max_x = readings.iter()
                        .map(|r| r.sensor.x + r.range)
                        .max().unwrap();

    let mut possible_positions = 0;
    for x in min_x ..= max_x {
        let pos = Position { x, y };
        let in_range = readings.iter()
                               .any(|r| r.signal_in_range(&pos));
        let is_beacon = readings.iter()
                                .any(|r| r.closest == pos);
        if in_range && !is_beacon {
            possible_positions += 1;
        }
    }
    possible_positions
}

fn tuning_frequency(input: &[String], min_coord: isize, max_coord: isize) -> isize {
    let readings = input.iter()
                        .map(|x| SensorReading::new(x))
                        .collect::<Vec<_>>();

    let mut x = min_coord;
    let mut y = min_coord;
    loop {
        if x > max_coord {
            x = min_coord;
            y += 1;
        }
        let pos = Position { x, y };
        let reading = readings.iter()
                              .find(|r| r.signal_in_range(&pos));

        let reading = match reading {
            Some(r) => r,
            None => return x * 4000000 + y,
        };

        let dist_y = reading.sensor.y.abs_diff(pos.y) as isize;
        /* move position outside of range of current sensor */
        x = reading.sensor.x + reading.range + 1 - dist_y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(positions_without_beacon(&input, 10), 26);
        assert_eq!(tuning_frequency(&input, 0, 20), 56000011);
    }
}
