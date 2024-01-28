#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_variables)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const TOTAL: u32 = 25;

fn part1() -> usize {
    let input = include_str!("input_lib.txt");
    let cnts: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();

    let min_k = (2..=cnts.len())
        .find(|&k| cnts.iter().combinations(k).any(|c| c.iter().copied().sum::<u32>() == TOTAL)).unwrap();
    
    cnts.iter().combinations(min_k).filter(|c| c.iter().cloned().sum::<u32>() == TOTAL).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, part1());
    }
}