use nom::{
    IResult,
    bytes::complete::tag,
    sequence::tuple,
    character::complete::{alpha1, digit1},
    combinator::{map_res, map},
};
use std::iter::FromIterator;

fn parse_integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse::<u32>)(input)
}

fn line_parser(input: &str) -> IResult<&str, (String, u32, u32, u32)> {
    map(tuple((alpha1, tag(" can fly "), parse_integer, tag(" km/s for "), parse_integer, tag(" seconds, but then must rest for "), parse_integer, tag(" seconds."))),|(name, _, speed, _, interval, _, rest_time, _)| {
            (name.to_string(), speed, interval, rest_time)
        },
    )(input)
}

#[derive(Debug, Clone)]
struct Deer {
    name: String,
    sp: u32,
    run_time: u32,
    rest_time: u32,
    ran: u32,
    points: u32,
}

impl Deer {
    fn new(name: String, sp: u32, run_time: u32, rest_time: u32) -> Self {
        Self { name, sp, run_time, rest_time, ran: 0, points: 0 }
    }
    fn race(&mut self, time: u32) {
        let cycle = self.run_time + self.rest_time;
        let full_cycles = time / cycle;
        let left = time % cycle;
        if left >= self.run_time { self.ran = self.sp * self.run_time * (full_cycles + 1) }
        else { self.ran = self.sp * self.run_time * full_cycles + self.sp * left } 
    }
}

#[derive(Debug, Clone)]
struct Deers (Vec<Deer>);

impl FromIterator<Deer> for Deers {
    fn from_iter<I: IntoIterator<Item = Deer>>(iter: I) -> Self {
        let deers = iter.into_iter().collect::<Vec<Deer>>();
        Deers(deers)
    }
}

impl Deers {
    fn update(&mut self, time: u32) {
        self.0.iter_mut().for_each(|deer| deer.race(time));
    }
    fn find_winner(&self) -> u32 {
        self.0.iter().max_by(|x, y| x.ran.cmp(&y.ran)).map(|winner| winner.ran).unwrap()
    }
    fn give_points(&mut self) {
        let lead = self.find_winner();
        (0..self.0.len()).for_each(|i| {
            if self.0[i].ran == lead { self.0[i].points += 1 }
        })
    }
    fn highest_score(&self) -> u32 {
        self.0.iter().max_by(|x, y| x.points.cmp(&y.points)).map(|winner| winner.points).unwrap()
    }
}

fn parse() -> Deers {
    let input = include_str!("input14.txt");
    let drs: Deers = input.lines().map(|l| {
        let (_, (name, sp, int, rest)) = line_parser(l).unwrap();
        Deer::new(name, sp, int, rest)
    }).collect();
    drs
}
fn part1(secs: u32) -> u32 {
    let mut drs = parse();
    drs.update(secs);
    drs.find_winner()
}

fn part2(secs: u32) -> u32 {
    let mut drs = parse();

    for s in 1..=secs { 
        drs.update(s); 
        drs.give_points();
    }
    
    drs.highest_score()
}
fn main() {
    println!("{}", part2(2503));
}