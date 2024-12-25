use std::{
    collections::{HashMap, HashSet},
    sync::RwLock,
};

use lazy_static::lazy_static;
use rayon::iter::{IntoParallelIterator as _, IntoParallelRefIterator as _, ParallelIterator as _};

lazy_static! {
    static ref CACHE: RwLock<HashMap<String, HashSet<String>>> = RwLock::default();
}

fn parse() -> HashSet<(String, String)> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            (line[..2].to_string(), line[3..].to_string())
        })
        .collect()
}

fn get_neighbors(input: HashSet<(String, String)>) -> HashMap<String, HashSet<String>> {
    let mut neighbors: HashMap<String, HashSet<String>> = HashMap::new();

    for (id1, id2) in input {
        neighbors
            .entry(id1.clone())
            .or_default()
            .insert(id2.clone());
        neighbors.entry(id2).or_default().insert(id1);
    }
    neighbors
}

fn part1(neighbors: &HashMap<String, HashSet<String>>) {
    let mut connected_sets = HashSet::new();

    for (id1, ids2) in neighbors {
        for id2 in ids2 {
            let ids3 = neighbors.get(id2).unwrap();
            for id3 in ids3 {
                if ids2.contains(id3) {
                    let mut set = [id1, id2, id3];
                    set.sort();
                    connected_sets.insert(set);
                }
            }
        }
    }

    let count = connected_sets
        .into_iter()
        .filter(|[id1, id2, id3]| {
            id1.starts_with('t') || id2.starts_with('t') || id3.starts_with('t')
        })
        .count();

    println!("{count}");
}

fn stringify(set: &HashSet<String>) -> String {
    let mut items = set.iter().map(|string| string.as_ref()).collect::<Vec<_>>();
    items.sort();
    items.join(",")
}

fn maximal_connected_set(
    set: HashSet<String>,
    neighbors: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    {
        let cache = CACHE.read().unwrap();
        if let Some(cached) = cache.get(&stringify(&set)) {
            return cached.clone();
        }
    }

    let common_neighbors = set
        .iter()
        .map(|id| neighbors.get(id).unwrap().clone())
        .reduce(|acc, e| acc.intersection(&e).cloned().collect())
        .unwrap();
    if common_neighbors.is_empty() {
        return set;
    }
    let rv = common_neighbors
        .par_iter()
        .map(|common_neighbor| {
            let mut set = set.to_owned();
            set.insert(common_neighbor.to_owned());
            maximal_connected_set(set, neighbors)
        })
        .max_by_key(|set| set.len())
        .unwrap();
    let mut cache = CACHE.write().unwrap();
    cache.insert(stringify(&set), rv.clone());
    rv
}

fn part2(neighbors: &HashMap<String, HashSet<String>>) {
    let max_set = neighbors
        .into_par_iter()
        .map(|(id, _)| maximal_connected_set(HashSet::from([id.to_owned()]), neighbors))
        .max_by_key(|set| set.len())
        .unwrap();
    println!("{}", stringify(&max_set));
}

fn main() {
    let input = parse();
    let neighbors = get_neighbors(input);
    part1(&neighbors);
    part2(&neighbors);
}
