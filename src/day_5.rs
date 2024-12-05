/* https://adventofcode.com/2024/day/5
 */

use aoc2024::read_lines;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let start = std::time::Instant::now();

    let (ordering_rules, updates) = parse_puzzle_input();
    let ordering = build_ordering_hashmap(&ordering_rules);
    let (valid_updates, invalid_updates): (Vec<_>, Vec<_>) = updates.into_iter().partition(|u| {
        for i in 0..u.len() {
            for j in i..u.len() {
                match ordering.get(&u[j]) {
                    None => continue,
                    Some(j_hs) => {
                        if j_hs.contains(&u[i]) {
                            return false;
                        }
                    }
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
                for j in i..u.len() {
                    match ordering.get(&u[j]) {
                        None => continue,
                        Some(j_hs) => {
                            if j_hs.contains(&u[i]) {
                                u.swap(i, j);
                            }
                        }
                    }
                }
            }
            u
        })
        .map(|u| u[u.len() / 2])
        .sum()
}

fn build_ordering_hashmap(ordering_rules: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    let mut ordering: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (before, after) in ordering_rules {
        match ordering.get_mut(before) {
            None => {
                ordering.insert(*before, HashSet::from([*after]));
            }
            Some(hs) => {
                hs.insert(*after);
            }
        }
    }

    ordering
}

fn parse_puzzle_input() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut ordering_rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let mut reading_ordering_rules = true;
    for line in read_lines("input/day_5.txt").unwrap().flatten() {
        if line.is_empty() {
            reading_ordering_rules = false;
            continue;
        }

        if reading_ordering_rules {
            let Some((n, m)) = line
                .split("|")
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
            else {
                unreachable!()
            };
            ordering_rules.push((n, m));
        } else {
            updates.push(
                line.split(",")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect_vec(),
            );
        }
    }

    (ordering_rules, updates)
}
