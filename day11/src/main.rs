use lazy_static::lazy_static;
use papaya::HashMap;

lazy_static! {
    static ref RESULTS: HashMap<(u64, u8, u8), u64> = HashMap::new();
}

fn parse() -> Vec<u64> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
        .trim_end()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn len(num: u64, step: u8, max_step: u8) -> u64 {
    let results = RESULTS.pin();
    if num == 0 {
        if step == max_step {
            return 1;
        }
        return *results.get_or_insert_with((1, step + 1, max_step), || len(1, step + 1, max_step));
    }

    let log = num.ilog10();
    if log % 2 == 1 {
        if step == max_step {
            return 2;
        }
        let pow = 10_u64.pow(log / 2 + 1);
        let upper = num / pow;
        let lower = num - upper * pow;
        return results.get_or_insert_with((upper, step + 1, max_step), || {
            len(upper, step + 1, max_step)
        }) + results.get_or_insert_with((lower, step + 1, max_step), || {
            len(lower, step + 1, max_step)
        });
    }

    if step == max_step {
        return 1;
    }
    *results.get_or_insert_with((2024 * num, step + 1, max_step), || {
        len(2024 * num, step + 1, max_step)
    })
}

fn part1(input: &[u64]) {
    let sum: u64 = input.iter().map(|num| len(*num, 1, 25)).sum();
    println!("{sum}");
}

fn part2(input: &[u64]) {
    let sum: u64 = input.iter().map(|num| len(*num, 1, 75)).sum();
    println!("{sum}");
}

fn main() {
    let input = parse();
    part1(&input);
    part2(&input);
}
