use geo::Point;
use itertools::all;
use nom::{
    bytes::complete::{is_not, tag, take_until},
    multi::separated_list1,
    IResult,
};

pub fn no_beacon_count(input: &str, row: isize) -> usize {
    let input = parse_input(input);

    let (max_x, min_x, max_y, min_y) = get_max_relevant_coordinates(&input);

    // Simple case where the row is irrelevant
    if row < min_y || row > max_y {
        return 0;
    }

    let mut result = 0;

    for x in min_x..max_x {
        for (sensor, beacon) in &input {
            let sensor_max_dist = manhattan_distance(sensor, beacon);
            let point_dist = manhattan_distance(sensor, &Point::new(x, row));

            if point_dist <= sensor_max_dist {
                result += 1;
                // We stop (a point can only appear once)
                break;
            }
        }

        // TODO We need to remove actual beacons... That's a bit stupid code but it works
        for (_, beacon) in &input {
            if beacon.eq(&Point::new(x, row)) {
                result -= 1;
                break;
            }
        }
    }

    result
}

pub fn get_tuning_frequency(input: &str, search_size: usize) -> usize {
    let input = parse_input(input);

    for x in 0..search_size {
        for y in 0..search_size {
            if all(&input, |(sensor, beacon)| {
                let sensor_max_dist = manhattan_distance(sensor, beacon);
                let point_dist = manhattan_distance(sensor, &Point::new(x as isize, y as isize));

                point_dist > sensor_max_dist
            }) {
                return 4_000_000 * x + y;
            }
        }
    }

    unreachable!("No solution found")
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

fn get_max_relevant_coordinates(
    input: &Vec<(Point<isize>, Point<isize>)>,
) -> (isize, isize, isize, isize) {
    input
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
        .unwrap()
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

    #[test]
    fn test_part_2() {
        assert_eq!(get_tuning_frequency(DEMO_INPUT, 20), 56_000_011)
    }
}
