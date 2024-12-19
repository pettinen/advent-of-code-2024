use std::collections::{HashMap, HashSet};

const MAP_SIZE: u8 = 71;

fn parse() -> Vec<(u8, u8)> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (a, b) = line.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn score_n(x: u8, y: u8, distances: &HashMap<(u8, u8), u32>) -> Option<u32> {
    if y > 0 {
        distances.get(&(x, y - 1)).copied()
    } else {
        None
    }
}

fn score_s(x: u8, y: u8, distances: &HashMap<(u8, u8), u32>) -> Option<u32> {
    if y < MAP_SIZE - 1 {
        distances.get(&(x, y + 1)).copied()
    } else {
        None
    }
}

fn score_w(x: u8, y: u8, distances: &HashMap<(u8, u8), u32>) -> Option<u32> {
    if x > 0 {
        distances.get(&(x - 1, y)).copied()
    } else {
        None
    }
}

fn score_e(x: u8, y: u8, distances: &HashMap<(u8, u8), u32>) -> Option<u32> {
    if x < MAP_SIZE - 1 {
        distances.get(&(x + 1, y)).copied()
    } else {
        None
    }
}

fn new_score(x: u8, y: u8, distances: &HashMap<(u8, u8), u32>) -> u32 {
    [
        score_n(x, y, distances),
        score_s(x, y, distances),
        score_w(x, y, distances),
        score_e(x, y, distances),
    ]
    .into_iter()
    .flatten()
    .min()
    .unwrap()
        + 1
}

fn distance_to_start(corrupted: &[(u8, u8)], corrupted_len: usize) -> Option<u32> {
    let corrupted = corrupted
        .iter()
        .copied()
        .take(corrupted_len)
        .collect::<HashSet<_>>();
    let mut distances = HashMap::new();
    distances.insert((MAP_SIZE - 1, MAP_SIZE - 1), 0);

    while distances.len() < usize::from(MAP_SIZE) * usize::from(MAP_SIZE) - corrupted_len {
        let mut new_scores = Vec::with_capacity(4);

        for &(x, y) in distances.keys() {
            if corrupted.contains(&(x, y)) {
                continue;
            }
            if y > 0 && !distances.contains_key(&(x, y - 1)) && !corrupted.contains(&(x, y - 1)) {
                new_scores.push(((x, y - 1), new_score(x, y - 1, &distances)));
            }
            if y < MAP_SIZE - 1
                && !distances.contains_key(&(x, y + 1))
                && !corrupted.contains(&(x, y + 1))
            {
                new_scores.push(((x, y + 1), new_score(x, y + 1, &distances)));
            }
            if x > 0 && !distances.contains_key(&(x - 1, y)) && !corrupted.contains(&(x - 1, y)) {
                new_scores.push(((x - 1, y), new_score(x - 1, y, &distances)));
            }
            if x < MAP_SIZE - 1
                && !distances.contains_key(&(x + 1, y))
                && !corrupted.contains(&(x + 1, y))
            {
                new_scores.push(((x + 1, y), new_score(x + 1, y, &distances)));
            }
        }

        if new_scores.is_empty() {
            break;
        }
        distances.extend(new_scores);
    }
    distances.get(&(0, 0)).copied()
}

fn part1(corrupted: &[(u8, u8)]) {
    println!("{}", distance_to_start(corrupted, 1024).unwrap());
}

fn part2(corrupted: &[(u8, u8)]) {
    let mut min = 1024;
    let mut max = corrupted.len();

    // binary search
    loop {
        let mut mid = (min + max) / 2;
        if (min + max) % 2 == 1 {
            mid += 1;
        }

        if distance_to_start(corrupted, mid).is_none() {
            max = mid - 1;
        } else {
            min = mid;
        }

        if min == max {
            break;
        }
    }

    println!("{},{}", corrupted[min].0, corrupted[min].1);
}

fn main() {
    let corrupted = parse();
    part1(&corrupted);
    part2(&corrupted);
}
