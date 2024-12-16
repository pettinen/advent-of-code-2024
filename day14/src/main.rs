use std::cmp::Ordering;

use regex::Regex;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse() -> Vec<Robot> {
    let line_re = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();

    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let captures = line_re.captures(&line).unwrap();
            Robot {
                position: (
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                ),
                velocity: (
                    captures.get(3).unwrap().as_str().parse().unwrap(),
                    captures.get(4).unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        .collect()
}

fn safety_factor(robots: &[Robot]) -> u32 {
    let mut counts = [0; 4];

    for robot in robots {
        match robot.position.0.cmp(&(WIDTH / 2)) {
            Ordering::Less => match robot.position.1.cmp(&(HEIGHT / 2)) {
                Ordering::Less => {
                    counts[0] += 1;
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    counts[1] += 1;
                }
            },
            Ordering::Equal => {}
            Ordering::Greater => match robot.position.1.cmp(&(HEIGHT / 2)) {
                Ordering::Less => {
                    counts[2] += 1;
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    counts[3] += 1;
                }
            },
        }
    }

    counts.into_iter().reduce(|acc, e| acc * e).unwrap()
}

fn tick(robot: &mut Robot) {
    robot.position.0 += robot.velocity.0;
    robot.position.0 %= WIDTH;
    if robot.position.0 < 0 {
        robot.position.0 += WIDTH;
    }

    robot.position.1 += robot.velocity.1;
    robot.position.1 %= HEIGHT;
    if robot.position.1 < 0 {
        robot.position.1 += HEIGHT;
    }
}

fn part1(robots: &mut [Robot]) {
    const TICKS: u8 = 100;

    for _ in 0..TICKS {
        for robot in robots.iter_mut() {
            tick(robot);
        }
    }

    println!("{}", safety_factor(robots));
}

fn print_map(robots: &[Robot]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if robots.iter().any(|robot| robot.position == (x, y)) {
                print!("\u{2588}");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn part2(robots: &mut [Robot]) {
    // vertical pattern appears first at 7 seconds and repeats every 101 seconds
    // horizontal pattern appears first at 53 seconds and repeats every 103 seconds
    // first intersection of these is at 8087 seconds (Python snippet):
    // >>> set(range(7, 10000, 101)) & set(range(53, 10000, 103)) == {8087}
    for i in 1..=8087 {
        for robot in robots.iter_mut() {
            tick(robot);
        }

        if i == 7 || i == 53 || i == 8087 {
            println!("after {i} seconds:");
            print_map(robots);
        }
    }
}

fn main() {
    let mut input = parse();
    part1(&mut input.clone());
    part2(&mut input);
}
