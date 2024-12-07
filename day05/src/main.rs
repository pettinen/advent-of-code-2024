use std::{cmp::Ordering, collections::HashSet};

fn parse() -> (Vec<Vec<u8>>, HashSet<(u8, u8)>) {
    let mut order = HashSet::new();
    let mut lists = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        let sep = line.chars().nth(2);

        if sep == Some('|') {
            let a = line[..2].parse().unwrap();
            let b = line[3..].parse().unwrap();

            order.insert((a, b));
        } else if sep == Some(',') {
            lists.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    (lists, order)
}

fn part1(lists: &Vec<Vec<u8>>, order: &HashSet<(u8, u8)>) {
    let mut sum: u32 = 0;

    for list in lists {
        assert!(list.len() % 2 == 1);

        if list.is_sorted_by(|a, b| order.contains(&(*a, *b))) {
            sum += u32::from(list[list.len() / 2]);
        }
    }

    println!("{sum}");
}

fn part2(lists: &mut Vec<Vec<u8>>, order: &HashSet<(u8, u8)>) {
    let mut sum: u32 = 0;

    for list in lists {
        assert!(list.len() % 2 == 1);

        if list.is_sorted_by(|a, b| order.contains(&(*a, *b))) {
            continue;
        }

        list.sort_by(|a, b| {
            if order.contains(&(*a, *b)) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        sum += u32::from(list[list.len() / 2]);
    }

    println!("{sum}");
}

fn main() {
    let (mut lists, order) = parse();

    part1(&lists, &order);
    part2(&mut lists, &order);
}
