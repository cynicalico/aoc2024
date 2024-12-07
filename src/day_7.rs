use aoc2024::read_lines;
use itertools::{Either, Itertools};

fn main() {
    let start = std::time::Instant::now();

    let equations = parse_puzzle_input();

    let (p1_checked, p2_checked): (Vec<Option<i64>>, Vec<Option<i64>>) =
        equations.iter().partition_map(|equation| {
            match is_solvable_p1(&equation.value, equation.numbers[0], &equation.numbers[1..]) {
                None => Either::Right(is_solvable_p2(
                    &equation.value,
                    equation.numbers[0],
                    &equation.numbers[1..],
                )),
                s => Either::Left(s),
            }
        });

    let p1_ans = p1_checked.iter().flatten().sum::<i64>();
    let p2_ans = p1_ans + p2_checked.iter().flatten().sum::<i64>();

    println!("P1: {p1_ans}");
    println!("P2: {p2_ans}");
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

struct Equation {
    value: i64,
    numbers: Vec<i64>,
}

fn is_solvable_p1(value: &i64, acc: i64, numbers: &[i64]) -> Option<i64> {
    match &numbers {
        [] => None,
        [n] => (acc + n == *value || acc * n == *value).then_some(*value),
        [n, rest @ ..] => {
            is_solvable_p1(value, acc + n, rest).or_else(|| is_solvable_p1(value, acc * n, rest))
        }
    }
}

fn concatenate(n: i64, m: i64) -> i64 {
    (n.to_string() + m.to_string().as_str()).parse().unwrap()
}

fn is_solvable_p2(value: &i64, acc: i64, numbers: &[i64]) -> Option<i64> {
    match &numbers {
        [] => None,
        [n] => (acc + n == *value || acc * n == *value || concatenate(acc, *n) == *value)
            .then_some(*value),
        [n, rest @ ..] => is_solvable_p2(value, acc + n, rest)
            .or_else(|| is_solvable_p2(value, acc * n, rest))
            .or_else(|| is_solvable_p2(value, concatenate(acc, *n), rest)),
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
