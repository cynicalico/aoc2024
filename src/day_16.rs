use aoc2024::read_lines;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

/** https://adventofcode.com/2024/day/16 */
fn main() {
    let start_time = std::time::Instant::now();

    let (maze, start, end) = parse_puzzle_input();

    let paths = a_star(&maze, start, end);

    println!("P1: {}", score_path(&paths[0]));
    println!(
        "\nTook {:.04}s",
        start_time.elapsed().as_nanos() as f64 / 1e9
    );
}

type Maze = Vec<Vec<bool>>;
type Pos = (usize, usize);
type Path = Vec<(Pos, Dir)>;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

fn left(p: &Pos, from: &Dir) -> Pos {
    match from {
        Dir::North => (p.0, p.1 - 1),
        Dir::East => (p.0 - 1, p.1),
        Dir::South => (p.0, p.1 + 1),
        Dir::West => (p.0 + 1, p.1),
    }
}

fn right(p: &Pos, from: &Dir) -> Pos {
    match from {
        Dir::North => (p.0, p.1 + 1),
        Dir::East => (p.0 + 1, p.1),
        Dir::South => (p.0, p.1 - 1),
        Dir::West => (p.0 - 1, p.1),
    }
}

fn forward(p: &Pos, from: &Dir) -> Pos {
    match from {
        Dir::North => (p.0 - 1, p.1),
        Dir::East => (p.0, p.1 + 1),
        Dir::South => (p.0 + 1, p.1),
        Dir::West => (p.0, p.1 - 1),
    }
}

fn dir(from: Pos, to: Pos) -> Dir {
    if from.0 < to.0 {
        Dir::South
    } else if from.0 > to.0 {
        Dir::North
    } else if from.1 < to.1 {
        Dir::East
    } else {
        Dir::West
    }
}

fn reconstruct_path(came_from: &HashMap<Pos, Pos>, start: Pos, mut current: Pos) -> Path {
    let mut path = Path::new();
    loop {
        let from = came_from[&current];
        path.push((current, dir(from, current)));

        if current == start {
            break;
        }
        current = from;
    }
    path.reverse();
    path
}

fn a_star(maze: &Maze, start: Pos, end: Pos) -> Vec<Path> {
    let mut paths = Vec::<Path>::new();

    let h = |p: &Pos| (p.0.abs_diff(end.0) + p.1.abs_diff(end.1)) as u32;

    let mut open_set = PriorityQueue::<Pos, Reverse<u32>>::new();
    let mut came_from = HashMap::<Pos, Pos>::new();
    let mut g_score = HashMap::<Pos, u32>::new();
    let mut f_score = HashMap::<Pos, u32>::new();

    came_from.insert(start, (start.0, start.1 - 1));
    g_score.insert(start, 0);
    f_score.insert(start, h(&start));
    open_set.push(start, Reverse(f_score[&start]));

    while !open_set.is_empty() {
        let (current, _) = open_set.peek().unwrap();
        if *current == end {
            paths.push(reconstruct_path(&came_from, start, *current));
        }

        let (current, _) = open_set.pop().unwrap();
        let current_dir = dir(came_from[&current], current);

        for (neighbor, d) in [
            (forward(&current, &current_dir), 1),
            (left(&current, &current_dir), 1001),
            (right(&current, &current_dir), 1001),
        ] {
            if maze[neighbor.0][neighbor.1] {
                continue;
            }

            let tentative_g_score = g_score[&current] + d;
            if tentative_g_score <= *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(&neighbor));
                open_set.push(neighbor, Reverse(f_score[&neighbor]));
            }
        }
    }

    paths
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

fn parse_puzzle_input() -> (Maze, Pos, Pos) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let maze: Maze = read_lines("input/day_16.txt")
        .expect("Failed to open input file")
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

    (maze, start, end)
}
