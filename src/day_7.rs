/* https://adventofcode.com/2024/day/7
 */

use aoc2024::read_lines;
use itertools::{Either, Itertools};

fn main() {
    let start = std::time::Instant::now();

    let equations = parse_puzzle_input();

    let p1_ops = vec![Op::Add, Op::Mul];
    let p2_ops = vec![Op::Add, Op::Mul, Op::Cat];

    let (p1_checked, p2_checked): (Vec<u64>, Vec<Option<u64>>) =
        equations.iter().partition_map(|(value, numbers)| {
            match is_solvable(&p1_ops, *value, numbers[0], &numbers[1..]) {
                None => Either::Right(is_solvable(&p2_ops, *value, numbers[0], &numbers[1..])),
                Some(v) => Either::Left(v),
            }
        });

    let p1_ans = p1_checked.into_iter().sum::<u64>();
    let p2_ans = p1_ans + p2_checked.into_iter().flatten().sum::<u64>();

    println!("P1: {p1_ans}");
    println!("P2: {p2_ans}");
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

enum Op {
    Add,
    Mul,
    Cat,
}

fn is_solvable(ops: &[Op], v: u64, acc: u64, ns: &[u64]) -> Option<u64> {
    if acc > v || ns.is_empty() {
        None
    } else {
        ops.iter()
            .flat_map(|op| match op {
                Op::Add => acc.checked_add(ns[0]),
                Op::Mul => acc.checked_mul(ns[0]),
                Op::Cat => {
                    let mut n = ns[0];
                    (0..=n.ilog10()).rev().try_fold(acc, |acc, i| {
                        let p = 10u64.pow(i);
                        let d = n / p;
                        acc.checked_mul(10).and_then(|mut new_acc| {
                            new_acc += d;
                            n -= d * p;
                            Some(new_acc)
                        })
                    })
                }
            })
            .any(|x| match &ns[1..] {
                [] => x == v,
                [rest @ ..] => is_solvable(ops, v, x, rest).is_some(),
            })
            .then_some(v)
    }
}

fn parse_puzzle_input() -> Vec<(u64, Vec<u64>)> {
    read_lines("input/day_7.txt")
        .unwrap()
        .flatten()
        .map(|line| {
            let (value, numbers) = line.split(": ").collect_tuple().unwrap();
            (
                value.parse().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec()
}
