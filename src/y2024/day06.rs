use crate::util::io::read_lines;
use itertools::Itertools;
use std::io;

type Input = (usize, u32);

pub fn parse(filepath: &str) -> io::Result<Input> {
    let mut guard_start_pos = (0, 0);
    let mut lab = Vec::new();

    for line in read_lines(filepath)?.flatten() {
        let chars = line.chars();

        if let Some(guard_pos) = chars.clone().position(|c| c == '^') {
            guard_start_pos = (lab.len(), guard_pos);
        }

        lab.push(chars.map(|c| Cell { obstacle: c == '#', visited_deltas: vec![] }).collect_vec())
    }

    Ok(calculate_ans(&mut lab, guard_start_pos))
}

pub fn part1(input: &Input) -> Option<usize> { input.0.into() }

pub fn part2(input: &Input) -> Option<u32> { input.1.into() }

struct Cell {
    obstacle: bool,
    visited_deltas: Vec<(i32, i32)>,
}

type Map = Vec<Vec<Cell>>;

struct Guard {
    pos: (usize, usize),
    delta: (i32, i32),
}

fn calculate_ans(lab: &mut Map, guard_start_pos: (usize, usize)) -> (usize, u32) {
    let mut guard = Guard { pos: guard_start_pos, delta: (0, -1) };

    reset_lab(lab, &mut guard, guard_start_pos);
    while let Some(step_pos) = try_get_step_pos(lab, &guard) {
        do_step(lab, &mut guard, &step_pos);
    }

    let possible_obstacle_locations = lab
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, c)| (!c.visited_deltas.is_empty()).then_some((y, x)))
        })
        .flatten()
        .collect_vec();
    let p1_ans = possible_obstacle_locations.len();

    let mut p2_ans = 0;
    for loc in possible_obstacle_locations {
        if loc == guard_start_pos {
            continue;
        }
        lab[loc.0][loc.1].obstacle = true;

        reset_lab(lab, &mut guard, guard_start_pos);
        while let Some(step_pos) = try_get_step_pos(lab, &guard) {
            if do_step(lab, &mut guard, &step_pos) {
                p2_ans += 1;
                break;
            }
        }

        lab[loc.0][loc.1].obstacle = false;
    }

    (p1_ans, p2_ans)
}

fn reset_lab(lab: &mut Map, guard: &mut Guard, guard_start_pos: (usize, usize)) {
    guard.pos = guard_start_pos;
    guard.delta = (-1, 0);

    for row in lab.iter_mut() {
        for cell in row.iter_mut() {
            cell.visited_deltas.clear();
        }
    }

    lab[guard.pos.0][guard.pos.1].visited_deltas.push(guard.delta);
}

fn try_get_step_pos(lab: &Map, guard: &Guard) -> Option<(usize, usize)> {
    let step_pos = (guard.pos.0 as i32 + guard.delta.0, guard.pos.1 as i32 + guard.delta.1);
    (step_pos.0 >= 0
        && step_pos.0 < lab.len() as i32
        && step_pos.1 >= 0
        && step_pos.1 < lab[0].len() as i32)
        .then_some((step_pos.0 as usize, step_pos.1 as usize))
}

fn do_step(lab: &mut Map, guard: &mut Guard, step_pos: &(usize, usize)) -> bool {
    let mut loop_detected = false;

    if lab[step_pos.0][step_pos.1].obstacle {
        guard.delta = (guard.delta.1, -guard.delta.0);
    } else {
        guard.pos = *step_pos;
    }

    if lab[guard.pos.0][guard.pos.1].visited_deltas.contains(&guard.delta) {
        loop_detected = true;
    }

    lab[guard.pos.0][guard.pos.1].visited_deltas.push(guard.delta);

    loop_detected
}
