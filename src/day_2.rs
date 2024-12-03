use aoc2024::read_lines;
use itertools::Itertools;

fn main() {
    let reports = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(&reports))
}

fn calculate_p1_ans(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| report.windows(2).map(|w| w[0].cmp(&w[1])).all_equal())
        .filter(|report| {
            report
                .windows(2)
                .all(|w| (1..4).contains(&w[0].abs_diff(w[1])))
        })
        .count()
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
