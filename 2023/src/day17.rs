use std::cmp::Reverse;
use std::collections::BinaryHeap;

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;

#[aoc_generator(day17)]
fn parse(input: &str) -> (Box<[Box<[u8]>]>, (usize, usize)) {
    let grid: Box<[Box<[u8]>]> = input.lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
                .into_boxed_slice()
        }).collect::<Vec<Box<[u8]>>>()
        .into_boxed_slice();

    let y_len = grid.len();
    let x_len = grid.first().unwrap().len();
    (grid, (y_len, x_len))
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    position: Point,
    direction: Option<Direction>,
    last_direction_change: u8
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn next(&self) -> [Direction; 2] {
        match self {
            Direction::North => [Direction::East, Direction::West],
            Direction::East => [Direction::South, Direction::North],
            Direction::South => [Direction::West, Direction::East],
            Direction::West => [Direction::North, Direction::South]
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    y: u8,
    x: u8
}

impl Point {
    fn new(y: u8, x: u8) -> Point {
        Point{y, x}
    }

    fn update(&self, y_len: u8, x_len: u8, direction: Direction) -> Option<Point> {
        if (direction == Direction::North && self.y == 0)
            || (direction == Direction::South && self.y == y_len - 1)
            || (direction == Direction::East && self.x == x_len - 1)
            || (direction == Direction::West && self.x == 0) {
            return None
        }
        let (y, x) = match direction {
            Direction::North => (self.y - 1, self.x),
            Direction::South => (self.y + 1, self.x),
            Direction::East => (self.y, self.x + 1),
            Direction::West => (self.y, self.x - 1)
        };
        Some(Point::new(y, x))
    }
}

fn solve(grid: &[Box<[u8]>], start: State, end: Point, same_dir: u8, size: (usize, usize), part2: bool) -> u16 {
    debug_assert!(!grid.is_empty());
    let (y_len, x_len) = size;
    let mut todo = BinaryHeap::new();
    let mut seen = FxHashMap::default();
    todo.push(Reverse((0u16, start)));
    seen.insert(start, None);

    let mut debug = vec![vec!['.'; x_len]; y_len];

    while let Some(Reverse((heat_loss, state))) = todo.pop() {
        if state.position == end && (!part2 || state.last_direction_change >= 4) {
            return heat_loss
        }
        let debug_char = &mut debug[state.position.y as usize][state.position.x as usize];
        *debug_char = match state.direction {
            Some(Direction::North) => '^',
            Some(Direction::South) => 'v',
            Some(Direction::East) => '>',
            Some(Direction::West) => '<',
            _ => '.'
        };

        let mut next_directions = Vec::with_capacity(4);
        if state.direction.is_none() {
            next_directions.extend([Direction::North, Direction::South, Direction::East, Direction::West]);
        } else if state.last_direction_change == same_dir {
            next_directions.extend(state.direction.unwrap().next());
        } else if !part2 || state.last_direction_change >= 4 {
            next_directions.extend(state.direction.unwrap().next());
            next_directions.push(state.direction.unwrap());
        } else {
            next_directions.push(state.direction.unwrap());
        }

        let neighbours: Vec<State> = next_directions.iter().filter_map(|&dir| {
            let pos = state.position.update(y_len as u8, x_len as u8, dir);
            if let Some(pos) = pos {
                return Some((pos, dir))
            }
            None
        }).map(|(pos, dir)| {
            State {
                position: pos,
                direction: Some(dir),
                last_direction_change: if Some(dir) == state.direction {state.last_direction_change + 1} else {1}
            }
        }).collect();

        for neighbour in neighbours {
            let tile = grid[neighbour.position.y as usize][neighbour.position.x as usize];

            match seen.get(&neighbour) {
                // if seen[neighbour] is a lower dist than the alternative one, we do nothing
                Some(Some(dist_next)) if heat_loss + tile as u16 >= *dist_next => {}
                // if seen[neighbour] is None then next is start and so the distance won't be changed, it won't be added again
                Some(None) => {}
                // the new path is shorter, either neighbour was not in seen or it was farther
                _ => {
                    seen.insert(neighbour, Some(heat_loss + tile as u16));
                    todo.push(Reverse((heat_loss + tile as u16, neighbour)));
                }
            }
        }
    }
    unreachable!()
}

#[aoc(day17, part1)]
fn part1(input: &(Box<[Box<[u8]>]>, (usize, usize))) -> u16 {
    let (grid, size) = input;
    let end = Point::new(size.0 as u8 - 1, size.1 as u8 - 1);
    let start = Point::new(0, 0);
    let start_state = State {
        position: start,
        direction: None,
        last_direction_change: 0
    };

    solve(grid, start_state, end, 3, *size, false)
}

#[aoc(day17, part2)]
fn part2(input: &(Box<[Box<[u8]>]>, (usize, usize))) -> u16 {
    let (grid, size) = input;
    let end = Point::new(size.0 as u8 - 1, size.1 as u8 - 1);
    let start = Point::new(0, 0);
    let start_state = State {
        position: start,
        direction: None,
        last_direction_change: 0
    };

    solve(grid, start_state, end, 10, *size, true)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const EXAMPLE_2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 102);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse(EXAMPLE)), 94);
    }
    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse(EXAMPLE_2)), 71);
    }
}