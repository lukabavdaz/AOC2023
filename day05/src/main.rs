fn get_input() -> Vec<Vec<Vec<usize>>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|ss| ss.split(' ').filter_map(|n| n.parse().ok()).collect())
                .filter(|v: &Vec<_>| !v.is_empty())
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<Vec<usize>>]) -> usize {
    input[0][0]
        .iter()
        .map(|&s| {
            input[1..].iter().fold(s, |i, m| {
                m.iter()
                    .find_map(|v| {
                        if v[1] <= i && i < v[1] + v[2] {
                            Some(v[0] + i - v[1])
                        } else {
                            None
                        }
                    })
                    .unwrap_or(i)
            })
        })
        .min()
        .unwrap()
}

fn part2_v1(input: &[Vec<Vec<usize>>]) -> usize {
    let seeds = input[0][0].chunks(2).map(|c| [c[0], c[1]]).collect();

    input[1..]
        .iter()
        .fold(seeds, |ranges: Vec<[usize; 2]>, m| {
            let mut v = vec![];
            for r in ranges {
                let mut v_premap = vec![];
                for m_r in m {
                    if (r[0] + r[1]) >= m_r[1] && (m_r[1] + m_r[2]) >= r[0] {
                        v_premap.push([
                            r[0].max(m_r[1]),
                            (r[0] + r[1]).min(m_r[1] + m_r[2]) - r[0].max(m_r[1]),
                        ]);
                        v.push([
                            m_r[0] + r[0].max(m_r[1]) - m_r[1],
                            (r[0] + r[1]).min(m_r[1] + m_r[2]) - r[0].max(m_r[1]),
                        ]);
                    }
                }
                v_premap.sort_by(|a, b| a[0].cmp(&b[0]));
                let mut i = r[0];
                let mut filler = vec![];
                for new_r in &v_premap {
                    if i < new_r[0] {
                        filler.push([i, new_r[0] - i]);
                    }
                    i += new_r[1]
                }
                if i < r[0] + r[1] {
                    filler.push([i, r[0] + r[1] - i]);
                }
                v.extend(filler);
            }
            v
        })
        .iter()
        .map(|r| r[0])
        .min()
        .unwrap()
}

fn part2_v2(input: &[Vec<Vec<usize>>]) -> usize {
    let seeds = input[0][0].chunks(2).map(|s| [s[0], s[1]]).collect();

    input[1..]
        .iter()
        .fold(seeds, |ranges: Vec<[usize; 2]>, map| {
            let mut mapped = vec![];
            let same = map.iter().fold(ranges, |todo, m| {
                let mut unmapped = vec![];
                for r in todo {
                    if r[0] < m[1] {
                        unmapped.push([r[0], r[1].min(m[1] - r[0])]);
                    }
                    if r[0] + r[1] > m[1] && m[1] + m[2] > r[0] {
                        mapped.push([
                            m[0] + r[0].max(m[1]) - m[1],
                            (r[0] + r[1]).min(m[1] + m[2]) - r[0].max(m[1]),
                        ]);
                    }
                    if r[0] + r[1] > m[1] + m[2] {
                        unmapped.push([r[0].max(m[1] + m[2]), r[1].min(r[0] + r[1] - m[1] - m[2])]);
                    }
                }
                unmapped
            });
            mapped.extend(same);
            mapped
        })
        .iter()
        .map(|r| r[0])
        .min()
        .unwrap()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    let t0 = std::time::Instant::now();
    println!("part2_v1: {}, {:?}", part2_v1(&input), t0.elapsed());
    let t0 = std::time::Instant::now();
    println!("part2_v2: {}, {:?}", part2_v2(&input), t0.elapsed());
}