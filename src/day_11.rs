#![feature(let_chains)]

/* https://adventofcode.com/2024/day/11
 */

use aoc2024::read_single_line;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let start = std::time::Instant::now();

    let mut stone_counts = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(&mut stone_counts));
    println!("P2: {}", calculate_p2_ans(&mut stone_counts));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn calculate_p1_ans(stone_counts: &mut HashMap<u64, usize>) -> usize {
    for _ in 0..25 {
        blink(stone_counts);
    }
    stone_counts.values().sum()
}

fn calculate_p2_ans(stone_counts: &mut HashMap<u64, usize>) -> usize {
    for _ in 0..50 {
        blink(stone_counts);
    }
    stone_counts.values().sum()
}

fn blink(stone_counts: &mut HashMap<u64, usize>) {
    let mut new_stone_counts: HashMap<u64, usize> = HashMap::new();

    for (stone_v, stone_count) in stone_counts.iter() {
        let mut insert_or_add = |v: u64| match new_stone_counts.get_mut(&v) {
            None => {
                new_stone_counts.insert(v, *stone_count);
            }
            Some(count) => {
                *count += *stone_count;
            }
        };

        if *stone_v == 0 {
            insert_or_add(1);
        } else if let num_digits = stone_v.ilog10() + 1
            && (num_digits % 2 == 0)
        {
            let (half1, half2) = (
                stone_v / (10u64.pow(num_digits / 2)),
                stone_v % (10u64.pow(num_digits / 2)),
            );
            insert_or_add(half1);
            insert_or_add(half2);
        } else {
            insert_or_add(stone_v * 2024);
        }
    }

    drop(std::mem::replace(stone_counts, new_stone_counts));
}

fn parse_puzzle_input() -> HashMap<u64, usize> {
    read_single_line("input/day_11.txt")
        .expect("Failed to open input file")
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .counts()
}
