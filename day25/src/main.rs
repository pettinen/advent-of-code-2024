use itertools::Itertools as _;

fn parse() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let lines = std::io::stdin()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if !line.is_empty() {
                Some(line.chars().collect::<Vec<_>>())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in lines.chunks_exact(7) {
        let mut heights = Vec::with_capacity(5);

        if schematic[0][0] == '#' {
            for i in 0..5 {
                for height in 0..6 {
                    if schematic[height + 1][i] == '.' {
                        heights.push(height as u8);
                        break;
                    }
                }
            }
            locks.push(heights);
        } else if schematic[0][0] == '.' {
            for i in 0..5 {
                for height in 0..6 {
                    if schematic[height + 1][i] == '#' {
                        heights.push(5 - height as u8);
                        break;
                    }
                }
            }
            keys.push(heights);
        } else {
            unreachable!();
        }
    }

    (locks, keys)
}

fn part1(locks: &[Vec<u8>], keys: &[Vec<u8>]) {
    let count = locks
        .iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| {
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    return false;
                }
            }
            true
        })
        .count();
    println!("{count}");
}

fn main() {
    let (locks, keys) = parse();
    part1(&locks, &keys);
}
