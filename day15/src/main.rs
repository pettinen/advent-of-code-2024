use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Wall,
    Box,
    Robot,
    Empty,
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn parse() -> (HashMap<(u8, u8), Square>, Vec<Move>) {
    let mut map = HashMap::new();
    let mut y = 0;
    let mut moves = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let chars = line.chars().collect::<Vec<_>>();
        match chars[0] {
            '#' => {
                for (x, char) in chars.iter().enumerate() {
                    let x = u8::try_from(x).unwrap();
                    match char {
                        '#' => {
                            map.insert((x, y), Square::Wall);
                        }
                        'O' => {
                            map.insert((x, y), Square::Box);
                        }
                        '@' => {
                            map.insert((x, y), Square::Robot);
                        }
                        '.' => {
                            map.insert((x, y), Square::Empty);
                        }
                        char => panic!("unexpected character '{char}'"),
                    }
                }
                y += 1;
            }
            '^' | 'v' | '<' | '>' => {
                for char in chars {
                    match char {
                        '^' => {
                            moves.push(Move::Up);
                        }
                        'v' => {
                            moves.push(Move::Down);
                        }
                        '<' => {
                            moves.push(Move::Left);
                        }
                        '>' => {
                            moves.push(Move::Right);
                        }
                        char => panic!("unexpected character '{char}'"),
                    }
                }
            }
            char => {
                panic!("unexpected character '{char}'");
            }
        }
    }

    (map, moves)
}

fn new_position(position: (u8, u8), r#move: Move, count: u8) -> (u8, u8) {
    match r#move {
        Move::Up => (position.0, position.1 - count),
        Move::Down => (position.0, position.1 + count),
        Move::Left => (position.0 - count, position.1),
        Move::Right => (position.0 + count, position.1),
    }
}

