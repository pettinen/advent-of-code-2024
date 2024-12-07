use regex::Regex;

fn parse() -> String {
    let stdin = std::io::stdin();
    let mut input = String::new();
    loop {
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }
    }
    input
}

fn part1(input: &str) {
    let re = Regex::new(r"mul\((?<a>\d\d?\d?),(?<b>\d\d?\d?)\)").unwrap();

    let mut sum = 0;
    for captures in re.captures_iter(input) {
        let a = captures.name("a").unwrap().as_str().parse::<u32>().unwrap();
        let b = captures.name("b").unwrap().as_str().parse::<u32>().unwrap();
        sum += a * b;
    }
    println!("{sum}");
}

fn part2(input: &str) {
    let re = Regex::new(r"mul\((?<a>\d\d?\d?),(?<b>\d\d?\d?)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for captures in re.captures_iter(input) {
        let full_match = captures.get(0).unwrap().as_str();
        if full_match == "do()" {
            enabled = true;
        } else if full_match == "don't()" {
            enabled = false;
        } else if enabled {
            let a = captures.name("a").unwrap().as_str().parse::<u32>().unwrap();
            let b = captures.name("b").unwrap().as_str().parse::<u32>().unwrap();
            sum += a * b;
        }
    }
    println!("{sum}");
}

fn main() {
    let input = parse();
    part1(&input);
    part2(&input);
}
