/* https://adventofcode.com/2024/day/1
 */

use aoc2024::read_single_line;
use itertools::Itertools;

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
    fn find_next_free_space(disk_map: &[Option<u32>], start: usize) -> Option<(usize, usize)> {
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
    }

    fn find_next_file(disk_map: &[Option<u32>], start: usize) -> Option<(usize, usize)> {
        let mut r = start;
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
                None => break,
                Some(Some(v)) => {
                    if *v != disk_map[r].unwrap() {
                        break;
                    }
                    l -= 1;
                }
                Some(None) => break,
            }
        }
        Some((l + 1, r))
    }

    fn move_file(disk_map: &mut [Option<u32>], file: (usize, usize), free_space: (usize, usize)) {
        for i in 0..=(file.1 - file.0) {
            disk_map[free_space.0 + i] = disk_map[file.0 + i];
            disk_map[file.0 + i] = None;
        }
    }

    let mut file = find_next_file(&disk_map, disk_map.len() - 1);
    loop {
        match file {
            None => break,
            Some(file) => {
                let mut free_space = find_next_free_space(&disk_map, 0);
                loop {
                    match free_space {
                        None => break,
                        Some(free_space) => {
                            if file.0 < free_space.0 {
                                break;
                            }
                            if (free_space.1 - free_space.0) >= (file.1 - file.0) {
                                move_file(&mut disk_map, file, free_space);
                            }
                        }
                    }
                    free_space = find_next_free_space(&disk_map, free_space.unwrap().1 + 1);
                }
            }
        }
        file = find_next_file(&disk_map, file.unwrap().0 - 1);
    }

    checksum(&disk_map)
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
