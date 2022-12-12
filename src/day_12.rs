use std::collections::VecDeque;

use grid::Grid;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn get_accessible_neighbours(&self, grid: Grid<usize>) -> Vec<Location> {
        let mut neighbours = Vec::new();
        let location_value = grid[self.x][self.y];

        if self.x > 0 && location_value + 1 >= grid[self.x - 1][self.y] {
            neighbours.push(Location {
                x: self.x - 1,
                y: self.y,
            });
        }

        if self.x < grid.rows() - 1 && location_value + 1 >= grid[self.x + 1][self.y] {
            neighbours.push(Location {
                x: self.x + 1,
                y: self.y,
            });
        }

        if self.y > 0 && location_value + 1 >= grid[self.x][self.y - 1] {
            neighbours.push(Location {
                x: self.x,
                y: self.y - 1,
            });
        }

        if self.y < grid.cols() - 1 && location_value + 1 >= grid[self.x][self.y + 1] {
            neighbours.push(Location {
                x: self.x,
                y: self.y + 1,
            });
        }

        neighbours
    }
}

#[derive(Clone)]
struct Map {
    points: Grid<usize>,
    start: Location,
    end: Location,
}

pub fn get_minimum_steps_count(input: &str) -> usize {
    // Notes:
    // - We need to find the shortest path between all the points
    // - Coming back on your steps cannot be right
    let map = parse_map(input);

    // Now, we just need to get all parents from the end
    get_shortest_path_length(&map).unwrap()
}

pub fn get_best_starting_position_step_count(input: &str) -> usize {
    let mut map = parse_map(input);
    let mut possible_starts = Vec::new();

    for x in 0..map.points.rows() {
        for y in 0..map.points.cols() {
            if map.points[x][y] == 0 {
                possible_starts.push(Location { x, y });
            }
        }
    }

    possible_starts
        .iter()
        // We need a filter map and an Option result in case there's no possible path
        .filter_map(|start| {
            map.start = *start;
            get_shortest_path_length(&map)
        })
        .min()
        .unwrap()
}

fn get_shortest_path_length(map: &Map) -> Option<usize> {
    // We do the BFS which returns the best parent for each point
    let parents = breadth_first_search(&map);

    let mut current_point = map.end;
    let mut steps = 0;

    while let Some(location) = parents[current_point.x][current_point.y] {
        current_point = location;
        steps += 1;

        if location == map.start {
            break;
        }
    }

    // If there's no route from the end, we return None
    if steps == 0 {
        None
    } else {
        Some(steps)
    }
}

fn breadth_first_search(map: &Map) -> Grid<Option<Location>> {
    // We do a BFS: https://en.wikipedia.org/wiki/Breadth-first_search#Pseudocode
    //  *IE* at each step we explore all options to know the shortest path to all points
    // The queue, needs to be Deque so we can add new points last
    let mut queue = VecDeque::from(vec![map.start]);

    // This is a way to know which nodes have been explored + their parent
    let mut parents: Grid<Option<Location>> = Grid::new(map.points.rows(), map.points.cols());
    while let Some(location) = queue.pop_front() {
        for neighbour in location.get_accessible_neighbours(map.points.clone()) {
            // If we have not explored this node yet
            if parents[neighbour.x][neighbour.y].is_none() {
                // We add a clone of it to the queue, at the back
                queue.push_back(neighbour);
                // We set its parent
                parents[neighbour.x][neighbour.y] = Some(location);
            }
        }
    }
    parents
}

fn parse_map(input: &str) -> Map {
    let mut map = Map {
        points: Grid::new(input.lines().count(), input.lines().next().unwrap().len()),
        start: Location { x: 0, y: 0 },
        end: Location { x: 0, y: 0 },
    };

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'a'..='z' => {
                    let idx = c as usize - 'a' as usize;
                    map.points[row][col] = idx;
                }
                'S' => {
                    map.points[row][col] = 0;
                    map.start = Location { x: row, y: col };
                }
                'E' => {
                    map.points[row][col] = 25;
                    map.end = Location { x: row, y: col };
                }
                _ => panic!("Invalid character: {}", c),
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_create_map() {
        let map = parse_map(DEMO_INPUT);

        // Start
        assert_eq!(map.start, Location { x: 0, y: 0 });
        assert_eq!(map.points[0][0], 0);

        // End
        assert_eq!(map.end, Location { x: 2, y: 5 });
        assert_eq!(map.points[2][5], 25);

        // Random point
        assert_eq!(map.points[1][2], 2);
    }

    #[test]
    fn test_minimize_steps() {
        assert_eq!(get_minimum_steps_count(DEMO_INPUT), 31)
    }

    #[test]
    fn test_find_best_start() {
        assert_eq!(get_best_starting_position_step_count(DEMO_INPUT), 29)
    }
}
