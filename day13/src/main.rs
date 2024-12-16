use itertools::Itertools as _;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn parse() -> Vec<Machine> {
    let button_re = Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_re = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .chunks(4)
        .into_iter()
        .map(|mut chunk| {
            let a_string = chunk.next().unwrap();
            let a_captures = button_re.captures(&a_string).unwrap();
            let a_x = a_captures.get(1).unwrap().as_str().parse().unwrap();
            let a_y = a_captures.get(2).unwrap().as_str().parse().unwrap();

            let b_string = chunk.next().unwrap();
            let b_captures = button_re.captures(&b_string).unwrap();
            let b_x = b_captures.get(1).unwrap().as_str().parse().unwrap();
            let b_y = b_captures.get(2).unwrap().as_str().parse().unwrap();

            let prize_string = chunk.next().unwrap();
            let prize_captures = prize_re.captures(&prize_string).unwrap();
            let prize_x = prize_captures.get(1).unwrap().as_str().parse().unwrap();
            let prize_y = prize_captures.get(2).unwrap().as_str().parse().unwrap();

            Machine {
                a: (a_x, a_y),
                b: (b_x, b_y),
                prize: (prize_x, prize_y),
            }
        })
        .collect()
}

fn find_solution(machine: &Machine, prize_offset: i64) -> Option<i64> {
    let prize_x = machine.prize.0 + prize_offset;
    let prize_y = machine.prize.1 + prize_offset;

    let dividend = machine.b.1 * prize_x - machine.b.0 * prize_y;
    let divisor = machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0;

    let a = if dividend % divisor == 0 {
        dividend / divisor
    } else {
        return None;
    };

    let remaining_x = prize_x - a * machine.a.0;
    let b = if remaining_x % machine.b.0 == 0 {
        remaining_x / machine.b.0
    } else {
        return None;
    };

    Some(3 * a + b)
}

fn part1(input: &[Machine]) {
    let sum: i64 = input
        .iter()
        .map(|machine| find_solution(machine, 0).unwrap_or(0))
        .sum();
    println!("{sum}");
}

fn part2(input: &[Machine]) {
    let sum: i64 = input
        .iter()
        .map(|machine| find_solution(machine, 10_000_000_000_000).unwrap_or(0))
        .sum();
    println!("{sum}");
}

fn main() {
    let input = parse();
    part1(&input);
    part2(&input);
}
