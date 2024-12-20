use std::collections::HashSet;

use bimap::BiMap;

fn parse() -> Vec<Vec<char>> {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn get_next_track(
    map: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    current_pos: (usize, usize),
) -> (usize, usize) {
    let (x, y) = current_pos;
    for (x, y) in [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)] {
        if (map[y][x] == '.' || map[y][x] == 'E') && !visited.contains(&(x, y)) {
            visited.insert((x, y));
            return (x, y);
        }
    }
    unreachable!();
}

fn get_track(map: &[Vec<char>]) -> BiMap<u32, (usize, usize)> {
    let mut start = None;
    let mut end = None;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                start = Some((x, y));
            } else if map[y][x] == 'E' {
                end = Some((x, y));
            }
            if start.is_some() && end.is_some() {
                break;
            }
        }
    }
    let (start, end) = (start.unwrap(), end.unwrap());

    let mut track = BiMap::new();
    track.insert(0, start);
    let mut i = 1;
    let mut current = start;
    let mut visited = [start].into_iter().collect();

    while !track.contains_right(&end) {
        current = get_next_track(map, &mut visited, current);
        track.insert(i, current);
        i += 1;
    }
    track
}

#[derive(Clone, Copy, Debug)]
struct Shortcut {
    from: u32,
    to: u32,
    length: u8,
}

fn get_shortcuts(map: &[Vec<char>], track: &BiMap<u32, (usize, usize)>) -> Vec<Shortcut> {
    let mut shortcuts = Vec::new();

    for (&from, &(x, y)) in track {
        // to the north
        if y > 2 && map[y - 1][x] == '#' {
            if let Some(&to) = track.get_by_right(&(x, y - 2)) {
                if to > from {
                    // length 1
                    shortcuts.push(Shortcut {
                        from,
                        to,
                        length: 1,
                    });
                }
            } else if y > 3 && map[y - 2][x] == '#' {
                if let Some(&to) = track.get_by_right(&(x, y - 3)) {
                    if to > from {
                        // length 2
                        shortcuts.push(Shortcut {
                            from,
                            to,
                            length: 2,
                        });
                    }
                }
            }
        }

        // to the south
        if y < map.len() - 3 && map[y + 1][x] == '#' {
            if let Some(&to) = track.get_by_right(&(x, y + 2)) {
                if to > from {
                    // length 1
                    shortcuts.push(Shortcut {
                        from,
                        to,
                        length: 1,
                    });
                }
            } else if y < map.len() - 4 && map[y + 2][x] == '#' {
                if let Some(&to) = track.get_by_right(&(x, y + 3)) {
                    if to > from {
                        // length 2
                        shortcuts.push(Shortcut {
                            from,
                            to,
                            length: 2,
                        });
                    }
                }
            }
        }

        // to the west
        if x > 2 && map[y][x - 1] == '#' {
            if let Some(&to) = track.get_by_right(&(x - 2, y)) {
                if to > from {
                    // length 1
                    shortcuts.push(Shortcut {
                        from,
                        to,
                        length: 1,
                    });
                }
            } else if x > 3 && map[y][x - 2] == '#' {
                if let Some(&to) = track.get_by_right(&(x - 3, y)) {
                    if to > from {
                        // length 2
                        shortcuts.push(Shortcut {
                            from,
                            to,
                            length: 2,
                        });
                    }
                }
            }
        }

        // to the east
        if x < map[0].len() - 3 && map[y][x + 1] == '#' {
            if let Some(&to) = track.get_by_right(&(x + 2, y)) {
                if to > from {
                    // length 1
                    shortcuts.push(Shortcut {
                        from,
                        to,
                        length: 1,
                    });
                }
            } else if x < map[0].len() - 4 && map[y][x + 2] == '#' {
                if let Some(&to) = track.get_by_right(&(x + 3, y)) {
                    if to > from {
                        // length 2
                        shortcuts.push(Shortcut {
                            from,
                            to,
                            length: 2,
                        });
                    }
                }
            }
        }
    }

    shortcuts
}

fn part1(map: &[Vec<char>]) {
    let track = get_track(map);
    let shortcuts = get_shortcuts(map, &track);
    let count = shortcuts
        .into_iter()
        .filter(|shortcut| shortcut.to - shortcut.from - u32::from(shortcut.length) > 100)
        .count();
    println!("{count}");
}

fn part2(map: &[Vec<char>]) {
    let track = get_track(map);

    let mut count = 0;

    let track_len = track.len();
    let index_pairs =
        (0..track.len() - 1).flat_map(|from| (from + 1..track_len).map(move |to| (from, to)));
    let mut track_vec = track.into_iter().collect::<Vec<_>>();
    track_vec.sort_by_key(|&(i, _)| i);
    let track_vec = track_vec
        .into_iter()
        .map(|(_, pos)| pos)
        .collect::<Vec<_>>();

    for (from, to) in index_pairs {
        let (x1, y1) = track_vec[from];
        let (x2, y2) = track_vec[to];

        let mut distance_by_cheat = 0;
        if x1 > x2 {
            distance_by_cheat += x1 - x2;
        } else {
            distance_by_cheat += x2 - x1;
        }
        if y1 > y2 {
            distance_by_cheat += y1 - y2;
        } else {
            distance_by_cheat += y2 - y1;
        }

        let distance_by_track = to - from;

        if distance_by_cheat >= distance_by_track {
            continue; // not a cheat
        }
        if distance_by_cheat > 20 {
            continue; // cheat is too long
        }
        if distance_by_track - distance_by_cheat >= 100 {
            count += 1;
        }
    }

    println!("{count}");
}

fn main() {
    let map = parse();
    part1(&map);
    part2(&map);
}
