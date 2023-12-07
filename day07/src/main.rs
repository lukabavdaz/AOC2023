fn get_input() -> Vec<(String, u64)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(a, b)| (a.to_owned(), b.parse().unwrap()))
        .collect()
}

fn part1(input: &[(String, u64)]) -> u64 {
    let mut cards = input.to_vec();
    cards.sort_by_cached_key(|(s, _)| {
        let mut map = std::collections::HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let set = map.values().map(|o| o * o * 1000000).sum::<u64>();

        let values = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        set + s.chars().fold(0, |acc, c| {
            acc * 13 + values.iter().position(|&x| x == c).unwrap() as u64
        })
    });
    cards
        .iter()
        .enumerate()
        .map(|(i, &(_, b))| (i + 1) as u64 * b)
        .sum()
}

fn part2(input: &[(String, u64)]) -> u64 {
    let mut cards = input.to_vec();
    cards.sort_by_cached_key(|(s, _)| {
        let mut map = std::collections::HashMap::new();
        for c in s.chars().filter(|&x| x != 'J') {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut occurrences = map.into_values().collect::<Vec<_>>();

        occurrences.sort();
        if let Some(o) = occurrences.last_mut() {
            *o += s.chars().filter(|&x| x == 'J').count();
        } else {
            occurrences.push(5);
        }
        let set = occurrences.iter().map(|o| (o * o) as u64 * 1000000).sum::<u64>();

        let values = [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];
        set + s.chars().fold(0, |acc, c| {
            acc * 13 + values.iter().position(|&x| x == c).unwrap() as u64
        })
    });
    cards
        .iter()
        .enumerate()
        .map(|(i, &(_, b))| (i + 1) as u64 * b)
        .sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}