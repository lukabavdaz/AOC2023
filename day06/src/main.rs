fn part1(input: &str) -> u64 {
    let numbers = input
        .lines()
        .map(|s| {
            s.split(|c: char| !c.is_numeric())
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect::<Vec<Vec<u64>>>();

    numbers[0]
        .iter()
        .zip(numbers[1].iter())
        .map(|(&t, x)| {
            ((t as f64 + ((t * t - 4 * x) as f64).sqrt()) / 2.0) as u64
                - ((t as f64 - ((t * t - 4 * x) as f64).sqrt()) / 2.0) as u64
        })
        .product()
}

fn part2(input: &str) -> u64 {
    part1(&input.replace(' ', ""))
}

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}