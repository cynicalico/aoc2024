/* https://adventofcode.com/2024/day/3
 */

use aoc2024::read_lines;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let start = std::time::Instant::now();

    let instructions = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(&instructions));
    println!("P2: {}", calculate_p2_ans(&instructions));
    println!("Took {}ms", start.elapsed().as_millis());
}

enum Instruction {
    Do,
    DoNot,
    Mul(i32, i32),
}

fn calculate_p1_ans(instructions: &[Instruction]) -> i32 {
    instructions.iter().fold(0, |acc, i| match i {
        Instruction::Mul(n, m) => acc + n * m,
        _ => acc,
    })
}

fn calculate_p2_ans(instructions: &[Instruction]) -> i32 {
    instructions
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
}

fn parse_puzzle_input() -> Vec<Instruction> {
    let re = Regex::new(r"(do)\(\)|(don't)\(\)|(mul)\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut instructions = Vec::<Instruction>::new();

    for line in read_lines("input/day_3.txt").unwrap().flatten() {
        for c in re.captures_iter(&line) {
            // This regex returns 5 groups, and each instruction is non-overlapping, so
            // we can filter out the None groups, and be left with a tag group, followed by
            // the arguments for that tag if they exist
            let cap_str = c
                .iter()
                .filter_map(|m| m)
                .map(|m| m.as_str())
                .skip(1)
                .collect_vec();

            instructions.push(match cap_str[0] {
                "do" => Instruction::Do,
                "don't" => Instruction::DoNot,
                "mul" => Instruction::Mul(cap_str[1].parse().unwrap(), cap_str[2].parse().unwrap()),
                _ => unreachable!(),
            });
        }
    }

    instructions
}
