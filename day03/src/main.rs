fn get_input() -> Vec<String> {
    std::fs::read_to_string("input/test_input.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

fn part1(input: &[String]) -> u32 {
    let mut sum = 0;
    for (y, l) in input.iter().enumerate() {
        let mut x: usize = 0;
        for s in l.split(|c: char| !c.is_numeric()) {
            if s.len() > 0
                && (y.saturating_sub(1)..input.len().min(y + 2)).any(|j| {
                    input[j][x.saturating_sub(1)..l.len().min(x + s.len() + 1)]
                        .contains(|c: char| !c.is_numeric() && c != '.')
                })
            {
                sum += s.parse::<u32>().unwrap();
            }
            x += s.len() + 1;
        }
    }
    sum
}

fn part2(input: &[String]) -> usize {
    let mut found = std::collections::HashMap::new();
    for (y, l) in input.iter().enumerate() {
        let mut x: usize = 0;
        for s in l.split(|c: char| !c.is_numeric()) {
            if s.len() > 0 {
                if let Some(p) = (y.saturating_sub(1)..input.len().min(y + 2)).find_map(|j| {
                    let slice = x.saturating_sub(1)..l.len().min(x + s.len() + 1);
                    Some((j, x.saturating_sub(1) + input[j][slice].find('*')?))
                }) {
                    found.entry(p).or_insert(vec![]).push(s.parse().unwrap());
                }
            }
            x += s.len() + 1;
        }
    }
    found
        .values()
        .map(|v| (v.len() - 1) * v.iter().product::<usize>())
        .sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}