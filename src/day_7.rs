/* https://adventofcode.com/2024/day/7
 */

use aoc2024::read_lines;
use itertools::{Either, Itertools};

fn main() {
    let start = std::time::Instant::now();

    let equations = parse_puzzle_input();

    let (p1_checked, p2_checked): (Vec<Option<u64>>, Vec<Option<u64>>) =
        equations.iter().partition_map(|e| {
            match is_solvable_p1(e.value, e.numbers[0], &e.numbers[1..]) {
                None => Either::Right(is_solvable_p2(e.value, e.numbers[0], &e.numbers[1..])),
                s => Either::Left(s),
            }
        });

    let p1_ans = p1_checked.iter().flatten().sum::<u64>();
    let p2_ans = p1_ans + p2_checked.iter().flatten().sum::<u64>();

    println!("P1: {p1_ans}");
    println!("P2: {p2_ans}");
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

struct Equation {
    value: u64,
    numbers: Vec<u64>,
}

fn is_solvable_p1(v: u64, acc: u64, ns: &[u64]) -> Option<u64> {
    if acc > v {
        None
    } else {
        match &ns {
            [] => None,
            [n] => (acc + n == v || acc * n == v).then_some(v),
            [n, rest @ ..] => {
                is_solvable_p1(v, acc + n, rest).or_else(|| is_solvable_p1(v, acc * n, rest))
            }
        }
    }
}

fn concatenate(mut n: u64, mut m: u64) -> Option<u64> {
    for i in (0..=m.ilog10()).rev() {
        let p = 10u64.pow(i);
        let d = m / p;
        match n.checked_mul(10) {
            None => return None,
            Some(v) => {
                n = v;
            }
        }
        n += d;
        m -= d * p;
    }
    Some(n)
}

fn is_solvable_p2(value: u64, acc: u64, numbers: &[u64]) -> Option<u64> {
    if acc > value {
        None
    } else {
        match &numbers {
            [] => None,
            [n] => {
                (acc + n == value || acc * n == value || concatenate(acc, *n).unwrap_or(0) == value)
                    .then_some(value)
            }
            [n, rest @ ..] => is_solvable_p2(value, acc + n, rest)
                .or_else(|| is_solvable_p2(value, acc * n, rest))
                .or_else(|| is_solvable_p2(value, concatenate(acc, *n).unwrap_or(0), rest)),
        }
    }
}

fn parse_puzzle_input() -> Vec<Equation> {
    read_lines("input/day_7.txt")
        .unwrap()
        .flatten()
        .map(|line| {
            let (value, numbers) = line.split(": ").collect_tuple().unwrap();
            Equation {
                value: value.parse().unwrap(),
                numbers: numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_vec(),
            }
        })
        .collect_vec()
}
