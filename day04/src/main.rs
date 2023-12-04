fn get_input() -> Vec<Vec<Vec<u64>>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| {
            s.split(&[':', '|'])
                .skip(1)
                .map(|ss| {
                    ss.split_whitespace()
                        .map(|sss| sss.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<Vec<u64>>]) -> u64 {
    input
        .iter()
        .map(|v| match v[0].iter().filter(|x| v[1].contains(x)).count() {
            0 => 0,
            n => 2_u64.pow(n as u32 - 1),
        })
        .sum()
}

fn part2(input: &[Vec<Vec<u64>>]) -> usize {
    let wins = input
        .iter()
        .map(|v| v[0].iter().filter(|x| v[1].contains(x)).count())
        .collect::<Vec<usize>>();

    let mut cards = vec![1; wins.len()];
    for i in 0..cards.len() {
        for j in (i + 1)..(i + 1 + wins[i]) {
            cards[j] += cards[i]
        }
    }
    cards.iter().sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}