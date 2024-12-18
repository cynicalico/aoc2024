use crate::util::io::read_lines_partitioned;
use crate::util::parse::ParseOps;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io;

type Input = (HashMap<u32, HashSet<u32>>, (Vec<Vec<u32>>, Vec<Vec<u32>>));

pub fn parse(filepath: &str) -> io::Result<Input> {
    let mut ordering = HashMap::<u32, HashSet<u32>>::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    read_lines_partitioned(
        filepath,
        |line| {
            let (before, after) = line.as_str().iter_unsigned().collect_tuple().unwrap();
            if let Some(hs) = ordering.get_mut(&before) {
                hs.insert(after);
            } else {
                ordering.insert(before, HashSet::from([after]));
            }
        },
        |line| {
            updates.push(line.as_str().iter_unsigned().collect());
        },
    )?;

    let updates: (Vec<_>, Vec<_>) = updates.into_iter().partition(|u| {
        u.is_sorted_by(|a, b| {
            ordering
                .get(b)
                .and_then(|o| Some(!o.contains(a)))
                .unwrap_or(true)
        })
    });

    Ok((ordering, updates))
}

pub fn part1(input: &Input) -> Option<u32> {
    Some(input.1 .0.iter().map(|u| u[u.len() / 2]).sum())
}

pub fn part2(input: &Input) -> Option<u32> {
    let ans = input
        .1
         .1
        .iter()
        .map(|u| {
            let mut u = u.to_owned();
            u.sort_by(|a, b| {
                input
                    .0
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
        .sum();
    Some(ans)
}
