use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use nom::{
    IResult,
    bytes::complete::tag,
    sequence::tuple,
    character::complete::{alpha1, space1, digit1},
    combinator::{map_res, map},
};

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse::<i32>)(input)
}

fn line_parser(input: &str) -> IResult<&str, (String, String, i32)> {
    map(tuple((alpha1, tag(" would "), alpha1, space1, parse_integer, tag(" happiness units by sitting next to "), alpha1)),|(name1, _, gain_lose, _, value, _, name2)| {
            let value = if gain_lose == "gain" { value } else { -value };
            (name1.to_string(), name2.to_string(), value)
        },
    )(input)
}

#[derive(Debug)]
struct Table {
    layout: HashSet<String>,
    list: HashMap<String, Vec<(String, i32)>>,
}

impl Table {
    fn new() -> Self {
        Self {
            layout: HashSet::new(),
            list: HashMap::new(),
        }
    }
    fn find_layout(&self) -> i32 {
        self.layout.iter().permutations(self.layout.len()).map(|perm| {
            perm.iter().circular_tuple_windows::<(_, _, _)>().map(|(&f, &s, &t)| {
                self.list.get(s).unwrap().iter().filter(|&(n, _)| n == f || n == t).map(|(_, v)| v).sum::<i32>()
            }).sum()
        }).max().unwrap()
    }
}

fn parse_input() -> Table {
    let input = include_str!("input13.txt");
    let mut table = Table::new();
    for line in input.lines() {
        let (_, (n1, n2, value)) = line_parser(line).unwrap();
        table.layout.insert(n1.clone());
        table.list.entry(n1).or_insert(Vec::new()).push((n2, value));
    }
    table
}
fn part1() -> i32 {
    let table = parse_input();
    table.find_layout()
}

fn part2() -> i32 {
    let mut table = parse_input();
    table.layout.iter().for_each(|name| {
        table.list.entry(name.clone()).or_default().push(("Me".to_string(), 0));
        table.list.entry("Me".to_string()).or_default().push((name.clone(), 0));
    });
    table.layout.insert("Me".to_string());

    table.find_layout()
}
fn main() {
    println!("{}", part2());
}