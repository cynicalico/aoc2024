use crate::util::io::read_lines;
use crate::util::parse::ParseOps;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
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
    for (x, y) in input.iter().take(1024) {
        maze[(y * W) + x] = true;
    }

    a_star(&maze).map(|p| p.len() - 1)
}

pub fn part2(input: &Input) -> Option<String> {
    let iter = input.iter();

    let mut maze: Maze = [false; W * H];
    for (x, y) in iter.clone().take(1024) {
        maze[(y * W) + x] = true;
    }

    let mut path = a_star(&maze).unwrap().into_iter().collect::<HashSet<Pos>>();

    let mut iter = iter.skip(1024);
    while let Some((x, y)) = iter.next() {
        maze[(y * W) + x] = true;
        if path.contains(&(*y as i32, *x as i32)) {
            if let Some(new_path) = a_star(&maze) {
                path = new_path.into_iter().collect::<HashSet<Pos>>();
                continue;
            }
            return Some(format!("{x},{y}"));
        }
    }
    Some("No ans, goal always reachable".to_owned())
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

fn a_star(maze: &Maze) -> Option<Vec<Pos>> {
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
            let mut path = Vec::new();
            let mut curr = goal;
            loop {
                path.push(curr);
                if curr == start {
                    break;
                }
                curr = came_from[&curr];
            }
            return Some(path);
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
