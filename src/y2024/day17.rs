use crate::util::io::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::io;

#[derive(Debug)]
pub struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug)]
pub enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

type Program = Vec<(Opcode, u64)>;

type Input = (Registers, Program, Vec<u64>);

pub fn parse(filepath: &str) -> io::Result<Input> {
    let mut regs = Registers::new(0);
    let mut prog = Program::new();
    let mut quine = Vec::new();

    let re = Regex::new(r"Register ([ABC]): (\d+)|(Program): ((?:\d+,\d+,?)+)$").unwrap();
    let _ = read_lines(filepath)?.flatten().filter(|line| !line.is_empty()).for_each(|line| {
        let (_, [k, v]) = re.captures(&line).map(|c| c.extract()).unwrap();
        match k {
            "A" => regs.a = v.parse().unwrap(),
            "B" => regs.b = v.parse().unwrap(),
            "C" => regs.c = v.parse().unwrap(),
            "Program" => {
                quine = v.split(',').map(|n| n.parse::<u64>().unwrap()).collect();
                for chunk in quine.chunks_exact(2) {
                    let [opcode, operand] = chunk else { unreachable!() };
                    prog.push(((*opcode).into(), *operand));
                }
            }
            _ => unreachable!(),
        }
    });

    Ok((regs, prog, quine))
}

pub fn part1(input: &Input) -> Option<String> {
    execute(&mut Registers::new(input.0.a), &input.1).iter().join(",").into()
}

pub fn part2(input: &Input) -> Option<u64> { find_a_quine(&input.1, &input.2).into() }

impl Registers {
    pub fn new(a: u64) -> Self { Self { a, b: 0, c: 0 } }
}

impl From<u64> for Opcode {
    fn from(value: u64) -> Self {
        match value {
            0 => Opcode::ADV,
            1 => Opcode::BXL,
            2 => Opcode::BST,
            3 => Opcode::JNZ,
            4 => Opcode::BXC,
            5 => Opcode::OUT,
            6 => Opcode::BDV,
            7 => Opcode::CDV,
            _ => panic!("Out of range"),
        }
    }
}

fn execute(regs: &mut Registers, prog: &Program) -> Vec<u64> {
    let mut output = Vec::new();
    let mut ip = 0;

    while ip < prog.len() {
        let (opcode, operand) = &prog[ip];
        match opcode {
            Opcode::ADV => regs.a >>= combo(regs, *operand),
            Opcode::BXL => regs.b ^= operand,
            Opcode::BST => regs.b = combo(regs, *operand) % 8,
            Opcode::JNZ => {
                if regs.a != 0 {
                    ip = (operand / 2) as usize;
                    continue;
                }
            }
            Opcode::BXC => regs.b ^= regs.c,
            Opcode::OUT => output.push(combo(regs, *operand) % 8),
            Opcode::BDV => regs.b = regs.a >> combo(regs, *operand),
            Opcode::CDV => regs.c = regs.a >> combo(regs, *operand),
        }
        ip += 1;
    }

    output
}

fn combo(regs: &Registers, operand: u64) -> u64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => panic!("Invalid combo operand: {operand}"),
    }
}

fn find_a_quine(prog: &Program, quine: &Vec<u64>) -> u64 {
    let mut starts = Vec::<u64>::from([0]);
    loop {
        let mut new_starts = Vec::<u64>::new();
        for a in starts {
            for a_test in a..a + 8 {
                let output_v = execute(&mut Registers::new(a_test), prog);
                if output_v == quine[quine.len() - output_v.len()..] {
                    if output_v.len() == quine.len() {
                        return a_test;
                    } else {
                        new_starts.push(a_test);
                    }
                }
            }
        }
        starts = new_starts.into_iter().map(|a| a << 3).collect();
    }
}
