fn get_input() -> Vec<String> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|l| 10 * l[0] + l[l.len() - 1])
        .sum()
}

//Should be faster for longer strings
//Could be more efficient by extracting the digit check out of find_number
fn part2_v1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|l| {
            let first = (1..=l.len()).find_map(|x| find_number(&l[x.saturating_sub(5)..x]));
            let last = (1..=l.len())
                .find_map(|x| find_number(&l[l.len() - x..l.len().min(l.len() - x + 5)]));
            10 * first.unwrap() + last.unwrap()
        })
        .sum()
}

fn find_number(ss: &str) -> Option<u32> {
    [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .position(|(i, n)| ss.contains(n) || ss.contains(char::from_digit(i as u32 + 1, 10).unwrap()))
    .map(|i| (i + 1) as u32)
}

//Simple solution, faster for my input
fn part2_v2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|l| {
            [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .enumerate()
            .fold(l.clone(), |acc, (i, s)| {
                acc.replace(s, &format!("{s}{}{s}", i + 1))
            })
        })
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|l| 10 * l[0] + l[l.len() - 1])
        .sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2_v1: {}", part2_v1(&input));
    println!("part2_v2: {}", part2_v2(&input));
}