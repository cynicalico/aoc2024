/* https://adventofcode.com/2024/day/13
 */

use aoc2024::read_lines;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let start = std::time::Instant::now();

    let machines = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(&machines));
    println!("P2: {}", calculate_p2_ans(&machines));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn calculate_p1_ans(machines: &[((i64, i64), (i64, i64), (i64, i64))]) -> i64 {
    machines
        .into_iter()
        .flat_map(|(a, b, c)| solve_system(a, b, c))
        .sum()
}

fn calculate_p2_ans(machines: &[((i64, i64), (i64, i64), (i64, i64))]) -> i64 {
    machines
        .into_iter()
        .flat_map(|(a, b, c)| solve_system(a, b, &(c.0 + 10000000000000, c.1 + 10000000000000)))
        .sum()
}

fn solve_system(a: &(i64, i64), b: &(i64, i64), c: &(i64, i64)) -> Option<i64> {
    let x = (c.0 * b.1 - b.0 * c.1) as f64 / (a.0 * b.1 - b.0 * a.1) as f64;
    let y = (a.0 * c.1 - c.0 * a.1) as f64 / (a.0 * b.1 - b.0 * a.1) as f64;
    (x.fract() == 0.0 && y.fract() == 0.0).then_some((x * 3.0 + y) as i64)
}

fn parse_puzzle_input() -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    let button_re = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    read_lines("input/day_13.txt")
        .expect("Failed to open input file")
        .flatten()
        .filter(|line| !line.is_empty())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let (b1, b2, p) = chunk.collect_tuple().unwrap();
            let (_, [b1x, b1y]) = button_re
                .captures(b1.as_str())
                .map(|c| c.extract())
                .unwrap();
            let (_, [b2x, b2y]) = button_re
                .captures(b2.as_str())
                .map(|c| c.extract())
                .unwrap();
            let (_, [px, py]) = prize_re.captures(p.as_str()).map(|c| c.extract()).unwrap();
            (
                (b1x.parse().unwrap(), b1y.parse().unwrap()),
                (b2x.parse().unwrap(), b2y.parse().unwrap()),
                (px.parse().unwrap(), py.parse().unwrap()),
            )
        })
        .collect_vec()
}
