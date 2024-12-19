use crate::util::io::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::io;

pub enum Instruction {
    Do,
    DoNot,
    Mul(i32, i32),
}

type Input = Vec<Instruction>;

pub fn parse(filepath: &str) -> io::Result<Vec<Instruction>> {
    let re = Regex::new(r"(do)\(\)|(don't)\(\)|(mul)\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut instructions = Vec::<Instruction>::new();

    for line in read_lines(filepath)?.flatten() {
        for c in re.captures_iter(&line) {
            // This regex returns 5 groups, and each instruction is non-overlapping, so
            // we can filter out the None groups, and be left with a tag group, followed by
            // the arguments for that tag if they exist
            let cap_str = c.iter().filter_map(|m| m).map(|m| m.as_str()).skip(1).collect_vec();

            instructions.push(match cap_str[0] {
                "do" => Instruction::Do,
                "don't" => Instruction::DoNot,
                "mul" => Instruction::Mul(cap_str[1].parse().unwrap(), cap_str[2].parse().unwrap()),
                _ => unreachable!(),
            });
        }
    }

    Ok(instructions)
}

pub fn part1(input: &Input) -> Option<i32> {
    input
        .iter()
        .fold(0, |acc, i| match i {
            Instruction::Mul(n, m) => acc + n * m,
            _ => acc,
        })
        .into()
}

pub fn part2(input: &Input) -> Option<i32> {
    input
        .iter()
        .fold((0, true), |(acc, mul_enabled), i| match i {
            Instruction::Do => (acc, true),
            Instruction::DoNot => (acc, false),
            Instruction::Mul(n, m) => {
                if mul_enabled {
                    (acc + n * m, mul_enabled)
                } else {
                    (acc, mul_enabled)
                }
            }
        })
        .0
        .into()
}
