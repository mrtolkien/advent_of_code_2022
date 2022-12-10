use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Motion {
    direction: Direction,
    distance: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

pub fn get_visited_positions_short_rope(input: &str) -> usize {
    let motions = get_motions(input);

    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };
    let mut visited_locations = HashSet::new();

    visited_locations.insert(tail_position.clone());

    for motion in motions {
        // TODO Learn how to *not* modify the objects!
        for _ in 0..motion.distance {
            move_once_short(&mut head_position, &mut tail_position, motion.direction);
            visited_locations.insert(tail_position.clone());
        }
    }

    visited_locations.len()
}

fn move_once_short(
    head_position: &mut Position,
    tail_position: &mut Position,
    direction: Direction,
) {
    // We first move the head
    match direction {
        Direction::Up => head_position.y += 1,
        Direction::Right => head_position.x += 1,
        Direction::Left => head_position.x -= 1,
        Direction::Down => head_position.y -= 1,
    }

    // We move the tail if the distance to the head in any dimension is >= 2
    if (head_position.x - tail_position.x).abs() >= 2
        || (head_position.y - tail_position.y).abs() >= 2
    {
        // When a move happens, the non-advancing coordinate will always be the same as the head
        match direction {
            Direction::Up => {
                tail_position.y += 1;
                tail_position.x = head_position.x;
            }
            Direction::Right => {
                tail_position.x += 1;
                tail_position.y = head_position.y
            }
            Direction::Left => {
                tail_position.x -= 1;
                tail_position.y = head_position.y
            }
            Direction::Down => {
                tail_position.y -= 1;
                tail_position.x = head_position.x;
            }
        }
    }
}

pub fn get_visited_positions_long_rope(input: &str) -> usize {
    let motions = get_motions(input);

    // We make it an array because we know its lenght!
    let mut rope = [Position { x: 0, y: 0 }; 10];

    let mut visited_locations = HashSet::new();

    // We can access our rope by index since it's an array
    visited_locations.insert(rope[9].clone());

    for motion in motions {
        // TODO Same as above, learn how to *not* modify the objects -> re-assign
        for _ in 0..motion.distance {
            move_once_long(&mut rope, motion.direction);

            visited_locations.insert(rope[9].clone());
        }
    }

    visited_locations.len()
}

fn move_once_long(rope: &mut [Position; 10], direction: Direction) {
    // We start by moving the head
    match direction {
        Direction::Up => rope[0].y += 1,
        Direction::Right => rope[0].x += 1,
        Direction::Left => rope[0].x -= 1,
        Direction::Down => rope[0].y -= 1,
    }

    for i in 0..9 {
        // The head and tail are two consecutive nodes
        let head = rope[i];
        // We make that a mutable reference
        let tail = &mut rope[i + 1];

        // Going right
        if head.x - tail.x == 2 {
            if head.y > tail.y {
                tail.y += 1;
            } else if head.y < tail.y {
                tail.y -= 1;
            }

            tail.x += 1;

        // Going left
        } else if head.x - tail.x == -2 {
            if head.y > tail.y {
                tail.y += 1;
            } else if head.y < tail.y {
                tail.y -= 1;
            }

            tail.x -= 1;

        // Going up
        } else if head.y - tail.y == 2 {
            if head.x > tail.x {
                tail.x += 1;
            } else if head.x < tail.x {
                tail.x -= 1;
            }

            tail.y += 1;

        // Going down
        } else if head.y - tail.y == -2 {
            if head.x > tail.x {
                tail.x += 1;
            } else if head.x < tail.x {
                tail.x -= 1;
            }

            tail.y -= 1;
        }
    }
}

fn get_motions(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_once(' ').unwrap();

            let distance = distance.parse::<i32>().unwrap();
            let direction = match direction {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "L" => Direction::Left,
                "D" => Direction::Down,
                _ => panic!("Unknown direction"),
            };

            Motion {
                direction,
                distance,
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_first_part() {
        assert_eq!(get_visited_positions_short_rope(INPUT), 13);
    }

    #[test]
    fn test_get_motions() {
        assert_eq!(
            get_motions("R 4"),
            vec![Motion {
                direction: Direction::Right,
                distance: 4
            }]
        );

        assert_eq!(
            get_motions("D 200"),
            vec![Motion {
                direction: Direction::Down,
                distance: 200
            }]
        );
    }

    #[test]
    #[should_panic]
    fn test_get_motions_panic() {
        get_motions("W 4");
    }

    #[test]
    fn test_move_rope_short() {
        let mut head_position = Position { x: 0, y: 0 };
        let mut tail_position = Position { x: 0, y: 0 };

        move_once_short(&mut head_position, &mut tail_position, Direction::Right);
        move_once_short(&mut head_position, &mut tail_position, Direction::Right);
        move_once_short(&mut head_position, &mut tail_position, Direction::Right);
        move_once_short(&mut head_position, &mut tail_position, Direction::Right);

        assert_eq!(head_position, Position { x: 4, y: 0 });
        assert_eq!(tail_position, Position { x: 3, y: 0 });

        move_once_short(&mut head_position, &mut tail_position, Direction::Up);

        assert_eq!(head_position, Position { x: 4, y: 1 });
        assert_eq!(tail_position, Position { x: 3, y: 0 });

        move_once_short(&mut head_position, &mut tail_position, Direction::Up);

        assert_eq!(head_position, Position { x: 4, y: 2 });
        assert_eq!(tail_position, Position { x: 4, y: 1 });
    }

    #[test]
    fn test_move_rope_long() {
        let mut rope = [Position { x: 0, y: 0 }; 10];

        move_once_long(&mut rope, Direction::Right);
        move_once_long(&mut rope, Direction::Right);
        move_once_long(&mut rope, Direction::Right);
        move_once_long(&mut rope, Direction::Right);

        assert_eq!(rope[0], Position { x: 4, y: 0 });
        assert_eq!(rope[1], Position { x: 3, y: 0 });
        assert_eq!(rope[9], Position { x: 0, y: 0 });

        move_once_long(&mut rope, Direction::Up);

        assert_eq!(rope[0], Position { x: 4, y: 1 });
        assert_eq!(rope[1], Position { x: 3, y: 0 });

        move_once_long(&mut rope, Direction::Up);

        // Cf test example:
        // ......
        // ......
        // ....H.
        // .4321.
        // 5.....  (5 covers 6, 7, 8, 9, s)
        assert_eq!(rope[0], Position { x: 4, y: 2 });
        assert_eq!(rope[1], Position { x: 4, y: 1 });
        assert_eq!(rope[2], Position { x: 3, y: 1 });
        assert_eq!(rope[3], Position { x: 2, y: 1 });
        assert_eq!(rope[4], Position { x: 1, y: 1 });
        assert_eq!(rope[5], Position { x: 0, y: 0 });
    }
}
