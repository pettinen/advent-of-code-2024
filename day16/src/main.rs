use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    End,
    Open,
    Wall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

type Position = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vertex {
    position: Position,
    direction: Direction,
}

fn parse() -> (Vec<Vec<Square>>, Position, Position) {
    let mut map = Vec::new();
    let mut start_pos = None;
    let mut end_pos = None;

    for (y, line) in std::io::stdin().lines().enumerate() {
        let mut map_line = Vec::new();
        for (x, char) in line.unwrap().chars().enumerate() {
            match char {
                'S' => {
                    start_pos = Some((x, y));
                    map_line.push(Square::Open);
                }
                'E' => {
                    end_pos = Some((x, y));
                    map_line.push(Square::End);
                }
                '.' => {
                    map_line.push(Square::Open);
                }
                '#' => {
                    map_line.push(Square::Wall);
                }
                char => panic!("unexpected character '{char}'"),
            }
        }
        map.push(map_line);
    }
    (map, start_pos.unwrap(), end_pos.unwrap())
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Square>>) {
    //std::thread::sleep(std::time::Duration::from_millis(50));
    print!("\x1B[1;1H");
    for line in map {
        for square in line {
            match square {
                Square::End => print!("E"),
                Square::Open => print!("."),
                Square::Wall => print!("#"),
            }
        }
        println!();
    }
}

fn get_graph(map: &[Vec<Square>]) -> HashMap<Vertex, Vec<Vertex>> {
    let mut graph = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !matches!(map[y][x], Square::Open | Square::End) {
                continue;
            }
            for direction in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let mut neighbors = Vec::new();

                if direction == Direction::North || direction == Direction::South {
                    neighbors.push(Vertex {
                        position: (x, y),
                        direction: Direction::West,
                    });
                    neighbors.push(Vertex {
                        position: (x, y),
                        direction: Direction::East,
                    });
                } else {
                    neighbors.push(Vertex {
                        position: (x, y),
                        direction: Direction::North,
                    });
                    neighbors.push(Vertex {
                        position: (x, y),
                        direction: Direction::South,
                    });
                }

                if direction == Direction::North
                    && matches!(map[y - 1][x], Square::Open | Square::End)
                {
                    neighbors.push(Vertex {
                        position: (x, y - 1),
                        direction: Direction::North,
                    });
                } else if direction == Direction::South
                    && matches!(map[y + 1][x], Square::Open | Square::End)
                {
                    neighbors.push(Vertex {
                        position: (x, y + 1),
                        direction: Direction::South,
                    });
                } else if direction == Direction::West
                    && matches!(map[y][x - 1], Square::Open | Square::End)
                {
                    neighbors.push(Vertex {
                        position: (x - 1, y),
                        direction: Direction::West,
                    });
                } else if direction == Direction::East
                    && matches!(map[y][x + 1], Square::Open | Square::End)
                {
                    neighbors.push(Vertex {
                        position: (x + 1, y),
                        direction: Direction::East,
                    });
                }

                graph.insert(
                    Vertex {
                        position: (x, y),
                        direction,
                    },
                    neighbors,
                );
            }
        }
    }

    graph
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct VertexDistance {
    position: Position,
    direction: Direction,
    distance: u32,
}

impl Ord for VertexDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for VertexDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_distances(graph: &HashMap<Vertex, Vec<Vertex>>, start: Vertex) -> HashMap<Vertex, u32> {
    // Dijkstra's algorithm
    let mut visited = HashSet::with_capacity(graph.len());
    let mut all_distances = BinaryHeap::with_capacity(graph.len());
    let mut distances = BinaryHeap::with_capacity(graph.len());

    all_distances.push(VertexDistance {
        position: start.position,
        direction: start.direction,
        distance: 0,
    });
    distances.push(VertexDistance {
        position: start.position,
        direction: start.direction,
        distance: 0,
    });

    let mut previous = HashMap::new();

