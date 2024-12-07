fn parse() -> Vec<Vec<char>> {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn check_up(chars: &[Vec<char>], x: usize, y: usize) -> bool {
    chars[y - 1][x] == 'M' && chars[y - 2][x] == 'A' && chars[y - 3][x] == 'S'
}

fn check_up_right(chars: &[Vec<char>], x: usize, y: usize) -> bool {
    chars[y - 1][x + 1] == 'M' && chars[y - 2][x + 2] == 'A' && chars[y - 3][x + 3] == 'S'
}

fn check_right(chars: &[Vec<char>], x: usize, y: usize) -> bool {
    chars[y][x + 1] == 'M' && chars[y][x + 2] == 'A' && chars[y][x + 3] == 'S'
}

fn check_down_right(chars: &[Vec<char>], x: usize, y: usize) -> bool {
    chars[y + 1][x + 1] == 'M' && chars[y + 2][x + 2] == 'A' && chars[y + 3][x + 3] == 'S'
}

fn check_down(chars: &[Vec<char>], x: usize, y: usize) -> bool {
    chars[y + 1][x] == 'M' && chars[y + 2][x] == 'A' && chars[y + 3][x] == 'S'
}

fn check_down_left(chars: &[Vec<char>], x: usize, y: usize) -> bool {
    chars[y + 1][x - 1] == 'M' && chars[y + 2][x - 2] == 'A' && chars[y + 3][x - 3] == 'S'
}

fn check_left(chars: &[Vec<char>], x: usize, y: usize) -> bool {
    chars[y][x - 1] == 'M' && chars[y][x - 2] == 'A' && chars[y][x - 3] == 'S'
}

fn check_up_left(chars: &[Vec<char>], x: usize, y: usize) -> bool {
    chars[y - 1][x - 1] == 'M' && chars[y - 2][x - 2] == 'A' && chars[y - 3][x - 3] == 'S'
}

fn part1(chars: &[Vec<char>]) {
    let w = chars[0].len();
    let h = chars.len();

    let mut sum = 0;
    for y in 0..h {
        for x in 0..w {
            if chars[y][x] == 'X' {
                if y >= 3 && check_up(chars, x, y) {
                    sum += 1;
                }
                if x < w - 3 && y >= 3 && check_up_right(chars, x, y) {
                    sum += 1;
                }
                if x < w - 3 && check_right(chars, x, y) {
                    sum += 1;
                }
                if x < w - 3 && y < h - 3 && check_down_right(chars, x, y) {
                    sum += 1;
                }
                if y < h - 3 && check_down(chars, x, y) {
                    sum += 1;
                }
                if x >= 3 && y < h - 3 && check_down_left(chars, x, y) {
                    sum += 1;
                }
                if x >= 3 && check_left(chars, x, y) {
                    sum += 1;
                }
                if x >= 3 && y >= 3 && check_up_left(chars, x, y) {
                    sum += 1;
                }
            }
        }
    }

    println!("{sum}");
}

fn part2(chars: &[Vec<char>]) {
    let w = chars[0].len();
    let h = chars.len();

    let mut sum = 0;
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if chars[y][x] == 'A'
                && ((chars[y - 1][x - 1] == 'M' && chars[y + 1][x + 1] == 'S')
                    || (chars[y - 1][x - 1] == 'S' && chars[y + 1][x + 1] == 'M'))
                && ((chars[y + 1][x - 1] == 'M' && chars[y - 1][x + 1] == 'S')
                    || (chars[y + 1][x - 1] == 'S' && chars[y - 1][x + 1] == 'M'))
            {
                sum += 1;
            }
        }
    }

    println!("{sum}");
}

fn main() {
    let chars = parse();
    part1(&chars);
    part2(&chars);
}
