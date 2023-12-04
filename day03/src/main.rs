fn get_input() -> Vec<Vec<char>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

fn part1(input: &[Vec<char>]) -> u32 {
    let mut sum = 0;
    for (y, l) in input.iter().enumerate() {
        let mut num = 0;
        let mut symbol_found = false;
        for (x, c) in l.iter().enumerate() {
            if !c.is_numeric() {
                if symbol_found {
                    sum += num;
                }
                num = 0;
                symbol_found = false;
            } else {
                num = num * 10 + c.to_digit(10).unwrap();
                for j in y.saturating_sub(1)..input.len().min(y + 2) {
                    for i in x.saturating_sub(1)..l.len().min(x + 2) {
                        if !input[j][i].is_numeric() && input[j][i] != '.' {
                            symbol_found = true;
                        }
                    }
                }
            }
        }
        if symbol_found {
            sum += num;
        }
    }
    sum
}

fn part2(input: &[Vec<char>]) -> u32 {
    let mut found = std::collections::HashMap::new();
    let mut sum = 0;
    for (y, l) in input.iter().enumerate() {
        let mut num = 0;
        let mut symbol_found = None;
        for (x, c) in l.iter().enumerate() {
            if !c.is_numeric() {
                if let Some(p) = symbol_found {
                    if let Some(other) = found.get(&p) {
                        sum += num * other;
                    } else {
                        found.insert(p, num);
                    }
                }
                num = 0;
                symbol_found = None;
            } else {
                num = num * 10 + c.to_digit(10).unwrap();
                for j in y.saturating_sub(1)..input.len().min(y + 2) {
                    for i in x.saturating_sub(1)..l.len().min(x + 2) {
                        if !input[j][i].is_numeric() && input[j][i] != '.' {
                            symbol_found = Some((i,j));
                        }
                    }
                }
            }
        }
        if let Some(p) = symbol_found {
            if let Some(other) = found.get(&p) {
                sum += num * other;
            } else {
                found.insert(p, num);
            }
        }
    }
    sum
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

// for s in l.split(|c: char| !c.is_numeric()) {
// if s.is_empty() {
// offset += 1;
// } else {
// let n = s.parse::<i64>().unwrap();
// println!("{n}");
// if (y > 0 && input[y-1][offset.saturating_sub(1)..l.len().min(offset+s.len()+1)].contains(|c: char| !c.is_numeric() && c != '.')) ||
// dbg!(y < l.len() - 1 && input[y+1][offset.saturating_sub(1)..l.len().min(offset+s.len()+1)].contains(|c: char| !c.is_numeric() && c != '.')) ||
// input[y][offset.saturating_sub(1)..offset].contains(|c: char| !c.is_numeric() && c != '.') ||
// input[y][offset+s.len()..l.len().min(offset+s.len()+1)].contains(|c: char| !c.is_numeric() && c != '.') {
// sum += n;
// }
// offset += s.len();
// }
// }