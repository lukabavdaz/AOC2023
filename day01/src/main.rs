fn get_input() -> Vec<String> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

fn part1(input: impl IntoIterator<Item = impl AsRef<str>>) -> u32 {
    input
        .into_iter()
        .map(|l| {
            l.as_ref()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|l| 10 * l[0] + l[l.len() - 1])
        .sum()
}

fn part2(input: &[String]) -> u32 {
    part1(input.iter().map(|l| {
        [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
            .iter()
            .enumerate()
            .fold(l.clone(), |acc, (i, s)| {
                acc.replace(s, &format!("{s}{}{s}", i + 1))
            })
    }))
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2_v2: {}", part2(&input));
}