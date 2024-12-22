use std::collections::{HashMap, VecDeque};

use itertools::{repeat_n, Itertools as _};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn parse() -> Vec<u64> {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

fn next_number(number: u64) -> u64 {
    let a = ((number << 6) ^ number) % 0x1000000;
    let b = ((a >> 5) ^ a) % 0x1000000;
    let c = ((b << 11) ^ b) % 0x1000000;
    c
}

fn part1(input: &[u64]) {
    let sum: u64 = input
        .par_iter()
        .map(|&(mut number)| {
            for _ in 0..2000 {
                number = next_number(number);
            }
            number
        })
        .sum();
    println!("{sum}");
}

fn prices_by_sequence(mut secret_number: u64) -> HashMap<(i8, i8, i8, i8), u8> {
    let mut prices = HashMap::with_capacity(19_usize.pow(4));

    let mut changes = VecDeque::new();
    let mut ones = (secret_number % 10) as i8;

    for i in 0..2000 {
        secret_number = next_number(secret_number);
        let new_ones = (secret_number % 10) as i8;
        changes.push_back(new_ones - ones);
        ones = new_ones;

        if i > 3 {
            changes.pop_front();
        }
        if i > 2 {
            prices.entry((changes[0], changes[1], changes[2], changes[3])).or_insert(ones as u8);
        }
    }

    prices
}

fn part2(input: &[u64]) {
    let prices_by_sequence = input
        .par_iter()
        .map(|&secret_number| prices_by_sequence(secret_number))
        .collect::<Vec<_>>();

    let changes = repeat_n(-9..=9, 4)
        .multi_cartesian_product()
        .map(|changes| (changes[0], changes[1], changes[2], changes[3]))
        .collect::<Vec<_>>();

    let best_changes = changes
        .into_par_iter()
        .map(|changes| {
            prices_by_sequence
                .par_iter()
                .map(|prices| u64::from(*prices.get(&changes).unwrap_or(&0)))
                .sum::<u64>()
        })
        .max()
        .unwrap();
    println!("{best_changes}");
}

fn main() {
    let input = parse();
    part1(&input);
    part2(&input);
}
