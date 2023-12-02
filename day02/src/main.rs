fn get_input() -> Vec<Vec<[u32; 3]>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(&[';', ':'])
                .skip(1)
                .map(|s| {
                    s.split(',')
                        .fold([0; 3], |acc, ss| match ss.trim().split_once(' ').unwrap() {
                            (n, "red") => [acc[0] + n.parse::<u32>().unwrap(), acc[1], acc[2]],
                            (n, "green") => [acc[0], acc[1] + n.parse::<u32>().unwrap(), acc[2]],
                            (n, "blue") => [acc[0], acc[1], acc[2] + n.parse::<u32>().unwrap()],
                            _ => panic!(),
                        })
                })
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<[u32; 3]>]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, v)| v.iter().all(|c| c[0] < 13 && c[1] < 14 && c[2] < 15))
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &[Vec<[u32; 3]>]) -> u32 {
    input
        .iter()
        .map(|v| {
            v.iter().fold([0, 0, 0], |acc, x| {
                [acc[0].max(x[0]), acc[1].max(x[1]), acc[2].max(x[2])]
            })
        })
        .map(|x| x[0] * x[1] * x[2])
        .sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}