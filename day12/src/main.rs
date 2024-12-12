use std::collections::HashSet;

fn area(region: &HashSet<(usize, usize)>) -> usize {
    region.len()
}

fn perimeter(region: &HashSet<(usize, usize)>) -> usize {
    let mut perimeter = 0;
    for &(x, y) in region {
        if x == 0 || !region.contains(&(x - 1, y)) {
            perimeter += 1;
        }
        if !region.contains(&(x + 1, y)) {
            perimeter += 1;
        }
        if y == 0 || !region.contains(&(x, y - 1)) {
            perimeter += 1
        }
        if !region.contains(&(x, y + 1)) {
            perimeter += 1;
        }
    }
    perimeter
}

fn sides(region: &HashSet<(usize, usize)>) -> usize {
    let mut horizontal_fences: Vec<HashSet<(usize, usize)>> = Vec::new();
    let mut squares = region.iter().collect::<Vec<_>>();

    squares.sort_by_key(|(x, y)| (y, x));

    for &&(x, y) in &squares {
        if y == 0 || !region.contains(&(x, y - 1)) {
            let mut added_to_existing = false;
            for fence in &mut horizontal_fences {
                if x > 0 && fence.contains(&(x - 1, y)) && region.contains(&(x - 1, y)) {
                    fence.insert((x, y));
                    added_to_existing = true;
                    break;
                }
            }
            if !added_to_existing {
                let mut new_fence = HashSet::new();
                new_fence.insert((x, y));
                horizontal_fences.push(new_fence);
            }
        }

        if !region.contains(&(x, y + 1)) {
            let mut added_to_existing = false;
            for fence in &mut horizontal_fences {
                if x > 0 && fence.contains(&(x - 1, y + 1)) {
                    fence.insert((x, y + 1));
                    added_to_existing = true;
                    break;
                }
            }
            if !added_to_existing {
                let mut new_fence = HashSet::new();
                new_fence.insert((x, y + 1));
                horizontal_fences.push(new_fence);
            }
        }
    }

    let mut vertical_fences: Vec<HashSet<(usize, usize)>> = Vec::new();
    squares.sort();

    for &&(x, y) in &squares {
        if x == 0 || !region.contains(&(x - 1, y)) {
            let mut added_to_existing = false;
            for fence in &mut vertical_fences {
                if y > 0 && fence.contains(&(x, y - 1)) && region.contains(&(x, y - 1)) {
                    fence.insert((x, y));
                    added_to_existing = true;
                    break;
                }
            }
            if !added_to_existing {
                let mut new_fence = HashSet::new();
                new_fence.insert((x, y));
                vertical_fences.push(new_fence);
            }
        }

        if !region.contains(&(x + 1, y)) {
            let mut added_to_existing = false;
            for fence in &mut vertical_fences {
                if y > 0 && fence.contains(&(x + 1, y - 1)) {
                    fence.insert((x + 1, y));
                    added_to_existing = true;
                    break;
                }
            }
            if !added_to_existing {
                let mut new_fence = HashSet::new();
                new_fence.insert((x + 1, y));
                vertical_fences.push(new_fence);
            }
        }
    }

    horizontal_fences.len() + vertical_fences.len()
}

fn expand_region(map: &[Vec<char>], region: &HashSet<(usize, usize)>) -> Option<(usize, usize)> {
    for &(x, y) in region {
        let char = map[y][x];
        if y > 0 && !region.contains(&(x, y - 1)) && map[y - 1][x] == char {
            return Some((x, y - 1));
        }
        if x > 0 && !region.contains(&(x - 1, y)) && map[y][x - 1] == char {
            return Some((x - 1, y));
        }
        if y < map.len() - 1 && !region.contains(&(x, y + 1)) && map[y + 1][x] == char {
            return Some((x, y + 1));
        }
        if x < map[0].len() - 1 && !region.contains(&(x + 1, y)) && map[y][x + 1] == char {
            return Some((x + 1, y));
        }
    }
    None
}

fn find_continuous_regions(map: &[Vec<char>]) -> Vec<HashSet<(usize, usize)>> {
    let mut regions = Vec::new();
    let mut squares_added = HashSet::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if squares_added.contains(&(x, y)) {
                continue;
            }
            let mut region = HashSet::new();
            region.insert((x, y));
            while let Some(new_square) = expand_region(map, &region) {
                region.insert(new_square);
                squares_added.insert(new_square);
            }
            regions.push(region);
        }
    }
    regions
}

fn part1(map: &[Vec<char>]) {
    let regions = find_continuous_regions(map);
    let sum = regions
        .iter()
        .map(|region| area(region) * perimeter(region))
        .sum::<usize>();
    println!("{sum}");
}

fn part2(map: &[Vec<char>]) {
    let regions = find_continuous_regions(map);
    let sum = regions
        .iter()
        .map(|region| area(region) * sides(region))
        .sum::<usize>();
    println!("{sum}");
}

fn main() {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    part1(&map);
    part2(&map);
}
