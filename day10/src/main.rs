use std::collections::HashSet;

fn parse() -> Vec<Vec<u8>> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|char| char.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect()
}

fn update_reachable(input: &[Vec<u8>], set: &mut HashSet<(usize, usize)>, x: usize, y: usize) {
    let height = input[y][x];
    if height == 9 {
        set.insert((x, y));
        return;
    }

    let w = input[0].len();
    let h = input.len();

    if x > 0 && input[y][x - 1] == height + 1 {
        update_reachable(input, set, x - 1, y);
    }
    if x < w - 1 && input[y][x + 1] == height + 1 {
        update_reachable(input, set, x + 1, y);
    }
    if y > 0 && input[y - 1][x] == height + 1 {
        update_reachable(input, set, x, y - 1);
    }
    if y < h - 1 && input[y + 1][x] == height + 1 {
        update_reachable(input, set, x, y + 1);
    }
}

fn part1(input: &[Vec<u8>]) {
    let mut sum: usize = 0;

    for (y, line) in input.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                let mut reachable = HashSet::new();
                update_reachable(input, &mut reachable, x, y);
                sum += reachable.len();
            }
        }
    }

    println!("{sum}");
}

fn rating(input: &[Vec<u8>], x: usize, y: usize) -> u32 {
    let height = input[y][x];
    if height == 9 {
        return 1;
    }

    let w = input[0].len();
    let h = input.len();

    let mut sum = 0;

    if x > 0 && input[y][x - 1] == height + 1 {
        sum += rating(input, x - 1, y);
    }
    if x < w - 1 && input[y][x + 1] == height + 1 {
        sum += rating(input, x + 1, y);
    }
    if y > 0 && input[y - 1][x] == height + 1 {
        sum += rating(input, x, y - 1);
    }
    if y < h - 1 && input[y + 1][x] == height + 1 {
        sum += rating(input, x, y + 1);
    }
    sum
}

fn part2(input: &[Vec<u8>]) {
    let mut sum = 0;

    for (y, line) in input.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                sum += rating(input, x, y);
            }
        }
    }

    println!("{sum}");
}

fn main() {
    let input = parse();
    part1(&input);
    part2(&input);
}
