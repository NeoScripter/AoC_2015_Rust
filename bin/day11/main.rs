

fn is_valid(b: &Vec<u8>) -> bool {
    let forbidden = [b'i', b'l', b'o'];
    let first = b.windows(3).any(|win| win[0] + 1 == win[1] && win[1] + 1 == win[2]);
    let second = b.iter().all(|c| !forbidden.contains(c));
    let third = b.windows(2).enumerate().any(|(idx, win)| {
        if win[0] == win[1] {
            b.windows(2).skip(idx + 2).any(|win2| win2[0] == win2[1])
        } else {
            false
        }
    });
    first && second && third
}
fn part1(input: String) -> String {
    let mut b: Vec<u8> = input.into_bytes();
    let len = b.len() - 1;
    let start = b'a';
    let end = b'z';
    let mut idx = len;

    loop {
        if b[idx] < end {
            b[idx] += 1;
            idx = len;
        } else if b[idx] == end {
            b[idx] = start;
            idx -= 1;
        }
        if is_valid(&b) {
            break
        }
    }
    String::from_utf8(b).expect("Invalid UTF-8")
}
fn main() {
    let input = "cqjxjnds".to_string();
    let first_pas = part1(input);
    println!("{}", part1(first_pas));
}