use aoc2024::read_lines;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let start = std::time::Instant::now();

    let (map_w, map_h, locs) = parse_puzzle_input();

    println!(
        "P1: {}",
        calculate_ans(map_w, map_h, &locs, [1i32].into_iter())
    );
    println!("P2: {}", calculate_ans(map_w, map_h, &locs, 0i32..));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn calculate_ans<I>(map_w: i32, map_h: i32, locs: &HashMap<char, Vec<(i32, i32)>>, it: I) -> usize
where
    I: Iterator<Item = i32> + Clone,
{
    let find_antinodes = |a: (i32, i32), b: (i32, i32)| {
        let mut antinode_locs = Vec::new();
        for (p1, p2) in [(a, b), (b, a)] {
            for i in it.clone() {
                let x = p1.0 + i * (p1.0 - p2.0);
                let y = p1.1 + i * (p1.1 - p2.1);
                if x < 0 || y < 0 || x >= map_w || y >= map_h {
                    break;
                } else {
                    antinode_locs.push((x, y));
                }
            }
        }
        antinode_locs
    };

    locs.iter()
        .map(|(_, vs)| {
            vs.iter()
                .combinations(2)
                .map(|v| find_antinodes(*v[0], *v[1]))
                .flatten()
        })
        .flatten()
        .unique()
        .count()
}

fn parse_puzzle_input() -> (i32, i32, HashMap<char, Vec<(i32, i32)>>) {
    let mut map_w: i32 = 0;
    let mut map_h: i32 = 0;
    let mut locs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, line) in read_lines("input/day_8.txt").unwrap().flatten().enumerate() {
        map_h = max(map_h, (y + 1) as i32);
        map_w = max(map_w, line.len() as i32);

        for (x, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
            let pos = (x as i32, y as i32);
            if let Some(v) = locs.get_mut(&c) {
                v.push(pos);
            } else {
                locs.insert(c, vec![pos]);
            }
        }
    }

    (map_w, map_h, locs)
}
