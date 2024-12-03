use aoc2024::read_lines;
use itertools::Itertools;

fn main() {
    let reports = parse_puzzle_input();

    let (safe_reports, unsafe_reports): (Vec<&[i32]>, Vec<&[i32]>) = reports
        .iter()
        .map(|v| v.as_slice())
        .partition(|report| is_safe(report));

    println!("P1: {}", safe_reports.len());
    println!(
        "P2: {}",
        safe_reports.len() + dampen_problems(&unsafe_reports)
    );
}

fn dampen_problems(unsafe_reports: &[&[i32]]) -> usize {
    let mut n = 0;
    for report in unsafe_reports {
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
    n
}

fn is_safe(report: &[i32]) -> bool {
    let is_monotonic = report.windows(2).map(|w| w[0].cmp(&w[1])).all_equal();

    let is_stablish = report
        .windows(2)
        .all(|w| (1..4).contains(&w[0].abs_diff(w[1])));

    is_monotonic && is_stablish
}

fn parse_puzzle_input() -> Vec<Vec<i32>> {
    read_lines("input/day_2.txt")
        .unwrap()
        .flatten()
        .map(|line| {
            let levels = line.split_whitespace();
            levels.map(|n| n.parse::<i32>().unwrap()).collect()
        })
        .collect()
}
