use std::ops::{Add, Sub};

use grid::Grid;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult,
};

pub fn sand_count_before_end(input: &str) -> usize {
    let input = parse_input(input);

    let mut map = create_map(&input);

    let mut result = 0;

    loop {
        match drop_sand(&map) {
            Some(position) => map[position.x as usize][position.y as usize] = Status::Sand,
            None => break,
        };

        result += 1;
    }

    result
}

fn drop_sand(map: &Grid<Status>) -> Option<Point> {
    // We start at 500,0
    let mut position = Point { x: 500, y: 0 };

    loop {
        // We simply check at each step if it's an infinitely falling spot
        // Barbaric, but it works
        // First we check if y is at the bottom -> it has to be infinite
        if position.y as usize == map.cols() - 1
        // Secondly we check if all y below are empty
            || ((position.y as usize + 1)..map.cols())
                .map(|y| map[position.x as usize][y])
                .all(|v| v == Status::Air)
        {
            return None;
        }

        // First situation: there's air below -> we check if it's infinite, and if not, we drop 1 step
        if map[position.x as usize][position.y as usize + 1] == Status::Air {
            position.y += 1;
        }
        // Second situation: we go diagonally left
        else if map[position.x as usize - 1][position.y as usize + 1] == Status::Air {
            position.y += 1;
            position.x -= 1;
        }
        // Third situation: we go diagonally right
        else if map[position.x as usize + 1][position.y as usize + 1] == Status::Air {
            position.y += 1;
            position.x += 1;
        }
        // If we can't put sand, we return the position
        else {
            return Some(position);
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
enum Status {
    #[default]
    Air,
    Rock,
    Sand,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn len(&self) -> usize {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt() as usize
    }

    fn divide_usize(&self, factor: usize) -> Point {
        Point {
            x: self.x / factor as isize,
            y: self.y / factor as isize,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn create_map(input: &Vec<Vec<Point>>) -> Grid<Status> {
    let max_x = input
        .iter()
        .map(|s| s.iter().map(|p| p.x).max().unwrap())
        .max()
        .unwrap();

    let max_y = input
        .iter()
        .map(|s| s.iter().map(|p| p.y).max().unwrap())
        .max()
        .unwrap();

    // We assume x/y cannot go below 0, looks ok with the data set
    //  We add 10 as sand can drop diagonally... It's good enough here
    let mut map: Grid<Status> = Grid::new((max_x + 10) as usize, (max_y + 10) as usize);

    // We fill the points from the vects with Rocks
    for rock_structure in input {
        for line in rock_structure.iter().tuple_windows::<(_, _)>() {
            let (start, end) = line;

            let step_change = (*end - *start).divide_usize((*end - *start).len());

            let mut current_point = start.clone();

            // We iterate over the points between start and end
            while current_point != *end {
                map[current_point.x as usize][current_point.y as usize] = Status::Rock;

                current_point = current_point + step_change;
            }

            // We still need to add the end point
            map[end.x as usize][end.y as usize] = Status::Rock;
        }
    }

    map
}

fn parse_input(input: &str) -> Vec<Vec<Point>> {
    let (_, res) = match separated_list1(tag("\n"), parse_row)(input) {
        Ok(v) => v,
        Err(err) => panic!("Could not parse input: {input}.\nError: {err}"),
    };

    res
}

fn parse_row(row: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), parse_point)(row)
}

fn parse_point(point: &str) -> IResult<&str, Point> {
    // TODO Could that be made better with map_err?
    //  https://docs.rs/nom/latest/nom/combinator/fn.map_res.html
    map(
        separated_pair(digit1, tag(","), digit1),
        |(x, y): (&str, &str)| Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        },
    )(point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        assert_eq!(parse_point("498,4").unwrap().1, Point { x: 498, y: 4 });
    }

    #[test]
    fn test_parse_row() {
        assert_eq!(
            parse_row("498,4 -> 498,6 -> 496,6").unwrap().1,
            vec![
                Point { x: 498, y: 4 },
                Point { x: 498, y: 6 },
                Point { x: 496, y: 6 },
            ]
        );
    }

    #[test]
    fn test_create_map() {
        let input = parse_input(DEMO_INPUT);
        let map = create_map(&input);

        assert_eq!(map[498][4], Status::Rock);
        assert_eq!(map[498][5], Status::Rock);
        assert_eq!(map[498][6], Status::Rock);
        assert_eq!(map[497][6], Status::Rock);
        assert_eq!(map[496][6], Status::Rock);

        assert_eq!(map[496][7], Status::Air);
        assert_eq!(map[496][8], Status::Air);
    }

    const DEMO_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_1() {
        assert_eq!(sand_count_before_end(DEMO_INPUT), 24);
    }
}
