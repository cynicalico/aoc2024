/* https://adventofcode.com/2024/day/4
 */

use aoc2024::read_lines;
use itertools::Itertools;

fn main() {
    let start = std::time::Instant::now();

    let word_search = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(&word_search));
    println!("P2: {}", calculate_p2_ans(&word_search));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn calculate_p1_ans(word_search: &[Vec<char>]) -> u32 {
    let mut p1_ans = 0;

    for y in 0..word_search.len() {
        for x in 0..word_search[y].len() {
            if word_search[y][x] != 'X' {
                continue;
            }

            p1_ans += count_xmas(word_search, y, x);
        }
    }

    p1_ans
}

fn count_xmas(word_search: &[Vec<char>], y: usize, x: usize) -> u32 {
    let mut n = 0;

    let word: Vec<char> = vec!['X', 'M', 'A', 'S'];
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let mut found_word = true;
            for i in 0..word.len() {
                let pos = (
                    (y as i32 + (i as i32 * dy)) as usize,
                    (x as i32 + (i as i32 * dx)) as usize,
                );

                let c = word_search.get(pos.0).and_then(|row| row.get(pos.1));
                if *c.unwrap_or(&'\0') != word[i] {
                    found_word = false;
                    break;
                }
            }
            if found_word {
                n += 1;
            }
        }
    }

    n
}

fn calculate_p2_ans(word_search: &[Vec<char>]) -> u32 {
    let mut p2_ans = 0;

    for y in 1..word_search.len() - 1 {
        for x in 1..word_search[y].len() - 1 {
            if word_search[y][x] != 'A' {
                continue;
            }

            if check_x_mas(word_search, y, x) {
                p2_ans += 1;
            }
        }
    }

    p2_ans
}

fn check_x_mas(word_search: &[Vec<char>], y: usize, x: usize) -> bool {
    let tl = word_search[y - 1][x - 1];
    let bl = word_search[y + 1][x - 1];
    let tr = word_search[y - 1][x + 1];
    let br = word_search[y + 1][x + 1];

    // Only four possibilities, A is always anchored in the middle
    (tl == 'M' && bl == 'M' && tr == 'S' && br == 'S')
        || (tl == 'S' && bl == 'S' && tr == 'M' && br == 'M')
        || (tl == 'M' && bl == 'S' && tr == 'M' && br == 'S')
        || (tl == 'S' && bl == 'M' && tr == 'S' && br == 'M')
}

fn parse_puzzle_input() -> Vec<Vec<char>> {
    read_lines("input/day_4.txt")
        .unwrap()
        .flatten()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}