    while let Some(VertexDistance {
        position,
        direction,
        distance,
    }) = distances.pop()
    {
        visited.insert(Vertex {
            position,
            direction,
        });

        for &neighbor in graph
            .get(&Vertex {
                position,
                direction,
            })
            .unwrap()
        {
            if !visited.contains(&neighbor) {
                let score = if direction == neighbor.direction {
                    1
                } else {
                    1000
                };
                let alt = distance + score;

                let neighbor_distance = distances
                    .iter()
                    .find(
                        |VertexDistance {
                             position,
                             direction,
                             ..
                         }| {
                            *position == neighbor.position && *direction == neighbor.direction
                        },
                    )
                    .map(|&VertexDistance { distance, .. }| distance)
                    .unwrap_or(u32::MAX);

                if alt < neighbor_distance {
                    all_distances.push(VertexDistance {
                        position: neighbor.position,
                        direction: neighbor.direction,
                        distance: alt,
                    });
                    distances.push(VertexDistance {
                        position: neighbor.position,
                        direction: neighbor.direction,
                        distance: alt,
                    });
                    previous.insert(
                        neighbor,
                        Vertex {
                            position,
                            direction,
                        },
                    );
                }
            }
        }
    }

    all_distances
        .into_iter()
        .map(
            |VertexDistance {
                 position,
                 direction,
                 distance,
             }| {
                (
                    Vertex {
                        position,
                        direction,
                    },
                    distance,
                )
            },
        )
        .collect()
}

#[allow(dead_code)]
fn print_scores(map: &[Vec<Square>], scores: &HashMap<(usize, usize, Direction), u32>) {
    //std::thread::sleep(std::time::Duration::from_millis(10));
    print!("\x1B[1;1H\n\n\n\n");
    for y in 0..map.len() {
        print!(" ");
        for x in 0..map[0].len() {
            if map[y][x] == Square::Wall {
                print!("\u{2588}");
            } else if scores.contains_key(&(x, y, Direction::North)) {
                print!("\u{2592}");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn part1(distances: &HashMap<Vertex, u32>, end_pos: Position) -> u32 {
    let score = distances
        .iter()
        .filter_map(|(Vertex { position, .. }, distance)| {
            (*position == end_pos).then_some(*distance)
        })
        .min()
        .unwrap();
    println!("{score}");
    score
}

fn part2(
    graph: &HashMap<Vertex, Vec<Vertex>>,
    distances: &HashMap<Vertex, u32>,
    end_pos: Position,
    best_path_score: u32,
) {
    let mut on_best_path = HashSet::<Position>::with_capacity(distances.len() / 4);

    let distances_from_end = [
        get_distances(
            graph,
            Vertex {
                position: end_pos,
                direction: Direction::North,
            },
        ),
        get_distances(
            graph,
            Vertex {
                position: end_pos,
                direction: Direction::South,
            },
        ),
        get_distances(
            graph,
            Vertex {
                position: end_pos,
                direction: Direction::West,
            },
        ),
        get_distances(
            graph,
            Vertex {
                position: end_pos,
                direction: Direction::East,
            },
        ),
    ];

    for (&vertex, &distance) in distances {
        let distance_to_end = distances_from_end
            .iter()
            .map(|distances| {
                distances
                    .get(&Vertex {
                        position: vertex.position,
                        direction: match vertex.direction {
                            Direction::North => Direction::South,
                            Direction::South => Direction::North,
                            Direction::West => Direction::East,
                            Direction::East => Direction::West,
                        },
                    })
                    .unwrap()
            })
            .min()
            .unwrap();

        if distance + distance_to_end == best_path_score {
            on_best_path.insert(vertex.position);
        }
    }

    println!("{}", on_best_path.len());
}

fn main() {
    let (map, start_pos, end_pos) = parse();
    let graph = get_graph(&map);
    let distances = get_distances(
        &graph,
        Vertex {
            position: start_pos,
            direction: Direction::East,
        },
    );
    let best_path_score = part1(&distances, end_pos);
    part2(&graph, &distances, end_pos, best_path_score);
}
