use itertools::Itertools;

const TOTAL: u32 = 150;

fn parse() -> Vec<u32> {
    let input = include_str!("input17.txt");
    let cnts: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    cnts
}
fn part1() -> usize {
    let cnts = parse();
    (2..=cnts.len())
    .map(|l| cnts.iter().combinations(l)
        .filter(|c| c.iter().copied().sum::<u32>() == TOTAL)
    )
    .flatten()
    .count()
}

fn part2() -> usize {
    let cnts = parse();
    let min_k = (2..=cnts.len())
        .find(|&k| cnts.iter().combinations(k).any(|c| c.iter().copied().sum::<u32>() == TOTAL)).unwrap();
    
    cnts.iter().combinations(min_k).filter(|c| c.iter().copied().sum::<u32>() == TOTAL).count()
}
fn main() {
    println!("{}", part2());
}