fn do_move(map: &mut HashMap<(u8, u8), Square>, r#move: Move, robot_pos: (u8, u8)) -> (u8, u8) {
    let mut boxes_to_move = Vec::new();

    let mut count = 0;
    let positions = std::iter::from_fn(|| {
        count += 1;
        Some(new_position(robot_pos, r#move, count))
    });

    for position in positions {
        match map.get(&position).unwrap() {
            Square::Wall => return robot_pos,
            Square::Box => {
                boxes_to_move.push(position);
            }
            Square::Robot => panic!("unexpected robot"),
            Square::Empty => break,
        }
    }

    for position in boxes_to_move.into_iter().rev() {
        map.insert(new_position(position, r#move, 1), Square::Box);
    }

    let new_robot_pos = new_position(robot_pos, r#move, 1);
    map.insert(new_robot_pos, Square::Robot);
    map.insert(robot_pos, Square::Empty);
    new_robot_pos
}

#[allow(dead_code)]
fn print_map(map: &HashMap<(u8, u8), Square>) {
    for y in 0.. {
        for x in 0.. {
            match map.get(&(x, y)) {
                Some(Square::Wall) => print!("#"),
                Some(Square::Box) => print!("O"),
                Some(Square::Robot) => print!("@"),
                Some(Square::Empty) => print!("."),
                None => {
                    println!();
                    if x == 0 {
                        return;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn part1(map: &mut HashMap<(u8, u8), Square>, moves: &[Move]) {
    let mut robot_pos = *map
        .iter()
        .find(|&(_, square)| *square == Square::Robot)
        .unwrap()
        .0;

    for r#move in moves {
        robot_pos = do_move(map, *r#move, robot_pos);
        //print_map(map);
    }
    let sum: u32 = map
        .iter()
        .filter(|&(_, square)| *square == Square::Box)
        .map(|((x, y), _)| 100 * u32::from(*y) + u32::from(*x))
        .sum();

    println!("{sum}");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square2 {
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

fn update_map(map: &HashMap<(u8, u8), Square>) -> HashMap<(u8, u8), Square2> {
    let mut new_map = HashMap::new();

    for (&(x, y), &square) in map {
        match square {
            Square::Wall => {
                new_map.insert((2 * x, y), Square2::Wall);
                new_map.insert((2 * x + 1, y), Square2::Wall);
            }
            Square::Box => {
                new_map.insert((2 * x, y), Square2::BoxLeft);
                new_map.insert((2 * x + 1, y), Square2::BoxRight);
            }
            Square::Robot => {
                new_map.insert((2 * x, y), Square2::Robot);
                new_map.insert((2 * x + 1, y), Square2::Empty);
            }
            Square::Empty => {
                new_map.insert((2 * x, y), Square2::Empty);
                new_map.insert((2 * x + 1, y), Square2::Empty);
            }
        }
    }

    new_map
}

#[allow(dead_code)]
fn print_map2(map: &HashMap<(u8, u8), Square2>) {
    for y in 0.. {
        for x in 0.. {
            match map.get(&(x, y)) {
                Some(Square2::Wall) => print!("#"),
                Some(Square2::BoxLeft) => print!("["),
                Some(Square2::BoxRight) => print!("]"),
                Some(Square2::Robot) => print!("@"),
                Some(Square2::Empty) => print!("."),
                None => {
                    println!();
                    if x == 0 {
                        return;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum BoxesToMove {
    CanMove(Vec<(u8, u8)>),
    Blocked,
}

fn get_boxes_to_move(
    map: &HashMap<(u8, u8), Square2>,
    (x, y): (u8, u8),
    r#move: Move,
    mut boxes: Vec<(u8, u8)>,
) -> BoxesToMove {
    boxes.push((x, y));
    match r#move {
        Move::Up => {
            match map.get(&(x, y - 1)).unwrap() {
                Square2::Wall => return BoxesToMove::Blocked,
                Square2::BoxLeft => match get_boxes_to_move(map, (x, y - 1), r#move, boxes) {
                    BoxesToMove::CanMove(new_boxes) => {
                        boxes = new_boxes;
                    }
                    BoxesToMove::Blocked => return BoxesToMove::Blocked,
                },
                Square2::BoxRight => match get_boxes_to_move(map, (x - 1, y - 1), r#move, boxes) {
                    BoxesToMove::CanMove(new_boxes) => {
                        boxes = new_boxes;
                    }
                    BoxesToMove::Blocked => return BoxesToMove::Blocked,
                },
                Square2::Robot => panic!("unexpected robot"),
                Square2::Empty => {}
            }

            match map.get(&(x + 1, y - 1)).unwrap() {
                Square2::Wall => return BoxesToMove::Blocked,
                Square2::BoxLeft => match get_boxes_to_move(map, (x + 1, y - 1), r#move, boxes) {
                    BoxesToMove::CanMove(new_boxes) => {
                        boxes = new_boxes;
                    }
                    BoxesToMove::Blocked => return BoxesToMove::Blocked,
                },
                Square2::BoxRight => match get_boxes_to_move(map, (x, y - 1), r#move, boxes) {
                    BoxesToMove::CanMove(new_boxes) => {
                        boxes = new_boxes;
                    }
                    BoxesToMove::Blocked => return BoxesToMove::Blocked,
                },
                Square2::Robot => panic!("unexpected robot"),
                Square2::Empty => {}
            }

            BoxesToMove::CanMove(boxes)
        }
        Move::Down => {
            match map.get(&(x, y + 1)).unwrap() {
                Square2::Wall => return BoxesToMove::Blocked,
                Square2::BoxLeft => match get_boxes_to_move(map, (x, y + 1), r#move, boxes) {
                    BoxesToMove::CanMove(new_boxes) => {
                        boxes = new_boxes;
                    }
                    BoxesToMove::Blocked => return BoxesToMove::Blocked,
                },
                Square2::BoxRight => match get_boxes_to_move(map, (x - 1, y + 1), r#move, boxes) {
                    BoxesToMove::CanMove(new_boxes) => {
                        boxes = new_boxes;
                    }
                    BoxesToMove::Blocked => return BoxesToMove::Blocked,
                },
                Square2::Robot => panic!("unexpected robot"),
                Square2::Empty => {}
            }

            match map.get(&(x + 1, y + 1)).unwrap() {
                Square2::Wall => return BoxesToMove::Blocked,
                Square2::BoxLeft => match get_boxes_to_move(map, (x + 1, y + 1), r#move, boxes) {
                    BoxesToMove::CanMove(new_boxes) => {
                        boxes = new_boxes;
                    }
                    BoxesToMove::Blocked => return BoxesToMove::Blocked,
                },
                Square2::BoxRight => match get_boxes_to_move(map, (x, y + 1), r#move, boxes) {
                    BoxesToMove::CanMove(new_boxes) => {
                        boxes = new_boxes;
                    }
                    BoxesToMove::Blocked => return BoxesToMove::Blocked,
                },
                Square2::Robot => panic!("unexpected robot"),
                Square2::Empty => {}
            }

            BoxesToMove::CanMove(boxes)
        }
        Move::Left => {
            match map.get(&(x - 1, y)).unwrap() {
                Square2::Wall => return BoxesToMove::Blocked,
                Square2::BoxLeft => panic!("unexpected BoxLeft"),
                Square2::BoxRight => match get_boxes_to_move(map, (x - 2, y), r#move, boxes) {
                    BoxesToMove::CanMove(new_boxes) => {
                        boxes = new_boxes;
                    }
                    BoxesToMove::Blocked => return BoxesToMove::Blocked,
                },
                Square2::Robot => panic!("unexpected robot"),
                Square2::Empty => {}
            }
            BoxesToMove::CanMove(boxes)
        }
        Move::Right => {
            match map.get(&(x + 2, y)).unwrap() {
                Square2::Wall => return BoxesToMove::Blocked,
                Square2::BoxLeft => match get_boxes_to_move(map, (x + 2, y), r#move, boxes) {
                    BoxesToMove::CanMove(new_boxes) => {
                        boxes = new_boxes;
                    }
                    BoxesToMove::Blocked => return BoxesToMove::Blocked,
                },
                Square2::BoxRight => panic!("unexpected BoxRight"),
                Square2::Robot => panic!("unexpected robot"),
                Square2::Empty => {}
            }
            BoxesToMove::CanMove(boxes)
        }
    }
}

fn do_move2(map: &mut HashMap<(u8, u8), Square2>, r#move: Move, robot_pos: (u8, u8)) -> (u8, u8) {
    let next_pos = new_position(robot_pos, r#move, 1);

    let boxes_to_move = match map.get(&next_pos).unwrap() {
        Square2::Wall => return robot_pos,
        Square2::BoxLeft => get_boxes_to_move(map, next_pos, r#move, Vec::new()),
        Square2::BoxRight => {
            get_boxes_to_move(map, (next_pos.0 - 1, next_pos.1), r#move, Vec::new())
        }
        Square2::Robot => panic!("unexpected robot"),
        Square2::Empty => {
            map.insert(next_pos, Square2::Robot);
            map.insert(robot_pos, Square2::Empty);
            return next_pos;
        }
    };

    if let BoxesToMove::CanMove(mut boxes_to_move) = boxes_to_move {
        match r#move {
            Move::Up => {
                boxes_to_move.sort_by_key(|&(_, y)| y);
            }
            Move::Down => {
                boxes_to_move.sort_by_key(|&(_, y)| -i16::from(y));
            }
            Move::Left => {
                boxes_to_move.sort_by_key(|&(x, _)| x);
            }
            Move::Right => {
                boxes_to_move.sort_by_key(|&(x, _)| -i16::from(x));
            }
        }
        for (x, y) in boxes_to_move {
            match r#move {
                Move::Up => {
                    map.insert((x, y - 1), Square2::BoxLeft);
                    map.insert((x + 1, y - 1), Square2::BoxRight);
                    map.insert((x, y), Square2::Empty);
                    map.insert((x + 1, y), Square2::Empty);
                }
                Move::Down => {
                    map.insert((x, y + 1), Square2::BoxLeft);
                    map.insert((x + 1, y + 1), Square2::BoxRight);
                    map.insert((x, y), Square2::Empty);
                    map.insert((x + 1, y), Square2::Empty);
                }
                Move::Left => {
                    map.insert((x - 1, y), Square2::BoxLeft);
                    map.insert((x, y), Square2::BoxRight);
                    map.insert((x + 1, y), Square2::Empty);
                }
                Move::Right => {
                    map.insert((x + 2, y), Square2::BoxRight);
                    map.insert((x + 1, y), Square2::BoxLeft);
                    map.insert((x, y), Square2::Empty);
                }
            }
        }
    } else {
        return robot_pos;
    }

    let new_robot_pos = new_position(robot_pos, r#move, 1);
    map.insert(new_robot_pos, Square2::Robot);
    map.insert(robot_pos, Square2::Empty);
    new_robot_pos
}

fn part2(map: &HashMap<(u8, u8), Square>, moves: &[Move]) {
    let mut map = update_map(map);
    let mut robot_pos = *map
        .iter()
        .find(|&(_, square)| *square == Square2::Robot)
        .unwrap()
        .0;

    for r#move in moves {
        robot_pos = do_move2(&mut map, *r#move, robot_pos);
        //print_map2(&map);
    }

    let sum: u32 = map
        .iter()
        .filter(|&(_, square)| *square == Square2::BoxLeft)
        .map(|((x, y), _)| 100 * u32::from(*y) + u32::from(*x))
        .sum();

    println!("{sum}");
}

fn main() {
    let (map, moves) = parse();
    part1(&mut map.clone(), &moves);
    part2(&map, &moves);
}
