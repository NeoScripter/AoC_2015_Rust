use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn part1(input: &str) -> u32 {
    let mut dis_map = HashMap::new();
    let mut city_map: HashMap<&str, Vec<&str>> = HashMap::new();
    
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (city1, city2, dist) = (parts[0], parts[2], parts[4].parse::<u32>().unwrap());
        dis_map.entry((city1, city2)).or_insert(dist);
        dis_map.entry((city2, city1)).or_insert(dist);
        city_map.entry(city1).or_default().push(city2);
        city_map.entry(city2).or_default().push(city1);
    });

    let cities: HashSet<&str> = city_map.keys().copied().collect();
    let mut max_distance = 0;

    let start_time = Instant::now();
    for &city in &cities {
        for path in find_paths(city, &city_map, cities.clone(), Vec::new()) {
            let distance = path.windows(2)
                .map(|w| *dis_map.get(&(&w[0], &w[1])).unwrap())
                .sum();
            max_distance = max_distance.max(distance);
        }
    }

    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);
    max_distance
}
fn find_paths(
    city: &str,
    city_map: &HashMap<&str, Vec<&str>>,
    mut remaining: HashSet<&str>,
    mut path: Vec<String>,
) -> Vec<Vec<String>> {
    path.push(city.to_string());
    remaining.remove(city);

    if remaining.is_empty() {
        return vec![path];
    }

    match city_map.get(city) {
        Some(available) => {
            let mut all_paths = Vec::new();
            for av in available {
                if remaining.contains(av) {
                    let paths = find_paths(av, city_map, remaining.clone(), path.clone());
                    all_paths.extend(paths);
                }
            }
            all_paths
        }
        None => Vec::new(),
    }
}
fn main() {
    let input = include_str!("input9.txt");
    println!("{}", part1(input));
}