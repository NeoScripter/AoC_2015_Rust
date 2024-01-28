#![allow(unused)]

use std::collections::HashMap;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, char, digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn key_value_pair(s: &str) -> IResult<&str, (&str, u16)> {
    separated_pair(alpha1, tag(": "), map_res(digit1, str::parse::<u16>))(s)
}

fn parse_line(s: &str) -> IResult<&str, Vec<(&str, u16)>> {
    preceded(
        take_until(": "),
        preceded(
            tag(": "),
            separated_list1(tag(", "), key_value_pair),
        ),
    )(s)
}


#[derive(Debug, Clone)]
struct Device<'a> {
    ticker: HashMap<&'a str, u16>,
}

impl<'a> Device<'a> {
    fn new() -> Self {
        let map: HashMap<&'a str, u16> = vec![("children", 3), ("cats", 7), ("samoyeds", 2), ("pomeranians", 3), ("akitas", 0), ("vizslas", 0), ("goldfish", 5), ("trees", 3), ("cars", 2), ("perfumes", 1)].into_iter().collect();
        Device { ticker: map }
    }
    fn compare(&self, aunt: Vec<(&'a str, u16)>) -> bool {
        for (item, qty) in aunt {
            match self.ticker.get(&item) {
                Some(&dct_qty) => {
                    match item {
                        "cats" | "trees" => if qty <= dct_qty { return false },
                        "pomeranians" | "goldfish" => if qty >= dct_qty { return false },
                        _ => if qty != dct_qty { return false },
                    }
                },
                None => return false,
            }
        }
        true
    }
}
fn main() {
    let input = include_str!("input16.txt");
    let device = Device::new();
    for (idx, line) in input.lines().enumerate() {
        let (_, vec) = parse_line(line).unwrap();
        if device.compare(vec) { println!("The answer is {}", idx + 1); }
    }
}