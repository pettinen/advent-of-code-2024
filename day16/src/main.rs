use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    End,
    Open,
    Wall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

type Position = (usize, usize);

fn parse() -> (HashMap<Position, Square>, Position, Position) {
    let mut map = HashMap::new();
    let mut start_pos = None;
    let mut end_pos = None;

    for (y, line) in std::io::stdin().lines().enumerate() {
        for (x, char) in line.unwrap().chars().enumerate() {
            match char {
                'S' => {
                    start_pos = Some((x, y));
                    map.insert((x, y), Square::Open);
                }
                'E' => {
                    end_pos = Some((x, y));
                    map.insert((x, y), Square::End);
                }
                '.' => {
                    map.insert((x, y), Square::Open);
                }
                '#' => {
                    map.insert((x, y), Square::Wall);
                }
                char => panic!("unexpected character '{char}'"),
            }
        }
    }
    (map, start_pos.unwrap(), end_pos.unwrap())
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Square>>) {
    //std::thread::sleep(std::time::Duration::from_millis(50));
    print!("\x1B[1;1H");
    for line in map {
        for square in line {
            match square {
                Square::End => print!("E"),
                Square::Open => print!("."),
                Square::Wall => print!("#"),
            }
        }
        println!();
    }
}

fn populate_scores(
    map: &HashMap<Position, Square>,
    scores: &mut HashMap<(usize, usize, Direction), u32>,
) {
    let keys = scores
        .keys()
        .filter_map(|&(x, y, _)| if x > 0 && y > 0 { Some((x, y)) } else { None })
        .collect::<Vec<_>>();
    for (x, y) in keys {
        if map.get(&(x, y - 1)) == Some(&Square::Open) {
            let (mut score_n, mut score_s, mut score_w, mut score_e) = {
                let score = scores.get(&(x, y, Direction::South)).unwrap();
                (score + 2001, score + 1, score + 1001, score + 1001)
            };

            if let Some(score) = scores.get(&(x, y - 2, Direction::North)) {
                score_n = score_n.min(score + 1);
                score_s = score_s.min(score + 2001);
                score_w = score_w.min(score + 1001);
                score_e = score_e.min(score + 1001);
            }

            if let Some(score) = scores.get(&(x - 1, y - 1, Direction::West)) {
                score_n = score_n.min(score + 1001);
                score_s = score_s.min(score + 1001);
                score_w = score_w.min(score + 1);
                score_e = score_e.min(score + 2001);
            }

            if let Some(score) = scores.get(&(x + 1, y - 1, Direction::East)) {
                score_n = score_n.min(score + 1001);
                score_s = score_s.min(score + 1001);
                score_w = score_w.min(score + 2001);
                score_e = score_e.min(score + 1);
            }

            scores.insert((x, y - 1, Direction::North), score_n);
            scores.insert((x, y - 1, Direction::South), score_s);
            scores.insert((x, y - 1, Direction::West), score_w);
            scores.insert((x, y - 1, Direction::East), score_e);
        }

        if map.get(&(x, y + 1)) == Some(&Square::Open) {
            let (mut score_n, mut score_s, mut score_w, mut score_e) = {
                let score = scores.get(&(x, y, Direction::North)).unwrap();
                (score + 1, score + 2001, score + 1001, score + 1001)
            };

            if let Some(score) = scores.get(&(x, y + 2, Direction::South)) {
                score_n = score_n.min(score + 2001);
                score_s = score_s.min(score + 1);
                score_w = score_w.min(score + 1001);
                score_e = score_e.min(score + 1001);
            }

            if let Some(score) = scores.get(&(x - 1, y + 1, Direction::West)) {
                score_n = score_n.min(score + 1001);
                score_s = score_s.min(score + 1001);
                score_w = score_w.min(score + 1);
                score_e = score_e.min(score + 2001);
            }

            if let Some(score) = scores.get(&(x + 1, y + 1, Direction::East)) {
                score_n = score_n.min(score + 1001);
                score_s = score_s.min(score + 1001);
                score_w = score_w.min(score + 2001);
                score_e = score_e.min(score + 1);
            }

            scores.insert((x, y + 1, Direction::North), score_n);
            scores.insert((x, y + 1, Direction::South), score_s);
            scores.insert((x, y + 1, Direction::West), score_w);
            scores.insert((x, y + 1, Direction::East), score_e);
        }

        if map.get(&(x - 1, y)) == Some(&Square::Open) {
            let (mut score_n, mut score_s, mut score_w, mut score_e) = {
                let score = scores.get(&(x, y, Direction::East)).unwrap();
                (score + 1001, score + 1001, score + 2001, score + 1)
            };

            if let Some(score) = scores.get(&(x - 1, y - 1, Direction::North)) {
                score_n = score_n.min(score + 1);
                score_s = score_s.min(score + 2001);
                score_w = score_w.min(score + 1001);
                score_e = score_e.min(score + 1001);
            }

            if let Some(score) = scores.get(&(x - 1, y + 1, Direction::South)) {
                score_n = score_n.min(score + 2001);
                score_s = score_s.min(score + 1);
                score_w = score_w.min(score + 1001);
                score_e = score_e.min(score + 1001);
            }

            if let Some(score) = scores.get(&(x - 2, y, Direction::West)) {
                score_n = score_n.min(score + 1001);
                score_s = score_s.min(score + 1001);
                score_w = score_w.min(score + 1);
                score_e = score_e.min(score + 2001);
            }

            scores.insert((x - 1, y, Direction::North), score_n);
            scores.insert((x - 1, y, Direction::South), score_s);
            scores.insert((x - 1, y, Direction::West), score_w);
            scores.insert((x - 1, y, Direction::East), score_e);
        }

        if map.get(&(x + 1, y)) == Some(&Square::Open) {
            let (mut score_n, mut score_s, mut score_w, mut score_e) = {
                let score = scores.get(&(x, y, Direction::West)).unwrap();
                (score + 1001, score + 1001, score + 1, score + 2001)
            };

            if let Some(score) = scores.get(&(x + 1, y - 1, Direction::North)) {
                score_n = score_n.min(score + 1);
                score_s = score_s.min(score + 2001);
                score_w = score_w.min(score + 1001);
                score_e = score_e.min(score + 1001);
            }

            if let Some(score) = scores.get(&(x + 1, y + 1, Direction::South)) {
                score_n = score_n.min(score + 2001);
                score_s = score_s.min(score + 1);
                score_w = score_w.min(score + 1001);
                score_e = score_e.min(score + 1001);
            }

            if let Some(score) = scores.get(&(x + 2, y, Direction::East)) {
                score_n = score_n.min(score + 1001);
                score_s = score_s.min(score + 1001);
                score_w = score_w.min(score + 2001);
                score_e = score_e.min(score + 1);
            }

            scores.insert((x + 1, y, Direction::North), score_n);
            scores.insert((x + 1, y, Direction::South), score_s);
            scores.insert((x + 1, y, Direction::West), score_w);
            scores.insert((x + 1, y, Direction::East), score_e);
        }
    }
}

