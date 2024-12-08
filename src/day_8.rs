use aoc2024::read_lines;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let start = std::time::Instant::now();

    let (map_w, map_h, locs) = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(map_w, map_h, &locs));
    println!("P2: {}", calculate_p2_ans(map_w, map_h, &locs));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn calculate_p1_ans(map_w: i32, map_h: i32, locs: &HashMap<char, Vec<(i32, i32)>>) -> usize {
    locs.iter()
        .map(|(_, vs)| {
            vs.iter()
                .combinations(2)
                .map(|v| {
                    vec![
                        (v[0].0 + (v[0].0 - v[1].0), v[0].1 + (v[0].1 - v[1].1)),
                        (v[1].0 + (v[1].0 - v[0].0), v[1].1 + (v[1].1 - v[0].1)),
                    ]
                })
                .flatten()
                .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < map_w && *y < map_h)
        })
        .flatten()
        .unique()
        .count()
}

fn calculate_p2_ans(map_w: i32, map_h: i32, locs: &HashMap<char, Vec<(i32, i32)>>) -> usize {
    locs.iter()
        .map(|(_, vs)| {
            vs.iter()
                .combinations(2)
                .map(|v| {
                    let mut antinode_locs: Vec<(i32, i32)> = vec![];
                    for i in 0.. {
                        let x = v[0].0 + i * (v[0].0 - v[1].0);
                        let y = v[0].1 + i * (v[0].1 - v[1].1);
                        if x < 0 || y < 0 || x >= map_w || y >= map_h {
                            break;
                        } else {
                            antinode_locs.push((x, y));
                        }
                    }
                    for i in 0.. {
                        let x = v[1].0 + i * (v[1].0 - v[0].0);
                        let y = v[1].1 + i * (v[1].1 - v[0].1);
                        if x < 0 || y < 0 || x >= map_w || y >= map_h {
                            break;
                        } else {
                            antinode_locs.push((x, y));
                        }
                    }
                    antinode_locs
                })
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
