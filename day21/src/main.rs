use std::collections::HashMap;

use itertools::Itertools;

fn parse() -> Vec<(String, u16)> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let numeric = line[..3].parse().unwrap();
            (line, numeric)
        })
        .collect()
}

fn numpad_paths() -> HashMap<(char, char), Vec<&'static str>> {
    HashMap::from([
        (('0', '0'), vec![""]),
        (('0', '1'), vec!["^<"]),
        (('0', '2'), vec!["^"]),
        (('0', '3'), vec!["^>", ">^"]),
        (('0', '4'), vec!["^^<"]),
        (('0', '5'), vec!["^^"]),
        (('0', '6'), vec!["^^>", ">^^"]),
        (('0', '7'), vec!["^^^<"]),
        (('0', '8'), vec!["^^^"]),
        (('0', '9'), vec!["^^^>", ">^^^"]),
        (('0', 'A'), vec![">"]),
        (('1', '0'), vec![">v"]),
        (('1', '1'), vec![""]),
        (('1', '2'), vec![">"]),
        (('1', '3'), vec![">>"]),
        (('1', '4'), vec!["^"]),
        (('1', '5'), vec!["^>", ">^"]),
        (('1', '6'), vec!["^>>", ">>^"]),
        (('1', '7'), vec!["^^"]),
        (('1', '8'), vec!["^^>", ">^^"]),
        (('1', '9'), vec!["^^>>", ">>^^"]),
        (('1', 'A'), vec![">>v"]),
        (('2', '0'), vec!["v"]),
        (('2', '1'), vec!["<"]),
        (('2', '2'), vec![""]),
        (('2', '3'), vec![">"]),
        (('2', '4'), vec!["^<", "<^"]),
        (('2', '5'), vec!["^"]),
        (('2', '6'), vec!["^>", ">^"]),
        (('2', '7'), vec!["^^<", "<^^"]),
        (('2', '8'), vec!["^^"]),
        (('2', '9'), vec!["^^>", ">^^"]),
        (('2', 'A'), vec!["v>", ">v"]),
        (('3', '0'), vec!["v<", "<v"]),
        (('3', '1'), vec!["<<"]),
        (('3', '2'), vec!["<"]),
        (('3', '3'), vec![""]),
        (('3', '4'), vec!["^<<", "<<^"]),
        (('3', '5'), vec!["^<", "<^"]),
        (('3', '6'), vec!["^"]),
        (('3', '7'), vec!["^^<<", "<<^^"]),
        (('3', '8'), vec!["^^<", "<^^"]),
        (('3', '9'), vec!["^^"]),
        (('3', 'A'), vec!["v"]),
        (('4', '0'), vec![">vv"]),
        (('4', '1'), vec!["v"]),
        (('4', '2'), vec!["v>", ">v"]),
        (('4', '3'), vec!["v>>", ">>v"]),
        (('4', '4'), vec![""]),
        (('4', '5'), vec![">"]),
        (('4', '6'), vec![">>"]),
        (('4', '7'), vec!["^"]),
        (('4', '8'), vec!["^>", ">^"]),
        (('4', '9'), vec!["^>>", ">>^"]),
        (('4', 'A'), vec![">>vv"]),
        (('5', '0'), vec!["vv"]),
        (('5', '1'), vec!["v<", "<v"]),
        (('5', '2'), vec!["v"]),
        (('5', '3'), vec!["v>", ">v"]),
        (('5', '4'), vec!["<"]),
        (('5', '5'), vec![""]),
        (('5', '6'), vec![">"]),
        (('5', '7'), vec!["^<", "<^"]),
        (('5', '8'), vec!["^"]),
        (('5', '9'), vec!["^>", ">^"]),
        (('5', 'A'), vec!["vv>", ">vv"]),
        (('6', '0'), vec!["vv<", "<vv"]),
        (('6', '1'), vec!["v<<", "<<v"]),
        (('6', '2'), vec!["v<", "<v"]),
        (('6', '3'), vec!["v"]),
        (('6', '4'), vec!["<<"]),
        (('6', '5'), vec!["<"]),
        (('6', '6'), vec![""]),
        (('6', '7'), vec!["^<<", "<<^"]),
        (('6', '8'), vec!["^<", "<^"]),
        (('6', '9'), vec!["^"]),
        (('6', 'A'), vec!["vv"]),
        (('7', '0'), vec![">vvv"]),
        (('7', '1'), vec!["vv"]),
        (('7', '2'), vec!["vv>", ">vv"]),
        (('7', '3'), vec!["vv>>", ">>vv"]),
        (('7', '4'), vec!["v"]),
        (('7', '5'), vec!["v>", ">v"]),
        (('7', '6'), vec!["v>>", ">>v"]),
        (('7', '7'), vec![""]),
        (('7', '8'), vec![">"]),
        (('7', '9'), vec![">>"]),
        (('7', 'A'), vec![">>vvv"]),
        (('8', '0'), vec!["vvv"]),
        (('8', '1'), vec!["vv<", "<vv"]),
        (('8', '2'), vec!["vv"]),
        (('8', '3'), vec!["vv>", ">vv"]),
        (('8', '4'), vec!["v<", "<v"]),
        (('8', '5'), vec!["v"]),
        (('8', '6'), vec!["v>", ">v"]),
        (('8', '7'), vec!["<"]),
        (('8', '8'), vec![""]),
        (('8', '9'), vec![">"]),
        (('8', 'A'), vec!["vvv>", ">vvv"]),
        (('9', '0'), vec!["vvv<", "<vvv"]),
        (('9', '1'), vec!["vv<<", "<<vv"]),
        (('9', '2'), vec!["vv<", "<vv"]),
        (('9', '3'), vec!["vv"]),
        (('9', '4'), vec!["v<<", "<<v"]),
        (('9', '5'), vec!["v<", "<v"]),
        (('9', '6'), vec!["v"]),
        (('9', '7'), vec!["<<"]),
        (('9', '8'), vec!["<"]),
        (('9', '9'), vec![""]),
        (('9', 'A'), vec!["vvv"]),
        (('A', '0'), vec!["<"]),
        (('A', '1'), vec!["^<<"]),
        (('A', '2'), vec!["^<", "<^"]),
        (('A', '3'), vec!["^"]),
        (('A', '4'), vec!["^^<<"]),
        (('A', '5'), vec!["^^<", "<^^"]),
        (('A', '6'), vec!["^^"]),
        (('A', '7'), vec!["^^^<<"]),
        (('A', '8'), vec!["^^^<", "<^^^"]),
        (('A', '9'), vec!["^^^"]),
        (('A', 'A'), vec![""]),
    ])
}

