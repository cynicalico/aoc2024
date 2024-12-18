use crate::util::io::read_lines;
use crate::util::parse::*;
use itertools::Itertools;
use std::io;

type Input = (Vec<Vec<i32>>, Vec<Vec<i32>>);

pub fn parse(filepath: &str) -> io::Result<Input> {
    let reports = read_lines(filepath)?
        .flatten()
        .map(|line| line.as_str().iter_signed().collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    Ok(reports.into_iter().partition(|report| is_safe(report)))
}

pub fn part1(input: &Input) -> Option<usize> {
    Some(input.0.len())
}

pub fn part2(input: &Input) -> Option<usize> {
    let mut n = 0;
    for report in &input.1 {
        for idx in 0..report.len() {
            let dampened: Vec<i32> = report
                .iter()
                .enumerate()
                .filter_map(|(i, level)| if i == idx { None } else { Some(*level) })
                .collect();

            if is_safe(&dampened) {
                n += 1;
                break;
            }
        }
    }

    Some(input.0.len() + n)
}

fn is_safe(report: &[i32]) -> bool {
    let is_monotonic = report.windows(2).map(|w| w[0].cmp(&w[1])).all_equal();

    let is_stablish = report
        .windows(2)
        .all(|w| (1..4).contains(&w[0].abs_diff(w[1])));

    is_monotonic && is_stablish
}
