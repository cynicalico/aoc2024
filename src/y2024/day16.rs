use crate::util::io::read_lines;
use hashbrown::{hash_map::Entry, HashMap};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, io};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

type Maze = Vec<Vec<bool>>;
type Pos = (usize, usize);
type Path = Vec<(Pos, Dir)>;

type Input = Vec<Path>;

pub fn parse(filepath: &str) -> io::Result<Input> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let maze: Maze = read_lines(filepath)?
        .flatten()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (y, x);
                    } else if c == 'E' {
                        end = (y, x);
                    }
                    c == '#'
                })
                .collect()
        })
        .collect();

    Ok(a_star(&maze, (start, Dir::East), end).unwrap())
}

pub fn part1(input: &Input) -> Option<u32> { score_path(&input[0]).into() }

pub fn part2(input: &Input) -> Option<usize> { count_tiles(&input).into() }

fn left(p: &(Pos, Dir)) -> (Pos, Dir) {
    match p.1 {
        Dir::North => ((p.0 .0, p.0 .1 - 1), Dir::West),
        Dir::East => ((p.0 .0 - 1, p.0 .1), Dir::North),
        Dir::South => ((p.0 .0, p.0 .1 + 1), Dir::East),
        Dir::West => ((p.0 .0 + 1, p.0 .1), Dir::South),
    }
}

fn right(p: &(Pos, Dir)) -> (Pos, Dir) {
    match p.1 {
        Dir::North => ((p.0 .0, p.0 .1 + 1), Dir::East),
        Dir::East => ((p.0 .0 + 1, p.0 .1), Dir::South),
        Dir::South => ((p.0 .0, p.0 .1 - 1), Dir::West),
        Dir::West => ((p.0 .0 - 1, p.0 .1), Dir::North),
    }
}

fn forward(p: &(Pos, Dir)) -> (Pos, Dir) {
    match p.1 {
        Dir::North => ((p.0 .0 - 1, p.0 .1), p.1),
        Dir::East => ((p.0 .0, p.0 .1 + 1), p.1),
        Dir::South => ((p.0 .0 + 1, p.0 .1), p.1),
        Dir::West => ((p.0 .0, p.0 .1 - 1), p.1),
    }
}

fn reconstruct_path(
    came_from: &mut HashMap<(Pos, Dir), Vec<(Pos, Dir)>>,
    start: (Pos, Dir),
    mut curr: (Pos, Dir),
) -> (Path, bool) {
    let mut path = Path::new();
    let mut has_more_paths = false;
    loop {
        path.push(curr);
        if curr == start {
            break;
        }
        let from_v = came_from.get_mut(&curr).unwrap();
        let from = if from_v.len() > 1 {
            has_more_paths = true;
            from_v.pop().unwrap()
        } else {
            *from_v.last().unwrap()
        };
        curr = from;
    }
    path.reverse();
    (path, has_more_paths)
}

fn a_star(maze: &Maze, start: (Pos, Dir), end: Pos) -> Option<Vec<Path>> {
    let mut path_ends = Vec::<(Pos, Dir)>::new();

    let h = |p: &(Pos, Dir)| (p.0 .0.abs_diff(end.0) + p.0 .1.abs_diff(end.1)) as u32;

    let mut open_set = PriorityQueue::<(Pos, Dir), Reverse<u32>>::new();
    let mut came_from = HashMap::<(Pos, Dir), Vec<(Pos, Dir)>>::new();
    let mut g_score = HashMap::<(Pos, Dir), u32>::new();
    let mut f_score = HashMap::<(Pos, Dir), u32>::new();

    match came_from.entry(start) {
        Entry::Occupied(mut e) => e.get_mut().push(((start.0 .0, start.0 .1 - 1), Dir::East)),
        Entry::Vacant(e) => {
            e.insert(Vec::from([((start.0 .0, start.0 .1 - 1), Dir::East)]));
        }
    }
    g_score.insert(start, 0);
    f_score.insert(start, h(&start));
    open_set.push(start, Reverse(f_score[&start]));

    while !open_set.is_empty() {
        let (curr, _) = open_set.pop().unwrap();
        if curr.0 == end {
            path_ends.push(curr);
            continue;
        }

        for (adj, d) in [(forward(&curr), 1), (left(&curr), 1001), (right(&curr), 1001)] {
            if maze[adj.0 .0][adj.0 .1] {
                continue;
            }

            let tentative_g_score = g_score[&curr] + d;
            if tentative_g_score <= *g_score.get(&adj).unwrap_or(&u32::MAX) {
                match came_from.entry(adj) {
                    Entry::Occupied(mut e) => e.get_mut().push(curr),
                    Entry::Vacant(e) => {
                        e.insert(Vec::from([curr]));
                    }
                }
                g_score.insert(adj, tentative_g_score);
                f_score.insert(adj, tentative_g_score + h(&adj));
                open_set.push(adj, Reverse(f_score[&adj]));
            }
        }
    }

    if !path_ends.is_empty() {
        came_from.values_mut().for_each(|v| v.reverse());

        let mut candidates = Vec::<(Path, u32)>::new();
        for ends in path_ends {
            loop {
                let (path, has_more_paths) = reconstruct_path(&mut came_from, start, ends);
                let score = score_path(&path);
                candidates.push((path, score));
                if !has_more_paths {
                    break;
                }
            }
        }

        let min_score = candidates.iter().min_by(|(_, s1), (_, s2)| s1.cmp(&s2)).unwrap().1;
        candidates
            .into_iter()
            .filter_map(|(p, s)| (s == min_score).then_some(p))
            .collect::<Vec<_>>()
            .into()
    } else {
        None
    }
}

fn score_path(path: &Path) -> u32 {
    path.iter()
        .skip(1)
        .fold((0, path[0].1), |(cost, last_dir), (_, dir)| {
            let new_cost = cost + if last_dir != *dir { 1001 } else { 1 };
            (new_cost, *dir)
        })
        .0
}

fn count_tiles(paths: &[Path]) -> usize {
    paths.iter().flatten().map(|(pos, _)| pos).unique().count()
}
