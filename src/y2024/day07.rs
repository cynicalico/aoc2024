use crate::util::io::read_lines;
use crate::util::parse::ParseOps;
use itertools::{Either, Itertools};
use std::io;

type Input = (Vec<u64>, Vec<Option<u64>>);

pub fn parse(filepath: &str) -> io::Result<Input> {
    let equations = read_lines(filepath)?
        .flatten()
        .map(|line| {
            let ns = line.as_str().iter_unsigned().collect_vec();
            (ns[0], ns[1..].to_vec())
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    let p1_ops = vec![Op::Add, Op::Mul];
    let p2_ops = vec![Op::Cat, Op::Add, Op::Mul];

    let input: (Vec<u64>, Vec<Option<u64>>) = equations.iter().partition_map(|(value, numbers)| {
        match is_solvable(&p1_ops, *value, numbers) {
            Some(v) => Either::Left(v),
            None => Either::Right(is_solvable(&p2_ops, *value, numbers)),
        }
    });

    Ok(input)
}

pub fn part1(input: &Input) -> Option<u64> {
    Some(input.0.iter().sum::<u64>())
}

pub fn part2(input: &Input) -> Option<u64> {
    Some(input.0.iter().sum::<u64>() + input.1.iter().flatten().sum::<u64>())
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
                        .and_then(|x| x.checked_add(numbers[0])),
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
