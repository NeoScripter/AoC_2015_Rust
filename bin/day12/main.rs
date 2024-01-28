use serde_json::{Value, from_str};

fn part1(input: &str) -> i32 {
    let mut nums = String::new();
    input.chars().for_each(|c| {
        if c.is_digit(10) || c == '-' {
            nums.push(c)
        } else {
            nums.push(' ')
        }
    });
    nums.split_whitespace()
    .filter_map(|num| num.parse::<i32>().ok())
    .sum()
}
fn process_char(c: char, rest: &mut String) {
    if c.is_digit(10) || c == '-' {
        rest.push(c)
    } else {
        rest.push(' ')
    }
}

fn sum_numbers(json: &Value, ignore_red: bool) -> i32 {
    match json {
        Value::Number(num) => num.as_i64().unwrap_or(0) as i32,
        Value::Array(arr) => arr.iter().map(|v| sum_numbers(v, ignore_red)).sum(),
        Value::Object(obj) => {
            if ignore_red && obj.values().any(|v| v == "red") {
                0
            } else {
                obj.values().map(|v| sum_numbers(v, ignore_red)).sum()
            }
        }
        _ => 0,
    }
}

fn part2(input: &str) -> i32 {
    let json = from_str(input).expect("Failed to parse JSON");
    sum_numbers(&json, true)
}

fn main() {
    let input = include_str!("input12.txt");
    println!("{:?}", part2(input));
}