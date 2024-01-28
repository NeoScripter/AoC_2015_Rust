use nom::{
    IResult,
    bytes::complete::tag,
    sequence::{tuple, preceded},
    character::complete::{alpha1, digit1},
    combinator::{map_res, map, recognize, opt},
};

#[derive(Debug, Clone)]
struct Recipe<'a> (Vec<Ing<'a>>);

impl<'a> FromIterator<Ing<'a>> for Recipe<'a> {
    fn from_iter<I: IntoIterator<Item = Ing<'a>>>(iter: I) -> Self {
        let mut recipe = Vec::new();
        for ing in iter { recipe.push(ing) }
        Recipe(recipe)
    }
}

impl<'a> Recipe<'a> {
    fn find_score(&self, elm: usize, trg_sum: i64, part2: bool) -> i64 {
        find_cmbs(elm, trg_sum).iter().filter_map(|cmb| {
            let clrs = self.0.iter().zip(cmb).map(|(ing, spns)| { ing.prts[4] * spns }).sum::<i64>();
            (0..4).map(|idx| {
                let mut sum = self.0.iter().zip(cmb).map(|(ing, spns)| { ing.prts[idx] * spns }).sum::<i64>();
                sum = sum.max(0);
                if part2 { if clrs == 500 { Some(sum) } else { None } } 
                else { Some(sum)}
            }).product()
        }).max().unwrap()
    }
}

#[derive(Debug, Clone)]
struct Ing<'a> {
    name: &'a str,
    prts: Vec<i64>,
}

impl<'a> Ing<'a> {
    fn new(name: &'a str, prts: Vec<i64>,) -> Self {
        Self { name, prts }
    }
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        str::parse::<i64>(s)
    })(input)
}

fn line_parser(input: &str) -> IResult<&str, (&str, i64, i64, i64, i64, i64)> {
    map(tuple((alpha1, tag(": capacity "), parse_i64, tag(", durability "), parse_i64, tag(", flavor "), parse_i64, tag(", texture "), parse_i64, tag(", calories "), parse_i64)),|(name, _, cap, _, dur, _, flr, _, txt, _, clrs)| {
        (name, cap, dur, flr, txt, clrs)
        },
    )(input)
}

fn cmbs_rec(cur_sum: i64, rem_nbs: usize, trg_sum: i64, cur_comb: &mut Vec<i64>, results: &mut Vec<Vec<i64>>) {
    if rem_nbs == 1 {
        let last_number = trg_sum - cur_sum;
        if last_number >= 0 {
            cur_comb.push(last_number);
            results.push(cur_comb.clone());
            cur_comb.pop();
        }
    } else {
        for i in 0..=trg_sum - cur_sum {
            cur_comb.push(i);
            cmbs_rec(cur_sum + i, rem_nbs - 1, trg_sum, cur_comb, results);
            cur_comb.pop();
        }
    }
}

fn find_cmbs(elms: usize, trg_sum: i64) -> Vec<Vec<i64>> {
    let mut results = Vec::new();
    cmbs_rec(0, elms, trg_sum, &mut Vec::new(), &mut results);
    results
}

fn parse_input<'a>() -> Recipe<'a> {
    let input = include_str!("input15.txt");
    let recipe: Recipe = input.lines().map(|l| {
        let (_, (name, cap, dur, flr, txt, clrs)) = line_parser(l).unwrap();
        let prts: Vec<i64> = vec![cap, dur, flr, txt, clrs];
        Ing::new(name, prts)
    }).collect();
    recipe
}
fn part1() -> i64 {
    let recipe = parse_input();
    recipe.find_score(4, 100, false)
}

fn part2() -> i64 {
    let recipe = parse_input();
    recipe.find_score(4, 100, true)
}

fn main() {
    println!("{}", part1());
}