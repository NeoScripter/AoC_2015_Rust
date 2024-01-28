use std::collections::{HashSet, HashMap};

fn process<'a>(lines: &Vec<&'a str>, signals: &mut HashMap<&'a str, u16>, seen: &mut HashSet<&'a str>) {
    while seen.len() != lines.len() {
        for line in lines.iter() {
            if seen.contains(line) {
                continue
            }
            if line.contains("NOT") {
                let (left, right) = line.trim().split_once(" -> ").unwrap();
                if let Some(num) = signals.get(&left.trim_start_matches("NOT ")) {
                    signals.insert(right, !num);
                    seen.insert(line);
                }
                if let Ok(num) = (left.trim_start_matches("NOT ")).parse::<u16>() {
                    signals.insert(right, num);
                    seen.insert(line);
                }
            }
            if line.contains("AND") {
                let (left, right) = line.trim().split_once(" -> ").unwrap();
                let (l, r) = left.split_once(" AND ").unwrap();
            
                let l_val = l.parse::<u16>().ok();
                let r_val = r.parse::<u16>().ok();
            
                let result = match (l_val, r_val) {
                    (Some(l_num), Some(r_num)) => Some(l_num & r_num),
                    (Some(l_num), None) => signals.get(r).copied().map(|r_num| l_num & r_num),
                    (None, Some(r_num)) => signals.get(l).copied().map(|l_num| l_num & r_num),
                    (None, None) => {
                        if let (Some(l_num), Some(r_num)) = (signals.get(l), signals.get(r)) {
                            Some(l_num & r_num)
                        } else {
                            None
                        }
                    }
                };
            
                if let Some(res) = result {
                    signals.insert(right, res);
                    seen.insert(line);
                }
            }            
            if line.contains("OR") {
                let (left, right) = line.trim().split_once(" -> ").unwrap();
                let (l, r) = left.split_once(" OR ").unwrap();
            
                let l_val = l.parse::<u16>().ok();
                let r_val = r.parse::<u16>().ok();
            
                let result = match (l_val, r_val) {
                    (Some(l_num), Some(r_num)) => Some(l_num & r_num),
                    (Some(l_num), None) => signals.get(r).copied().map(|r_num| l_num & r_num),
                    (None, Some(r_num)) => signals.get(l).copied().map(|l_num| l_num & r_num),
                    (None, None) => {
                        if let (Some(l_num), Some(r_num)) = (signals.get(l), signals.get(r)) {
                            Some(l_num | r_num)
                        } else {
                            None
                        }
                    }
                };
            
                if let Some(res) = result {
                    signals.insert(right, res);
                    seen.insert(line);
                }
            }    
            if line.contains("LSHIFT") {
                let (left, right) = line.trim().split_once(" -> ").unwrap();
                let(l, r) = left.split_once(" LSHIFT ").unwrap();
                if let Some(num) = signals.get(&l) {
                    signals.insert(right, num << r.parse::<u16>().unwrap());
                    seen.insert(line);
                }
                if let Ok(num) = l.parse::<u16>() {
                    signals.insert(right, num);
                    seen.insert(line);
                }
            }
            if line.contains("RSHIFT") {
                let (left, right) = line.trim().split_once(" -> ").unwrap();
                let(l, r) = left.split_once(" RSHIFT ").unwrap();
                if let Some(num) = signals.get(&l) {
                    signals.insert(right, num >> r.parse::<u16>().unwrap());
                    seen.insert(line);
                }
                if let Ok(num) = l.parse::<u16>() {
                    signals.insert(right, num);
                    seen.insert(line);
                }
            }
            let (left, right) = line.trim().split_once(" -> ").unwrap();
            if let Ok(num) = left.parse::<u16>() {
                signals.insert(right, num);
                seen.insert(line);
            }
            if let Some(num) = signals.get(&left) {
                signals.insert(right, *num);
                seen.insert(line);
            }
        }
    }
}
fn part1(input: &str) -> u16 {
    let lines: Vec<&str> = input.lines().collect();
    let mut signals: HashMap<&str, u16> = HashMap::new();
    let mut seen = HashSet::new();
    process(&lines, &mut signals, &mut seen);
    *signals.get(&"a").unwrap()
}
fn part2(input: &str) -> u16 {
    let lines: Vec<&str> = input.lines().collect();
    let mut signals = HashMap::new();
    let mut seen = HashSet::new();

    process(&lines, &mut signals, &mut seen);

    let signal_a = match signals.get("a") {
        Some(&value) => value,
        None => panic!("Signal 'a' not found after initial processing"),
    };

    signals.clear();
    signals.insert("b", signal_a);
    seen.clear();
    
    let b_line = lines.iter().find(|&&line| line.ends_with(" -> b"))
                      .expect("No line ending with ' -> b' found");

    seen.insert(b_line);
    process(&lines, &mut signals, &mut seen);
    *signals.get("a").expect("Signal 'a' not found after second processing")
}

fn main() {
    let input = include_str!("input7.txt");
    println!("{}", part2(input));
}