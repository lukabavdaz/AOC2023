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

fn part2_v5(input: &[String]) -> u32 {
    part1(input.iter().map(|l| {
        [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
            .iter()
            .enumerate()
            .fold(l.clone(), |acc, (i, s)| {
                acc.replace(s, &format!("{}{}{}", &s[0..1], i + 1, &s[s.len()-1..s.len()]))
            })
    }))
}

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

fn part2_v4(input: &[String]) -> u32 {
    input
        .iter()
        .map(|l| {
            let first = (1..=l.len()).find_map(|x| (&l[x-1..x]).parse::<u32>().ok().or(find_number2(&l[x.saturating_sub(5)..x])));
            let last = (1..=l.len())
                .find_map(|x| (&l[l.len()-x..l.len()-x+1]).parse::<u32>().ok().or(find_number2(&l[l.len() - x..l.len().min(l.len() - x + 5)])));
            10 * first.unwrap() + last.unwrap()
        })
        .sum()
}

fn find_number2(ss: &str) -> Option<u32> {
    [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
        .iter()
        .position(|n| ss.contains(n))
        .map(|i| (i + 1) as u32)
}


fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    let t0 = std::time::Instant::now();
    for _ in 0..100 {
        println!("part2: {}", part2(&input));
    }
    let t1 = std::time::Instant::now();
    for _ in 0..100 {
        println!("part2: {}", get_calibration_sum(&input, true));
    }
    let t2 = std::time::Instant::now();
    for _ in 0..100 {
        println!("part2: {}", part2_v1(&input));
    }
    let t3 = std::time::Instant::now();
    for _ in 0..100 {
        println!("part2: {}", part2_v4(&input));
    }
    let t4 = std::time::Instant::now();
    for _ in 0..100 {
        println!("part2: {}", part2_v5(&input));
    }
    let t5 = std::time::Instant::now();
    println!("part2 luka v2: {}",(t1-t0).as_millis());
    println!("part2 fabian: {}",(t2-t1).as_millis());
    println!("part2 luka v1: {}",(t3-t2).as_millis());
    println!("part2 luka v1 optimized: {}",(t4-t3).as_millis());
    println!("part2 luka v2 optimized: {}",(t5-t4).as_millis());
}

fn get_calibration_sum(data: &Vec<String>, spelled: bool) -> i32{
    let mut sum:i32 = 0;
    // Create vector with valid digits
    let mut digits:Vec<&str> = vec!["1","2","3","4","5","6","7","8","9"];
    if spelled{
        digits.append(&mut vec!["one","two","three","four","five","six","seven","eight","nine"])
    }
    // Loop over every line
    for line in data{
        let minInd = digits.clone().iter().map(|x| line.find(x) ).enumerate().filter(|(_, v)| v.is_some()).min_by(|(_, v), (_, v2)| v.cmp(v2)).map(|(idx, _)| idx).unwrap();
        let maxInd = digits.clone().iter().map(|x| line.rfind(x)).enumerate().filter(|(_, v)| v.is_some()).max_by(|(_, v), (_, v2)| v.cmp(v2)).map(|(idx, _)| idx).unwrap();
        sum += format!("{}{}", (minInd % 9) + 1, (maxInd % 9) + 1).parse::<i32>().unwrap();
    }
    return sum
}