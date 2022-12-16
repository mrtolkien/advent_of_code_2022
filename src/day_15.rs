use geo::Point;
use ndarray::Array2;
use nom::{
    bytes::complete::{is_not, tag, take_until},
    multi::separated_list1,
    IResult,
};

pub fn no_beacon_count(input: &str, row: isize) -> usize {
    let input = parse_input(input);

    let mut map = NegativeMap::new_from_input(&input);

    for (sensor, beacon) in input {
        map.set(sensor.x(), sensor.y(), Location::Sensor);
        map.set(beacon.x(), beacon.y(), Location::Beacon);

        let dist = manhattan_distance(&sensor, &beacon);

        for x in sensor.x() - dist..sensor.x() + dist {
            // TODO It's possible to iterate directly on the right y here
            for y in sensor.y() - dist..sensor.y() + dist {
                if manhattan_distance(&sensor, &Point::new(x, y)) <= dist {
                    if map.get(x, y) == Location::Unknown {
                        map.set(x, y, Location::NoBeacon);
                    }
                }
            }
        }
    }

    map.row(row)
        .iter()
        .filter(|&&location| location != Location::Unknown)
        .count()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Location {
    #[default]
    Unknown,
    Beacon,
    NoBeacon,
    Sensor,
}

/// A custom 2D grid that supports negative indexes
struct NegativeMap {
    array: Array2<Location>,
    offset: isize,
}

impl NegativeMap {
    fn new(size: usize) -> Self {
        Self {
            array: Array2::<Location>::default((size as usize, size as usize)),
            offset: size as isize / 2,
        }
    }

    fn new_from_input(input: &Vec<(Point<isize>, Point<isize>)>) -> Self {
        // We first get the maximum point in the input
        let (max_x, min_x, max_y, min_y) = input
            .iter()
            .map(|(sensor, beacon)| {
                let dist = manhattan_distance(sensor, beacon);
                (
                    // Min and maximum x and y that are covered by our sensor
                    sensor.x() + dist,
                    sensor.x() - dist,
                    sensor.y() + dist,
                    sensor.y() - dist,
                )
            })
            // Reduce to the maximum values
            .reduce(|(x1, x2, y1, y2), (x3, x4, y3, y4)| {
                (x1.max(x3), x2.min(x4), y1.max(y3), y2.min(y4))
            })
            .unwrap();

        // The max size we'll need is the biggest coordinate span
        let max_size = (max_x - min_x).max(max_y - min_y) as usize;

        // We *2 it to allow for positive and negative values
        NegativeMap::new(max_size * 2)
    }

    fn get(&self, x: isize, y: isize) -> Location {
        self.array[((x + self.offset) as usize, (y + self.offset) as usize)]
    }

    fn row(&self, y: isize) -> ndarray::ArrayView1<Location> {
        self.array.row((y + self.offset) as usize)
    }

    fn set(&mut self, x: isize, y: isize, location: Location) {
        self.array[((x + self.offset) as usize, (y + self.offset) as usize)] = location;
    }
}

/// Returns sensor -> beacon info
fn parse_input(input: &str) -> Vec<(Point<isize>, Point<isize>)> {
    let (_, rows) = match separated_list1(tag("\n"), parse_row)(input) {
        Ok(r) => r,
        Err(err) => panic!("Could not parse input: {input}.\nError: {err}"),
    };

    rows
}

/// Returns sensor and beacon as points
fn parse_row(row: &str) -> IResult<&str, (Point<isize>, Point<isize>)> {
    let (row, _) = tag("Sensor at x=")(row)?;
    let (row, x_sensor) = take_until(",")(row)?;
    let (row, _) = tag(", y=")(row)?;
    let (row, y_sensor) = take_until(":")(row)?;
    let (row, _) = tag(": closest beacon is at x=")(row)?;
    let (row, x_beacon) = take_until(",")(row)?;
    let (row, _) = tag(", y=")(row)?;
    let (row, y_beacon) = is_not("\n")(row)?;

    Ok((
        row,
        (
            Point::new(x_sensor.parse().unwrap(), y_sensor.parse().unwrap()),
            Point::new(x_beacon.parse().unwrap(), y_beacon.parse().unwrap()),
        ),
    ))
}

fn manhattan_distance(p1: &Point<isize>, p2: &Point<isize>) -> isize {
    (p1.x() - p2.x()).abs() + (p1.y() - p2.y()).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        assert_eq!(
            parse_row("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Ok(("", (Point::new(2, 18), Point::new(-2, 15))))
        )
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            vec![(Point::new(2, 18), Point::new(-2, 15))]
        )
    }

    #[test]
    fn test_negative_map() {
        let mut map = NegativeMap::new(14);

        assert_eq!(map.get(0, 0), Location::Unknown);
        assert_eq!(map.get(-5, -5), Location::Unknown);

        map.set(-5, -5, Location::Beacon);

        assert_eq!(map.get(-5, -5), Location::Beacon);
        assert_eq!(map.array[(2, 2)], Location::Beacon);
    }

    const DEMO_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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

    #[test]
    fn test_part_1() {
        assert_eq!(no_beacon_count(DEMO_INPUT, 10), 26);
    }
}
