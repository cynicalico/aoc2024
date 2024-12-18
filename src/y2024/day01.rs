use crate::util::io::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::io;

type Input = (Vec<i32>, Vec<i32>);

pub fn parse(filepath: &str) -> io::Result<Input> {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    let (mut l1, mut l2): (Vec<_>, Vec<_>) = read_lines(filepath)?
        .flatten()
        .map(|line| {
            let (_, [n, m]) = re.captures(line.as_str()).unwrap().extract();
            (n.parse::<i32>().unwrap(), m.parse::<i32>().unwrap())
        })
        .unzip();

    l1.sort();
    l2.sort();

    Ok((l1, l2))
}

pub fn part1(input: &Input) -> Option<u32> {
    let ans = input
        .0
        .iter()
        .zip(&input.1)
        .map(|(n, m)| n.abs_diff(*m))
        .sum();
    Some(ans)
}

pub fn part2(input: &Input) -> Option<i32> {
    let l2_counts = input.1.iter().counts();
    let ans = input
        .0
        .iter()
        .map(|n| n * *l2_counts.get(n).unwrap_or(&0) as i32)
        .sum();
    Some(ans)
}
