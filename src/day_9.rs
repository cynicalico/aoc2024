/* https://adventofcode.com/2024/day/1
 */

use aoc2024::read_single_line;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let start = std::time::Instant::now();

    let disk_map = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(disk_map.clone()));
    println!("P2: {}", calculate_p2_ans(disk_map));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn calculate_p1_ans(mut disk_map: Vec<Option<u32>>) -> u64 {
    let mut l = 0usize;
    let mut r = disk_map.iter().rposition(|o| o.is_some()).unwrap();
    while l < r {
        if disk_map[l].is_none() {
            disk_map.swap(l, r);
            while disk_map[r].is_none() {
                r -= 1;
            }
        }
        l += 1;
    }

    checksum(&disk_map)
}

fn calculate_p2_ans(mut disk_map: Vec<Option<u32>>) -> u64 {
    fn find_next_file(disk_map: &[Option<u32>], start: Option<usize>) -> Option<(usize, usize)> {
        let mut r;
        if let Some(start) = start {
            r = start;
        } else {
            return None;
        }
        loop {
            match disk_map.get(r) {
                None => return None,
                Some(Some(_)) => break,
                Some(None) => {
                    r -= 1;
                }
            }
        }
        let mut l = r;
        loop {
            match disk_map.get(l) {
                None => break None,
                Some(Some(v)) => {
                    if *v != disk_map[r].unwrap() {
                        break Some((l + 1, r));
                    }
                    match l.checked_sub(1) {
                        None => break Some((l, r)),
                        Some(lm1) => {
                            l = lm1;
                        }
                    }
                }
                Some(None) => break None,
            }
        }
        .or(Some((l + 1, r)))
    }

    let mut free_spaces: HashMap<usize, BinaryHeap<Reverse<usize>>> = HashMap::new();
    let mut disk_map_search_start = 0usize;

    let mut last_moved_file_id = u32::MAX;
    let mut file = None;
    loop {
        file = find_next_file(
            &disk_map,
            file.map_or(Some(disk_map.len() - 1), |(l, _): (usize, _)| {
                l.checked_sub(1)
            }),
        );
        match file {
            None => break,
            Some(file) => {
                let id = disk_map[file.0].unwrap();
                if id > last_moved_file_id {
                    continue;
                }

                let filesize = file.1 - file.0;

                let mut valid_free_space = free_spaces
                    .iter()
                    .flat_map(|(size, heap)| heap.peek().map(|s| (*size, s.0)))
                    .filter(|(size, start)| *size >= filesize && *start < file.0)
                    .sorted_by(|a, b| a.1.cmp(&b.1))
                    .next();
                if valid_free_space.is_none() && disk_map_search_start < file.0 {
                    let (new_search_start, free_space_size) =
                        find_free_spaces(&mut free_spaces, &disk_map, file, disk_map_search_start);
                    disk_map_search_start = new_search_start;
                    if let Some(size) = free_space_size {
                        valid_free_space = free_spaces[&size].peek().map(|s| (size, s.0));
                    }
                }

                if let Some((free_space_size, free_space_start)) = valid_free_space {
                    for i in 0..=filesize {
                        disk_map[free_space_start + i] = disk_map[file.0 + i];
                        disk_map[file.0 + i] = None;
                    }
                    last_moved_file_id = id;

                    // Remove the one we chose
                    free_spaces.get_mut(&free_space_size).unwrap().pop();

                    if free_space_size > filesize {
                        let new_size = free_space_size - filesize - 1;
                        if let Some(h) = free_spaces.get_mut(&new_size) {
                            h.push(Reverse(free_space_start + filesize + 1));
                        } else {
                            free_spaces.insert(
                                new_size,
                                BinaryHeap::from([Reverse(free_space_start + filesize + 1)]),
                            );
                        }
                    }
                }
            }
        }
    }

    checksum(&disk_map)
}

fn find_free_spaces(
    free_spaces: &mut HashMap<usize, BinaryHeap<Reverse<usize>>>,
    disk_map: &[Option<u32>],
    file: (usize, usize),
    initial_start: usize,
) -> (usize, Option<usize>) {
    let find_next_free_space = |start: usize| -> Option<(usize, usize)> {
        let mut l = start;
        loop {
            match disk_map.get(l) {
                None => return None,
                Some(Some(_)) => {
                    l += 1;
                }
                Some(None) => break,
            }
        }
        let mut r = l;
        loop {
            match disk_map.get(r) {
                None => break,
                Some(Some(_)) => break,
                Some(None) => {
                    r += 1;
                }
            }
        }
        Some((l, r - 1))
    };

    let mut free_space = None;
    loop {
        free_space = find_next_free_space(free_space.map_or(initial_start, |(_, r)| r + 1));
        match free_space {
            None => break (disk_map.len() - 1, None),
            Some(free_space) => {
                let size = free_space.1 - free_space.0;
                if let Some(h) = free_spaces.get_mut(&size) {
                    h.push(Reverse(free_space.0));
                } else {
                    free_spaces.insert(size, BinaryHeap::from([Reverse(free_space.0)]));
                }

                if free_space.0 < file.0 && size >= (file.1 - file.0) {
                    break (free_space.1 + 1, Some(size));
                }
            }
        }
    }
}

fn checksum(disk_map: &[Option<u32>]) -> u64 {
    disk_map
        .iter()
        .enumerate()
        .map(|(idx, o)| match o {
            None => 0,
            Some(v) => (idx as u64) * (*v as u64),
        })
        .sum()
}

fn parse_puzzle_input() -> Vec<Option<u32>> {
    read_single_line("input/day_9.txt")
        .expect("Failed to open input file")
        .chars()
        .chunks(2)
        .into_iter()
        .zip(0u32..)
        .map(|(chunk, id)| {
            let sizes = chunk.collect_vec();
            let file_blocks = sizes[0].to_digit(10).unwrap() as usize;
            let free_blocks = if sizes.len() > 1 {
                sizes[1].to_digit(10).unwrap() as usize
            } else {
                0
            };
            std::iter::repeat_n(Some(id), file_blocks).chain(std::iter::repeat_n(None, free_blocks))
        })
        .flatten()
        .collect_vec()
}
