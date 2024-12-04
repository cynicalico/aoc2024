/* https://adventofcode.com/2024/day/3
 */

use aoc2024::read_lines;
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
    instructions
        .iter()
        .map(|i| match i {
            Instruction::Do => 0,
            Instruction::DoNot => 0,
            Instruction::Mul(n, m) => n * m,
        })
        .sum()
}

fn calculate_p2_ans(instructions: &[Instruction]) -> i32 {
    let mut sum = 0;
    let mut mult_enabled = true;

    for i in instructions {
        match i {
            Instruction::Do => mult_enabled = true,
            Instruction::DoNot => mult_enabled = false,
            Instruction::Mul(n, m) => {
                if mult_enabled {
                    sum += n * m;
                }
            }
        }
    }

    sum
}

fn parse_puzzle_input() -> Vec<Instruction> {
    let re = Regex::new(r"(do|don't|mul)\((?:(\d{1,3}),(\d{1,3}))?\)").unwrap();

    let mut instructions = Vec::<Instruction>::new();

    for line in read_lines("input/day_3.txt").unwrap().flatten() {
        for c in re.captures_iter(&line) {
            instructions.push(match c.get(1).unwrap().as_str() {
                "do" => Instruction::Do,
                "don't" => Instruction::DoNot,
                "mul" => Instruction::Mul(
                    c.get(2).unwrap().as_str().parse().unwrap(),
                    c.get(3).unwrap().as_str().parse().unwrap(),
                ),
                _ => unreachable!(),
            })
        }
    }

    instructions
}
