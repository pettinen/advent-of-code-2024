use std::collections::HashSet;

use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};

#[derive(Clone, Copy, Debug)]
enum Square {
    NotVisited,
    Visited,
    Obstacle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug)]
struct State {
    map: Vec<Vec<Square>>,
    position: (usize, usize),
    direction: Direction,
}

impl State {
    fn advance(&mut self) -> bool {
        let next_position = match self.direction {
            Direction::Up => {
                if self.position.1 == 0 {
                    return false;
                }
                (self.position.0, self.position.1 - 1)
            }
            Direction::Right => {
                if self.position.0 == self.map[0].len() - 1 {
                    return false;
                }
                (self.position.0 + 1, self.position.1)
            }
            Direction::Down => {
                if self.position.1 == self.map.len() - 1 {
                    return false;
                }
                (self.position.0, self.position.1 + 1)
            }
            Direction::Left => {
                if self.position.0 == 0 {
                    return false;
                }
                (self.position.0 - 1, self.position.1)
            }
        };

        match self.map[next_position.1][next_position.0] {
            Square::NotVisited | Square::Visited => {
                self.map[next_position.1][next_position.0] = Square::Visited;
                self.position = next_position;
            }
            Square::Obstacle => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
        }
        true
    }
}

fn parse() -> State {
    let mut position = None;

    let map = std::io::stdin()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '.' => Square::NotVisited,
                    '^' => {
                        position = Some((x, y));
                        Square::Visited
                    }
                    '#' => Square::Obstacle,
                    _ => panic!("unexpected character"),
                })
                .collect()
        })
        .collect();

    State {
        map,
        position: position.unwrap(),
        direction: Direction::Up,
    }
}

fn visited(mut state: State) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    visited.insert(state.position);
    while state.advance() {
        visited.insert(state.position);
    }
    visited
}

fn part1(state: State) {
    println!("{}", visited(state).len());
}

fn part2(original_state: State) {
    let obstacle_positions = {
        let mut obstacle_positions = visited(original_state.clone());
        obstacle_positions.remove(&original_state.position);
        obstacle_positions
    };

    let sum: u32 = obstacle_positions
        .into_par_iter()
        .map(|(x, y)| {
            match original_state.map[y][x] {
                Square::Obstacle | Square::Visited => return 0,
                Square::NotVisited => {}
            }

            let mut state = original_state.clone();
            let mut visited = HashSet::new();
            state.map[y][x] = Square::Obstacle;

            while state.advance() {
                if visited.contains(&(state.position, state.direction)) {
                    return 1;
                }
                visited.insert((state.position, state.direction));
            }
            0
        })
        .sum();

    println!("{sum}");
}

fn main() {
    let state = parse();

    part1(state.clone());
    part2(state);
}
