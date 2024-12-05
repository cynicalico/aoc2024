#![feature(let_chains)]

/* https://adventofcode.com/2024/day/5
 */

use aoc2024::read_lines_partitioned;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let start = std::time::Instant::now();

    let (ordering, updates) = parse_puzzle_input();

    let (valid_updates, invalid_updates): (Vec<_>, Vec<_>) = updates.into_iter().partition(|u| {
        for i in 0..u.len() {
            for j in i + 1..u.len() {
                if let Some(o) = ordering.get(&u[j])
                    && o.contains(&u[i])
                {
                    return false;
                }
            }
        }
        true
    });

    println!("P1: {}", calculate_p1_ans(&valid_updates));
    println!("P2: {}", calculate_p2_ans(&ordering, &invalid_updates));
    println!("Took {}ms", start.elapsed().as_millis());
}

fn calculate_p1_ans(valid_updates: &[Vec<u32>]) -> u32 {
    valid_updates.iter().map(|u| u[u.len() / 2]).sum()
}

fn calculate_p2_ans(ordering: &HashMap<u32, HashSet<u32>>, invalid_updates: &[Vec<u32>]) -> u32 {
    invalid_updates
        .iter()
        .map(|u| {
            let mut u = u.to_owned();
            for i in 0..u.len() {
                for j in i + 1..u.len() {
                    if let Some(o) = ordering.get(&u[j])
                        && o.contains(&u[i])
                    {
                        u.swap(i, j);
                    }
                }
            }
            u
        })
        .map(|u| u[u.len() / 2])
        .sum()
}

fn parse_puzzle_input() -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut ordering: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut updates = Vec::new();

    let (ordering_rules, page_numbers) = read_lines_partitioned("input/day_5.txt").unwrap();

    for rule in ordering_rules {
        if let Some((before, after)) = rule
            .split("|")
            .map(|n| n.parse::<u32>().unwrap())
            .collect_tuple()
        {
            if let Some(hs) = ordering.get_mut(&before) {
                hs.insert(after);
            } else {
                ordering.insert(before, HashSet::from([after]));
            }
        }
    }

    for numbers in page_numbers {
        updates.push(
            numbers
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect_vec(),
        );
    }

    (ordering, updates)
}
