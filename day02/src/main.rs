use std::cmp::Ordering;

fn parse() -> Vec<Vec<i8>> {
    std::io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(' ')
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1(lines: &[Vec<i8>]) {
    let count: i64 = lines
        .iter()
        .map(|line| {
            let increasing = match line[0].cmp(&line[1]) {
                Ordering::Less => true,
                Ordering::Equal => return 0,
                Ordering::Greater => false,
            };
            for i in 0..line.len() - 1 {
                let diff = line[i + 1] - line[i];
                if increasing && !(1..=3).contains(&diff) {
                    return 0;
                }
                if !increasing && !(-3..=-1).contains(&diff) {
                    return 0;
                }
            }
            1
        })
        .sum();

    println!("{count}");
}

fn is_safe(line: &[i8], remove_index: usize) -> bool {
    let first = if remove_index == 0 { 1 } else { 0 };
    let second = if remove_index < 2 { 2 } else { 1 };
    let increasing = match line[first].cmp(&line[second]) {
        Ordering::Less => true,
        Ordering::Equal => return false,
        Ordering::Greater => false,
    };

    for i in 0..line.len() - 2 {
        let first = if i >= remove_index { i + 1 } else { i };
        let second = if first + 1 == remove_index {
            first + 2
        } else {
            first + 1
        };
        let diff = line[second] - line[first];
        if increasing && !(1..=3).contains(&diff) {
            return false;
        }
        if !increasing && !(-3..=-1).contains(&diff) {
            return false;
        }
    }
    true
}

fn part2(lines: &[Vec<i8>]) {
    let count: i64 = lines
        .iter()
        .map(|line| {
            for remove_index in 0..line.len() {
                if is_safe(line, remove_index) {
                    return 1;
                }
            }
            0
        })
        .sum();

    println!("{count}");
}

fn main() {
    let lines = parse();
    part1(&lines);
    part2(&lines);
}