fn numpad_paths_for_code(code: &str, paths: &HashMap<(char, char), Vec<&str>>) -> Vec<String> {
    let mut chars = Vec::with_capacity(code.len() + 1);
    chars.push('A');
    chars.extend(code.chars());

    (0..chars.len() - 1)
        .map(|i| paths.get(&(chars[i], chars[i + 1])).unwrap())
        .multi_cartesian_product()
        .map(|path_segments| path_segments.into_iter().join("A") + "A")
        .collect()
}

fn empty_counts() -> HashMap<(char, char), u64> {
    let directional_chars = ['^', 'v', '<', '>', 'A'];
    let mut counts = HashMap::with_capacity(30);

    for &char1 in &directional_chars {
        // space character signifies the start of the directions
        counts.insert((' ', char1), 0);
        for &char2 in &directional_chars {
            counts.insert((char1, char2), 0);
        }
    }
    counts
}

fn counts(directions: &str) -> HashMap<(char, char), u64> {
    let mut counts = empty_counts();
    let mut chars = Vec::with_capacity(directions.len() + 1);
    chars.push(' ');
    chars.extend(directions.chars());

    for i in 0..chars.len() - 1 {
        *counts.get_mut(&(chars[i], chars[i + 1])).unwrap() += 1;
    }
    counts
}

