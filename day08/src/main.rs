use std::collections::{HashMap, HashSet};

use itertools::Itertools as _;

type Nodes = HashMap<char, Vec<(i8, i8)>>;

fn parse() -> (Nodes, i8, i8) {
    let mut nodes: Nodes = HashMap::new();
    let mut w = None;
    let mut h = None;

    for (y, line) in std::io::stdin().lines().enumerate() {
        for (x, char) in line.unwrap().chars().enumerate() {
            match char {
                '0'..='9' | 'A'..='Z' | 'a'..='z' => {
                    nodes
                        .entry(char)
                        .or_default()
                        .push((x.try_into().unwrap(), y.try_into().unwrap()));
                }
                _ => {}
            }
            w = Some((x + 1).try_into().unwrap());
        }
        h = Some((y + 1).try_into().unwrap());
    }
    (nodes, w.unwrap(), h.unwrap())
}

fn part1(nodes: &Nodes, w: i8, h: i8) {
    let mut antinodes = HashSet::new();
    for positions in nodes.values() {
        for (a, b) in positions.iter().tuple_combinations() {
            let dx = b.0 - a.0;
            let dy = b.1 - a.1; // guaranteed to be non-negative

            let antinode1_x = (0..w).contains(&(a.0 - dx)).then_some(a.0 - dx);
            let antinode1_y = (0..h).contains(&(a.1 - dy)).then_some(a.1 - dy);
            let antinode2_x = (0..w).contains(&(b.0 + dx)).then_some(b.0 + dx);
            let antinode2_y = (0..h).contains(&(b.1 + dy)).then_some(b.1 + dy);
            if let (Some(x), Some(y)) = (antinode1_x, antinode1_y) {
                antinodes.insert((x, y));
            }
            if let (Some(x), Some(y)) = (antinode2_x, antinode2_y) {
                antinodes.insert((x, y));
            }
        }
    }
    println!("{}", antinodes.len());
}

fn part2(nodes: &Nodes, w: i8, h: i8) {
    let mut antinodes = HashSet::new();

    for positions in nodes.values() {
        for (a, b) in positions.iter().tuple_combinations() {
            let dx = b.0 - a.0;
            let dy = b.1 - a.1; // guaranteed to be non-negative

            for i in 0.. {
                let x = a.0 - i * dx;
                let y = a.1 - i * dy;

                if !(0..w).contains(&x) || !(0..h).contains(&y) {
                    break;
                }

                antinodes.insert((x, y));
            }

            for i in 0.. {
                let x = b.0 + i * dx;
                let y = b.1 + i * dy;

                if !(0..w).contains(&x) || !(0..h).contains(&y) {
                    break;
                }

                antinodes.insert((x, y));
            }
        }
    }

    println!("{}", antinodes.len());
}

fn main() {
    let (nodes, w, h) = parse();
    part1(&nodes, w, h);
    part2(&nodes, w, h);
}
