use crate::util::io::read_lines;
use crate::util::parse::ParseOps;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::io;

type Input = Vec<(usize, usize)>;

pub fn parse(filepath: &str) -> io::Result<Input> {
    let bytes: Input = read_lines(filepath)?
        .flatten()
        .map(|line| line.as_str().iter_unsigned().collect_tuple().unwrap())
        .collect();
    Ok(bytes)
}

pub fn part1(input: &Input) -> Option<usize> {
    let mut maze: Maze = [false; W * H];
    for (x, y) in input.iter().take(12) {
        maze[(y * W) + x] = true;
    }

    a_star(&maze)
}

pub fn part2(input: &Input) -> Option<String> {
    let mut iter = input.iter();

    let mut maze: Maze = [false; W * H];
    for (x, y) in iter.clone().take(12) {
        maze[(y * W) + x] = true;
    }

    let mut iter = iter.skip(12);
    while let Some((x, y)) = iter.next() {
        maze[(y * W) + x] = true;
        if let Some(_) = bfs(&maze) {
            continue;
        }
        return Some(format!("{x},{y}"));
    }
    Some("Invalid input, all paths okay".to_owned())
}

const W: usize = 71;
const H: usize = 71;
type Maze = [bool; W * H];
type Pos = (i32, i32);

fn wall_at(maze: &Maze, p: &Pos) -> bool {
    if p.0 < 0 || p.1 < 0 || p.0 >= H as i32 || p.1 >= H as i32 {
        true // acts as a wall, good enough for maze solving
    } else {
        maze[(p.0 as usize * W) + p.1 as usize]
    }
}

fn a_star(maze: &Maze) -> Option<usize> {
    let start = (0, 0);
    let goal = (H as i32 - 1, W as i32 - 1);

    let h = |p: &Pos| p.0.abs_diff(goal.0) + p.1.abs_diff(goal.1);

    let mut open_set = PriorityQueue::<Pos, Reverse<u32>>::new();
    let mut came_from = HashMap::<Pos, Pos>::new();
    let mut g_score = HashMap::<Pos, u32>::new();
    let mut f_score = HashMap::<Pos, u32>::new();

    g_score.insert(start, 0);
    f_score.insert(start, h(&start));
    open_set.push(start, Reverse(f_score[&start]));

    while !open_set.is_empty() {
        let (curr, _) = open_set.peek().unwrap();
        if *curr == goal {
            let mut steps = 0;
            let mut curr = goal;
            loop {
                if curr == start {
                    break;
                }
                steps += 1;
                curr = came_from[&curr];
            }
            return Some(steps);
        }

        let (curr, _) = open_set.pop().unwrap();
        for o in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let adj = (curr.0 + o.0, curr.1 + o.1);
            if wall_at(maze, &adj) {
                continue;
            }

            let tentative_g_score = g_score[&curr] + 1;
            if tentative_g_score < *g_score.get(&adj).unwrap_or(&u32::MAX) {
                came_from.insert(adj, curr);
                g_score.insert(adj, tentative_g_score);
                f_score.insert(adj, tentative_g_score + h(&adj));
                open_set.push(adj, Reverse(f_score[&adj]));
            }
        }
    }

    None
}

fn bfs(maze: &Maze) -> Option<usize> {
    let start = (0, 0);
    let goal = (H as i32 - 1, W as i32 - 1);

    let mut queue = Vec::from([start]);
    let mut parent: [Option<Pos>; W * H] = [None; W * H];

    while !queue.is_empty() {
        let mut p = queue.pop().unwrap();
        if p == goal {
            let mut steps = 0;
            while let Some(pp) = parent[(p.0 as usize * W) + p.1 as usize] {
                steps += 1;
                p = pp;
            }
            return Some(steps);
        }

        for o in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let np = (p.0 + o.0, p.1 + o.1);
            if np == start || wall_at(&maze, &np) {
                continue;
            }

            if parent[(np.0 as usize * W) + np.1 as usize].is_none() {
                parent[(np.0 as usize * W) + np.1 as usize] = Some(p);
                queue.push(np);
            }
        }
    }

    None
}