fn next_counts(counts: &HashMap<(char, char), u64>) -> HashMap<(char, char), u64> {
    let mut new = empty_counts();

    for (chars, count) in counts {
        match chars {
            (' ', '^') => {
                *new.get_mut(&(' ', '<')).unwrap() += count;
                *new.get_mut(&('<', 'A')).unwrap() += count;
            }
            (' ', 'v') => {
                *new.get_mut(&(' ', '<')).unwrap() += count;
                *new.get_mut(&('<', 'v')).unwrap() += count;
                *new.get_mut(&('v', 'A')).unwrap() += count;
            }
            (' ', '<') => {
                *new.get_mut(&(' ', 'v')).unwrap() += count;
                *new.get_mut(&('v', '<')).unwrap() += count;
                *new.get_mut(&('<', '<')).unwrap() += count;
                *new.get_mut(&('<', 'A')).unwrap() += count;
            }
            (' ', '>') => {
                *new.get_mut(&(' ', 'v')).unwrap() += count;
                *new.get_mut(&('v', 'A')).unwrap() += count;
            }
            (' ', 'A') => {
                *new.get_mut(&(' ', 'A')).unwrap() += count;
            }
            ('^', '^') => {
                *new.get_mut(&('A', 'A')).unwrap() += count;
            }
            ('^', 'v') => {
                *new.get_mut(&('A', 'v')).unwrap() += count;
                *new.get_mut(&('v', 'A')).unwrap() += count;
            }
            ('^', '<') => {
                *new.get_mut(&('A', 'v')).unwrap() += count;
                *new.get_mut(&('v', '<')).unwrap() += count;
                *new.get_mut(&('<', 'A')).unwrap() += count;
            }
            ('^', '>') => {
                *new.get_mut(&('A', 'v')).unwrap() += count;
                *new.get_mut(&('v', '>')).unwrap() += count;
                *new.get_mut(&('>', 'A')).unwrap() += count;
            }
            ('^', 'A') => {
                *new.get_mut(&('A', '>')).unwrap() += count;
                *new.get_mut(&('>', 'A')).unwrap() += count;
            }
            ('v', '^') => {
                *new.get_mut(&('A', '^')).unwrap() += count;
                *new.get_mut(&('^', 'A')).unwrap() += count;
            }
            ('v', 'v') => {
                *new.get_mut(&('A', 'A')).unwrap() += count;
            }
            ('v', '<') => {
                *new.get_mut(&('A', '<')).unwrap() += count;
                *new.get_mut(&('<', 'A')).unwrap() += count;
            }
            ('v', '>') => {
                *new.get_mut(&('A', '>')).unwrap() += count;
                *new.get_mut(&('>', 'A')).unwrap() += count;
            }
            ('v', 'A') => {
                *new.get_mut(&('A', '^')).unwrap() += count;
                *new.get_mut(&('^', '>')).unwrap() += count;
                *new.get_mut(&('>', 'A')).unwrap() += count;
            }
            ('<', '^') => {
                *new.get_mut(&('A', '>')).unwrap() += count;
                *new.get_mut(&('>', '^')).unwrap() += count;
                *new.get_mut(&('^', 'A')).unwrap() += count;
            }
            ('<', 'v') => {
                *new.get_mut(&('A', '>')).unwrap() += count;
                *new.get_mut(&('>', 'A')).unwrap() += count;
            }
            ('<', '<') => {
                *new.get_mut(&('A', 'A')).unwrap() += count;
            }
            ('<', '>') => {
                *new.get_mut(&('A', '>')).unwrap() += count;
                *new.get_mut(&('>', '>')).unwrap() += count;
                *new.get_mut(&('>', 'A')).unwrap() += count;
            }
            ('<', 'A') => {
                *new.get_mut(&('A', '>')).unwrap() += count;
                *new.get_mut(&('>', '>')).unwrap() += count;
                *new.get_mut(&('>', '^')).unwrap() += count;
                *new.get_mut(&('^', 'A')).unwrap() += count;
            }
            ('>', '^') => {
                *new.get_mut(&('A', '<')).unwrap() += count;
                *new.get_mut(&('<', '^')).unwrap() += count;
                *new.get_mut(&('^', 'A')).unwrap() += count;
            }
            ('>', 'v') => {
                *new.get_mut(&('A', '<')).unwrap() += count;
                *new.get_mut(&('<', 'A')).unwrap() += count;
            }
            ('>', '<') => {
                *new.get_mut(&('A', '<')).unwrap() += count;
                *new.get_mut(&('<', '<')).unwrap() += count;
                *new.get_mut(&('<', 'A')).unwrap() += count;
            }
            ('>', '>') => {
                *new.get_mut(&('A', 'A')).unwrap() += count;
            }
            ('>', 'A') => {
                *new.get_mut(&('A', '^')).unwrap() += count;
                *new.get_mut(&('^', 'A')).unwrap() += count;
            }
            ('A', '^') => {
                *new.get_mut(&('A', '<')).unwrap() += count;
                *new.get_mut(&('<', 'A')).unwrap() += count;
            }
            ('A', 'v') => {
                *new.get_mut(&('A', '<')).unwrap() += count;
                *new.get_mut(&('<', 'v')).unwrap() += count;
                *new.get_mut(&('v', 'A')).unwrap() += count;
            }
            ('A', '<') => {
                *new.get_mut(&('A', 'v')).unwrap() += count;
                *new.get_mut(&('v', '<')).unwrap() += count;
                *new.get_mut(&('<', '<')).unwrap() += count;
                *new.get_mut(&('<', 'A')).unwrap() += count;
            }
            ('A', '>') => {
                *new.get_mut(&('A', 'v')).unwrap() += count;
                *new.get_mut(&('v', 'A')).unwrap() += count;
            }
            ('A', 'A') => {
                *new.get_mut(&('A', 'A')).unwrap() += count;
            }
            _ => unreachable!(),
        }
    }

    new
}

fn shortest_sequence_length(directions: &str, iterations: u8) -> u64 {
    let mut counts = counts(directions);

    for _ in 0..iterations {
        counts = next_counts(&counts);
    }

    counts.values().sum()
}

fn part1(codes: &[(String, u16)], numpad_paths: &HashMap<(char, char), Vec<&str>>) {
    let sum: u64 = codes
        .iter()
        .map(|(code, numeric)| {
            let min = numpad_paths_for_code(code, numpad_paths)
                .into_iter()
                .map(|directions| shortest_sequence_length(&directions, 2))
                .min()
                .unwrap();
            min * u64::from(*numeric)
        })
        .sum();
    println!("{sum}");
}

fn part2(codes: &[(String, u16)], numpad_paths: &HashMap<(char, char), Vec<&str>>) {
    let sum: u64 = codes
        .iter()
        .map(|(code, numeric)| {
            let min = numpad_paths_for_code(code, numpad_paths)
                .into_iter()
                .map(|directions| shortest_sequence_length(&directions, 25))
                .min()
                .unwrap();
            min * u64::from(*numeric)
        })
        .sum();
    println!("{sum}");
}

fn main() {
    let codes = parse();
    let numpad_paths = numpad_paths();

    part1(&codes, &numpad_paths);
    part2(&codes, &numpad_paths);
}