#[allow(dead_code)]
fn print_scores(map: &HashMap<Position, Square>, scores: &HashMap<(usize, usize, Direction), u32>) {
    //std::thread::sleep(std::time::Duration::from_millis(10));
    print!("\x1B[1;1H\n\n\n\n");
    for y in 0..=141 {
        print!(" ");
        for x in 0..=141 {
            if map.get(&(x, y)) == Some(&Square::Wall) {
                print!("\u{2588}");
            } else if scores.contains_key(&(x, y, Direction::North)) {
                print!("\u{2592}");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn part1(map: &HashMap<Position, Square>, start_pos: Position, end_pos: Position) {
    let mut scores = HashMap::new();
    scores.insert((end_pos.0, end_pos.1, Direction::North), 0);
    scores.insert((end_pos.0, end_pos.1, Direction::South), 0);
    scores.insert((end_pos.0, end_pos.1, Direction::West), 0);
    scores.insert((end_pos.0, end_pos.1, Direction::East), 0);

    while scores.len()
        < 4 * map
            .values()
            .filter(|square| matches!(square, Square::Open | Square::End))
            .count()
    {
        populate_scores(map, &mut scores);
    }

    let score = scores
        .get(&(start_pos.0, start_pos.1, Direction::East))
        .unwrap();
    println!("{score}");
}

fn get_next_best_paths(
    scores: &HashMap<(usize, usize, Direction), u32>,
    x: usize,
    y: usize,
    direction: Direction,
) -> Vec<(usize, usize, Direction)> {
    if scores.get(&(x, y, direction)) == Some(&0) {
        return Vec::new();
    }

    let (score_n, score_s, score_w, score_e) = match direction {
        Direction::North => (
            scores
                .get(&(x, y - 1, Direction::North))
                .map(|score| score + 1),
            scores
                .get(&(x, y + 1, Direction::South))
                .map(|score| score + 2001),
            scores
                .get(&(x - 1, y, Direction::West))
                .map(|score| score + 1001),
            scores
                .get(&(x + 1, y, Direction::East))
                .map(|score| score + 1001),
        ),
        Direction::South => (
            scores
                .get(&(x, y - 1, Direction::North))
                .map(|score| score + 2001),
            scores
                .get(&(x, y + 1, Direction::South))
                .map(|score| score + 1),
            scores
                .get(&(x - 1, y, Direction::West))
                .map(|score| score + 1001),
            scores
                .get(&(x + 1, y, Direction::East))
                .map(|score| score + 1001),
        ),
        Direction::West => (
            scores
                .get(&(x, y - 1, Direction::North))
                .map(|score| score + 1001),
            scores
                .get(&(x, y + 1, Direction::South))
                .map(|score| score + 1001),
            scores
                .get(&(x - 1, y, Direction::West))
                .map(|score| score + 1),
            scores
                .get(&(x + 1, y, Direction::East))
                .map(|score| score + 2001),
        ),
        Direction::East => (
            scores
                .get(&(x, y - 1, Direction::North))
                .map(|score| score + 1001),
            scores
                .get(&(x, y + 1, Direction::South))
                .map(|score| score + 1001),
            scores
                .get(&(x - 1, y, Direction::West))
                .map(|score| score + 2001),
            scores
                .get(&(x + 1, y, Direction::East))
                .map(|score| score + 1),
        ),
    };

    let min = [score_n, score_s, score_w, score_e]
        .into_iter()
        .flatten()
        .min()
        .unwrap();

    let mut results = Vec::new();

    if score_n == Some(min) {
        results.push((x, y - 1, Direction::North));
    }
    if score_s == Some(min) {
        results.push((x, y + 1, Direction::South));
    }
    if score_w == Some(min) {
        results.push((x - 1, y, Direction::West));
    }
    if score_e == Some(min) {
        results.push((x + 1, y, Direction::East));
    }
    results
}

fn part2(map: &HashMap<Position, Square>, start_pos: Position, end_pos: Position) {
    let mut scores = HashMap::new();
    scores.insert((end_pos.0, end_pos.1, Direction::North), 0);
    scores.insert((end_pos.0, end_pos.1, Direction::South), 0);
    scores.insert((end_pos.0, end_pos.1, Direction::West), 0);
    scores.insert((end_pos.0, end_pos.1, Direction::East), 0);

    while scores.len()
        < 4 * map
            .values()
            .filter(|square| matches!(square, Square::Open | Square::End))
            .count()
    {
        populate_scores(map, &mut scores);
    }

    let mut best_paths = HashSet::new();
    best_paths.insert(start_pos);
    let mut current_best_paths = vec![(start_pos.0, start_pos.1, Direction::East)];
    loop {
        current_best_paths = current_best_paths
            .into_iter()
            .flat_map(|(x, y, direction)| get_next_best_paths(&scores, x, y, direction))
            .collect();
        if current_best_paths.is_empty() {
            break;
        }
        best_paths.extend(
            current_best_paths
                .clone()
                .into_iter()
                .map(|(x, y, _)| (x, y)),
        );
    }

    println!("{}", best_paths.len());
}

fn main() {
    let (map, start_pos, end_pos) = parse();
    part1(&map, start_pos, end_pos);
    part2(&map, start_pos, end_pos);
}
