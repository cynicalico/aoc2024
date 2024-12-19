use crate::util::io::read_lines;
use itertools::Itertools;
use std::{collections::HashMap, io, iter::once};

type Input = (i32, i32, HashMap<char, Vec<(i32, i32)>>);

pub fn parse(filepath: &str) -> io::Result<Input> {
    let mut map_w: i32 = 0;
    let mut map_h: i32 = 0;
    let mut locs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, line) in read_lines(filepath)?.flatten().enumerate() {
        map_h = map_h.max(y as i32 + 1);
        map_w = map_w.max(line.len() as i32);

        for (x, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
            let pos = (x as i32, y as i32);
            if let Some(v) = locs.get_mut(&c) {
                v.push(pos);
            } else {
                locs.insert(c, vec![pos]);
            }
        }
    }

    Ok((map_w, map_h, locs))
}

pub fn part1(input: &Input) -> Option<usize> {
    calculate_ans(input.0, input.1, &input.2, once(1i32)).into()
}

pub fn part2(input: &Input) -> Option<usize> {
    calculate_ans(input.0, input.1, &input.2, 0i32..).into()
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
        .map(|(_, vs)| vs.iter().combinations(2).map(|v| find_antinodes(*v[0], *v[1])).flatten())
        .flatten()
        .unique()
        .count()
}
