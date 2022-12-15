use ndarray::Array2;
use nom::{
    bytes::complete::{is_not, tag, take_until},
    multi::separated_list1,
    IResult,
};

pub fn no_beacon_count(input: &str, row: isize) -> isize {
    let input = parse_input(input);

    // TODO This is actually very similar to Grid
    let map = Array2::<Location>::default((1000, 1000));

    todo!()
}

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    let (_, rows) = match separated_list1(tag("\n"), parse_row)(input) {
        Ok(r) => r,
        Err(err) => panic!("Could not parse input: {input}.\nError: {err}"),
    };

    rows
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Point {
    x: isize,
    y: isize,
}

/// Returns sensor and beacon as points
fn parse_row(row: &str) -> IResult<&str, (Point, Point)> {
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
            Point {
                x: x_sensor.parse().unwrap(),
                y: y_sensor.parse().unwrap(),
            },
            Point {
                x: x_beacon.parse().unwrap(),
                y: y_beacon.parse().unwrap(),
            },
        ),
    ))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Location {
    #[default]
    Unkown,
    Beacon,
    NoBeacon,
    Sensor,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        assert_eq!(
            parse_row("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Ok(("", (Point { x: 2, y: 18 }, Point { x: -2, y: 15 })))
        )
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            vec![(Point { x: 2, y: 18 }, Point { x: -2, y: 15 })]
        )
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
