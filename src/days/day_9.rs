use advent_of_code_2022::to_u32;
use std::ops::Add;
use std::{
    collections::HashSet,
    ops::AddAssign,
    str::FromStr,
};
use Direction::{Down, Left, Right, Up};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coords {
    x: i32,
    y: i32,
}

impl Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Coords) -> Coords {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Coords) {
        *self = *self + rhs;
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(()),
        }
    }
}

struct Movement {
    direction: Direction,
    steps: u32,
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction_str = &s[..1];
        let steps_str = &s[2..];

        let direction = Direction::from_str(direction_str);
        let steps = to_u32(steps_str);

        direction.map(|d| Movement {
            direction: d,
            steps: steps,
        })
    }
}

fn head_movement(direction: Direction) -> Coords {
    match direction {
        Up => Coords { x: 0, y: 1 },
        Down => Coords { x: 0, y: -1 },
        Left => Coords { x: -1, y: 0 },
        Right => Coords { x: 1, y: 0 },
    }
}

fn tail_movement(head_pos: Coords, tail_pos: Coords) -> Coords {
    let x_dist = head_pos.x - tail_pos.x;
    let y_dist = head_pos.y - tail_pos.y;

    let head_is_within_one = x_dist.abs() <= 1 && y_dist.abs() <= 1;
    let head_is_within_two =
        (x_dist.abs() <= 2 && y_dist.abs() <= 2) || (y_dist.abs() <= 2 && x_dist.abs() <= 2);

    if head_is_within_one {
        return Coords { x: 0, y: 0 }; // no movement required
    } else if head_is_within_two {
        let capped_x = (-1).max(x_dist.min(1));
        let capped_y = (-1).max(y_dist.min(1));
        return Coords {
            x: capped_x,
            y: capped_y,
        };
    } else {
        panic!(
            "Head [{:?}] and Tail [{:?}] are separated by an invalid distance",
            head_pos, tail_pos
        );
    }
}

pub fn unique_coords_visited_by_tail(input: &str, rope_length: usize) -> u32 {
    let movements: Vec<Movement> = input
        .lines()
        .map(|m| Movement::from_str(m).unwrap())
        .collect();

    let mut knot_positions: Vec<Coords> = Vec::new();
    for _ in 0..rope_length {
        knot_positions.push(Coords { x: 0, y: 0});
    }

    let mut visited_coords: HashSet<Coords> = HashSet::from([*knot_positions.last().unwrap()]);
    for movement in movements {
        for _ in 0..movement.steps {
            knot_positions[0] += head_movement(movement.direction);
            for idx in 1..knot_positions.len() {
                let movement = tail_movement(knot_positions[idx - 1], knot_positions[idx]);
                knot_positions[idx] += movement;
            }
            visited_coords.insert(knot_positions[rope_length - 1]);
        }
    }
    visited_coords.len().try_into().unwrap()

}

pub fn part_a(input: &str) -> u32 {
    unique_coords_visited_by_tail(input, 2)
}

pub fn part_b(input: &str) -> u32 {
    unique_coords_visited_by_tail(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_tail_movement() {
        let same_pos = tail_movement(Coords { x: 0, y: 0 }, Coords { x: 0, y: 0 });
        assert_eq!(same_pos, Coords { x: 0, y: 0 });

        let tail_below = tail_movement(Coords { x: 0, y: 0 }, Coords { x: 0, y: -1 });
        assert_eq!(tail_below, Coords { x: 0, y: 0 });

        let head_right = tail_movement(Coords { x: 1, y: 0 }, Coords { x: 0, y: 0 });
        assert_eq!(head_right, Coords { x: 0, y: 0 });

        let tail_diag = tail_movement(Coords { x: 1, y: 1 }, Coords { x: 0, y: 0 });
        assert_eq!(tail_diag, Coords { x: 0, y: 0 });

        let head_two_left = tail_movement(Coords { x: -2, y: 0 }, Coords { x: 0, y: 0 });
        assert_eq!(head_two_left, Coords { x: -1, y: 0 });

        let head_two_right = tail_movement(Coords { x: 2, y: 0 }, Coords { x: 0, y: 0 });
        assert_eq!(head_two_right, Coords { x: 1, y: 0 });

        let head_one_down_and_two_left =
            tail_movement(Coords { x: -1, y: 0 }, Coords { x: 1, y: 1 });
        assert_eq!(head_one_down_and_two_left, Coords { x: -1, y: -1 });

        let head_two_up_and_one_left = tail_movement(Coords { x: -1, y: 2 }, Coords { x: 0, y: 0 });
        assert_eq!(head_two_up_and_one_left, Coords { x: -1, y: 1 });
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(9);
        assert_eq!(part_a(&input), 88);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(9);
        assert_eq!(part_b(&input), 36);
    }
}
