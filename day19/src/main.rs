use std::{collections::HashMap, sync::RwLock};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref OPTIONS: RwLock<HashMap<String, u64>> = RwLock::default();
}

fn parse() -> (Vec<String>, Vec<String>) {
    let mut lines = std::io::stdin().lines();

    let patterns = lines
        .next()
        .unwrap()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    lines.next();
    let designs = lines.map(|line| line.unwrap()).collect();
    (patterns, designs)
}

fn part1(patterns: &[String], designs: &[String]) {
    let patterns_joined = patterns.join("|");
    let patterns_re = Regex::new(&format!("^({patterns_joined})+$")).unwrap();

    let sum: u64 = designs
        .iter()
        .map(|design| u64::from(patterns_re.is_match(design)))
        .sum();
    println!("{sum}");
}

fn count_options(remaining: &str, patterns: &[String]) -> u64 {
    if remaining.is_empty() {
        return 1;
    }
    let mut sum = 0;
    if let Some(&value) = OPTIONS.read().unwrap().get(remaining) {
        return value;
    }

    for pattern in patterns {
        if remaining.starts_with(pattern) {
            sum += count_options(&remaining[pattern.len()..], patterns);
        }
    }
    let mut options = OPTIONS.write().unwrap();
    options.insert(remaining.to_string(), sum);
    sum
}

fn part2(patterns: &[String], designs: &[String]) {
    let sum: u64 = designs
        .iter()
        .map(|design| count_options(design, patterns))
        .sum();
    println!("{sum}");
}

fn main() {
    let (patterns, designs) = parse();
    part1(&patterns, &designs);
    part2(&patterns, &designs);
}
