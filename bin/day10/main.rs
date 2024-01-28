fn part1(input: String) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    let mut count = 0;

    while let Some(next) = chars.next() {
        count += 1;
        if chars.peek() != Some(&next) {
            result.push_str(&count.to_string());
            result.push(next);
            count = 0;
        }
    }

    result
}

fn main() {
    let mut result = String::from("1113222113");
    for _ in 0..50 {
        result = part1(result);
    }
    println!("{}", result.len());
}