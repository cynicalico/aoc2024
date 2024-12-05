#![feature(let_chains)]

/* https://adventofcode.com/2024/day/5
 */
use aoc2024::read_lines_partitioned;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn main() {
    let start = std::time::Instant::now();

    let (ordering, updates) = parse_puzzle_input();

    let (valid_updates, invalid_updates): (Vec<_>, Vec<_>) = updates.into_iter().partition(|u| {
        u.is_sorted_by(|a, b| {
            ordering
                .get(b)
                .and_then(|o| Some(!o.contains(a)))
                .unwrap_or(true)
        })
    });

    println!("P1: {}", calculate_p1_ans(&valid_updates));
    println!("P2: {}", calculate_p2_ans(&ordering, &invalid_updates));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn calculate_p1_ans(valid_updates: &[Vec<u32>]) -> u32 {
    valid_updates.iter().map(|u| u[u.len() / 2]).sum()
}

fn calculate_p2_ans(ordering: &HashMap<u32, HashSet<u32>>, invalid_updates: &[Vec<u32>]) -> u32 {
    invalid_updates
        .iter()
        .map(|u| {
            let mut u = u.to_owned();
            u.sort_by(|a, b| {
                ordering
                    .get(b)
                    .and_then(|o| {
                        Some(if o.contains(a) {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        })
                    })
                    .unwrap_or(Ordering::Equal)
            });
            u
        })
        .map(|u| u[u.len() / 2])
        .sum()
}

fn parse_puzzle_input() -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut ordering = HashMap::<u32, HashSet<u32>>::new();
    let mut updates = Vec::new();

    read_lines_partitioned(
        "input/day_5.txt",
        |line| {
            if let Some((before, after)) = line
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
        },
        |line| {
            updates.push(
                line.split(",")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect_vec(),
            );
        },
    )
    .expect("Failed to read input file");

    (ordering, updates)
}
