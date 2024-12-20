//! Race Condition

use crate::util::io::read_lines;
use std::io;

type Pos = (usize, usize);
type Racetrack = Vec<Vec<u32>>;
type Input = (Racetrack, u32);

pub fn parse(filepath: &str) -> io::Result<Input> {
    let mut track_len = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut racetrack = read_lines(filepath)?
        .flatten()
        .enumerate()
        .map(|(y, l)| {
            l.char_indices()
                .map(|(x, c)| match c {
                    'S' => {
                        track_len += 1;
                        start = (y, x);
                        0
                    }
                    'E' => {
                        track_len += 1;
                        end = (y, x);
                        0
                    }
                    '.' => {
                        track_len += 1;
                        0
                    }
                    '#' => u32::MAX,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    mark_dist_from_end(&mut racetrack, track_len, &start, &end);

    Ok((racetrack, track_len))
}

fn mark_dist_from_end(racetrack: &mut Racetrack, track_len: u32, start: &Pos, end: &Pos) {
    let mut curr = *start;
    let mut dist_from_end = track_len - 1;
    loop {
        racetrack[curr.0][curr.1] = dist_from_end;
        dist_from_end -= 1;

        for o in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = ((curr.0 as i32 + o.0) as usize, (curr.1 as i32 + o.1) as usize);
            if racetrack[next.0][next.1] == 0 {
                curr = next;
                break;
            }
        }

        if curr == *end {
            break;
        }
    }
}

pub fn part1(input: &Input) -> Option<usize> {
    let (racetrack, track_len) = &input;
    let cheats = find_cheats(racetrack, *track_len, 2);
    cheats.into_iter().filter(|s| *s >= 100).count().into()
}

pub fn part2(input: &Input) -> Option<usize> {
    let (racetrack, track_len) = &input;
    let cheats = find_cheats(racetrack, *track_len, 20);
    cheats.into_iter().filter(|s| *s >= 100).count().into()
}

fn find_cheats(racetrack: &Racetrack, track_len: u32, taxicab_d: i32) -> Vec<u32> {
    let mut cheats = Vec::new();
    let mut make_cheat = |pa: Pos, pb: Pos, a: i32, b: i32| {
        let cost = (pa.0.abs_diff(pb.0) + pa.1.abs_diff(pb.1)) as i32 - 1;
        let savings = track_len as i32 - ((track_len as i32 - a) + b + cost) - 1;
        if savings > 0 {
            cheats.push(savings as u32);
        }
    };

    for y in 1..racetrack.len() - 1 {
        for x in 1..racetrack[y].len() - 1 {
            if racetrack[y][x] == u32::MAX {
                continue;
            }

            let p = (y, x);
            for c in taxicab_from_pos(racetrack, &p, taxicab_d) {
                if racetrack[p.0][p.1] > racetrack[c.0][c.1] {
                    make_cheat(p, c, racetrack[p.0][p.1] as i32, racetrack[c.0][c.1] as i32);
                }
            }
        }
    }

    cheats
}

fn taxicab_from_pos(racetrack: &Racetrack, p: &Pos, d: i32) -> Vec<Pos> {
    let mut offsets = Vec::new();
    for o in 1..d + 1 {
        offsets.push((o, 0));
        offsets.push((-o, 0));
        offsets.push((0, o));
        offsets.push((0, -o));
        for i in 1..o {
            offsets.push((-i, -o + i));
            offsets.push((i, -o + i));
            offsets.push((-o + i, i));
            offsets.push((o - i, i));
        }
    }

    offsets
        .into_iter()
        .flat_map(|(dy, dx)| {
            let trans = (p.0 as i32 + dy, p.1 as i32 + dx);
            (trans.0 >= 1
                && trans.1 >= 1
                && trans.0 <= racetrack.len() as i32 - 2
                && trans.1 <= racetrack[0].len() as i32 - 2)
                .then_some((trans.0 as usize, trans.1 as usize))
        })
        .collect()
}
