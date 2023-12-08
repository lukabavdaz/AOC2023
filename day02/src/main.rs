fn get_input() -> Vec<Vec<[u32; 3]>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(&[';', ':'])
                .skip(1)
                .map(|s| {
                    s.split(',').fold([0; 3], |[r, g, b], ss| {
                        match ss.trim().split_once(' ').unwrap() {
                            (n, "red") => [r + n.parse::<u32>().unwrap(), g, b],
                            (n, "green") => [r, g + n.parse::<u32>().unwrap(), b],
                            (n, "blue") => [r, g, b + n.parse::<u32>().unwrap()],
                            _ => panic!(),
                        }
                    })
                })
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<[u32; 3]>]) -> usize {
    input
        .iter()
        .zip(1..)
        .filter(|(v, _)| v.iter().all(|&[r, g, b]| r < 13 && g < 14 && b < 15))
        .map(|(_, i)| i)
        .sum()
}

fn part2(input: &[Vec<[u32; 3]>]) -> u32 {
    input
        .iter()
        .map(|v| {
            v.iter().fold([0; 3], |[r, g, b], &[r2, g2, b2]| {
                [r.max(r2), g.max(g2), b.max(b2)]
            })
        })
        .map(|[r, g, b]| r * g * b)
        .sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}