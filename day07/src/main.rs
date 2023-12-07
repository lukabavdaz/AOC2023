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
        let mut occurrences = map.into_values().collect::<Vec<_>>();
        occurrences.sort();
        let set = 1000000
            * match &occurrences[..] {
            &[.., 5] => 6,
            &[.., 4] => 5,
            &[.., 2, 3] => 4,
            &[.., 3] => 3,
            &[.., 2, 2] => 2,
            &[.., 2] => 1,
            _ => 0,
        };

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
        .map(|(i, &(_, v))| (i + 1) as u64 * v)
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

        if occurrences.len() == 0 {
            occurrences.push(5);
        } else {
            *occurrences.last_mut().unwrap() += s.chars().filter(|&x| x == 'J').count();
        }
        let set = 1000000
            * match &occurrences[..] {
                &[.., 5] => 6,
                &[.., 4] => 5,
                &[.., 2, 3] => 4,
                &[.., 3] => 3,
                &[.., 2, 2] => 2,
                &[.., 2] => 1,
                _ => 0,
            };

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
        .map(|(i, &(_, v))| (i + 1) as u64 * v)
        .sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}