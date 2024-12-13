#![feature(iter_array_chunks)]

use aoc2024::read;
use regex::Regex;

/** https://adventofcode.com/2024/day/13 */
fn main() {
    let start = std::time::Instant::now();

    let (p1, p2): (Vec<_>, Vec<_>) = parse_puzzle_input()
        .into_iter()
        .map(|xs| (solve_system(&xs, 0), solve_system(&xs, 1e13 as i64)))
        .unzip();

    println!("P1: {}", p1.into_iter().flatten().sum::<i64>());
    println!("P2: {}", p2.into_iter().flatten().sum::<i64>());
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn solve_system(xs: &[i64], c: i64) -> Option<i64> {
    let x = ((xs[4] + c) * xs[3] - xs[2] * (xs[5] + c)) / (xs[0] * xs[3] - xs[2] * xs[1]);
    let y = (xs[0] * (xs[5] + c) - (xs[4] + c) * xs[1]) / (xs[0] * xs[3] - xs[2] * xs[1]);
    (xs[0] * x + xs[2] * y == xs[4] + c && xs[1] * x + xs[3] * y == xs[5] + c).then_some(x * 3 + y)
}

fn parse_puzzle_input() -> Vec<[i64; 6]> {
    Regex::new(r"(\d+)")
        .unwrap()
        .find_iter(
            read("input/day_13.txt")
                .expect("Failed to open input file")
                .as_str(),
        )
        .map(|m| m.as_str().parse().unwrap())
        .array_chunks::<6>()
        .collect()
}
