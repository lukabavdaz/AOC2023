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
    let (seeds, maps) = input.split_at(1);
    seeds[0][0]
        .iter()
        .map(|&s| {
            maps.iter().fold(s, |i, m| {
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

fn part2(input: &[Vec<Vec<usize>>]) -> usize {
    let (seeds, maps) = input.split_at(1);
    let seed_pairs = seeds[0][0].chunks(2).map(|c| c.to_vec()).collect();

    maps.iter()
        .fold(seed_pairs, |ranges: Vec<Vec<usize>>, m| {
            let mut v = vec![];
            for r in ranges {
                let mut v_premap = vec![];
                for m_r in m {
                    if (r[0] + r[1]) >= m_r[1] && (m_r[1] + m_r[2]) >= r[0] {
                        v_premap.push(vec![
                            r[0].max(m_r[1]),
                            (r[0] + r[1]).min(m_r[1] + m_r[2]) - r[0].max(m_r[1]),
                        ]);
                        v.push(vec![
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
                        filler.push(vec![i, new_r[0] - i]);
                    }
                    i += new_r[1]
                }
                if i < r[0] + r[1] {
                    filler.push(vec![i, r[0] + r[1] - i]);
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

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}