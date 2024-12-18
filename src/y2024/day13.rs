use crate::util::io::read;
use crate::util::parse::ParseOps;
use std::io;

type Input = (Vec<Option<i64>>, Vec<Option<i64>>);

pub fn parse(filepath: &str) -> io::Result<Input> {
    let (p1, p2): (Vec<_>, Vec<_>) = read(filepath)?
        .as_str()
        .iter_signed::<i64>()
        .array_chunks::<6>()
        .map(|xs| (solve_system(&xs, 0), solve_system(&xs, 1e13 as i64)))
        .unzip();
    Ok((p1, p2))
}

pub fn part1(input: &Input) -> Option<i64> {
    Some(input.0.iter().flatten().sum())
}

pub fn part2(input: &Input) -> Option<i64> {
    Some(input.1.iter().flatten().sum())
}

fn solve_system(xs: &[i64], c: i64) -> Option<i64> {
    let d = xs[0] * xs[3] - xs[2] * xs[1];
    let x = ((xs[4] + c) * xs[3] - xs[2] * (xs[5] + c)) / d;
    let y = (xs[0] * (xs[5] + c) - (xs[4] + c) * xs[1]) / d;
    (xs[0] * x + xs[2] * y == xs[4] + c && xs[1] * x + xs[3] * y == xs[5] + c).then_some(x * 3 + y)
}
