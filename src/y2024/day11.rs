use crate::util::io::read_single_line;
use crate::util::parse::ParseOps;
use itertools::Itertools;
use std::collections::HashMap;
use std::io;

type Input = (usize, usize);

pub fn parse(filename: &str) -> io::Result<Input> {
    let mut stone_counts = read_single_line(filename)?
        .as_str()
        .iter_unsigned::<u64>()
        .counts();

    for _ in 0..25 {
        blink(&mut stone_counts);
    }
    let p1_ans = stone_counts.values().sum();

    for _ in 0..50 {
        blink(&mut stone_counts);
    }

    Ok((p1_ans, stone_counts.values().sum()))
}

pub fn part1(input: &Input) -> Option<usize> {
    Some(input.0)
}

pub fn part2(input: &Input) -> Option<usize> {
    Some(input.1)
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
