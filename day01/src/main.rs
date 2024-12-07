use std::collections::HashMap;

fn parse() -> (Vec<i64>, Vec<i64>) {
    let (mut list_a, mut list_b): (Vec<_>, Vec<_>) = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let numbers = line.split_once("   ").unwrap();
            (
                numbers.0.parse::<i64>().unwrap(),
                numbers.1.parse::<i64>().unwrap(),
            )
        })
        .unzip();
    list_a.sort();
    list_b.sort();
    (list_a, list_b)
}

fn part1(list_a: &[i64], list_b: &[i64]) {
    let sum: i64 = list_a.iter().zip(list_b).map(|(a, b)| (a - b).abs()).sum();
    println!("{sum}");
}

fn part2(list_a: &[i64], list_b: &[i64]) {
    let mut map = HashMap::new();
    for n in list_b {
        if let Some(count) = map.get_mut(&n) {
            *count += 1;
        } else {
            map.insert(n, 1);
        }
    }

    let sum: i64 = list_a.iter().map(|n| n * map.get(&n).unwrap_or(&0)).sum();
    println!("{sum}");
}

fn main() {
    let (list_a, list_b) = parse();
    part1(&list_a, &list_b);
    part2(&list_a, &list_b);
}
