/* https://adventofcode.com/2024/day/7
 */

use aoc2024::read_lines;
use itertools::{Either, Itertools};

fn main() {
    let start = std::time::Instant::now();

    let equations = parse_puzzle_input();

    let p1_ops = vec![Op::Add, Op::Mul];
    let p2_ops = vec![Op::Cat, Op::Add, Op::Mul];

    let (p1_checked, p2_checked): (Vec<u64>, Vec<Option<u64>>) =
        equations.iter().partition_map(|(value, numbers)| {
            match is_solvable(&p1_ops, *value, numbers) {
                Some(v) => Either::Left(v),
                None => Either::Right(is_solvable(&p2_ops, *value, numbers)),
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

fn is_solvable(ops: &[Op], value: u64, numbers: &[u64]) -> Option<u64> {
    fn inner(ops: &[Op], value: u64, numbers: &[u64], acc: u64) -> Option<u64> {
        if acc > value {
            None
        } else {
            ops.iter()
                .flat_map(|op| match op {
                    Op::Add => acc.checked_add(numbers[0]),
                    Op::Mul => acc.checked_mul(numbers[0]),
                    Op::Cat => acc
                        .checked_mul(10u64.pow(numbers[0].ilog10() + 1))
                        .map(|x| x + numbers[0]),
                })
                .any(|x| match &numbers[1..] {
                    [] => x == value,
                    [rest @ ..] => inner(ops, value, rest, x).is_some(),
                })
                .then_some(value)
        }
    }

    inner(ops, value, &numbers[1..], numbers[0])
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
