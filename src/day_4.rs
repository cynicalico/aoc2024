use aoc2024::{read_lines, Vec2D};
use itertools::Itertools;

fn main() {
    let word_search = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(&word_search));
    println!("P2: {}", calculate_p2_ans(&word_search));
}

fn calculate_p1_ans(word_search: &Vec2D<char>) -> u32 {
    let mut p1_ans = 0;

    for y in 0..word_search.height() {
        for x in 0..word_search.width() {
            if word_search[(y, x)] != 'X' {
                continue;
            }

            p1_ans += count_xmas(word_search, y, x);
        }
    }

    p1_ans
}

fn count_xmas(word_search: &Vec2D<char>, y: usize, x: usize) -> u32 {
    let mut n = 0;

    let word: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let deltas: Vec<(i32, i32)> = vec![
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for (dy, dx) in deltas {
        let mut found_word = true;
        for i in 0..word.len() {
            let pos = (
                (y as i32 + (i as i32 * dy)) as usize,
                (x as i32 + (i as i32 * dx)) as usize,
            );

            if *word_search.get(pos.0, pos.1).unwrap_or(&'\0') != word[i] {
                found_word = false;
                break;
            }
        }
        if found_word {
            n += 1;
        }
    }

    n
}

fn calculate_p2_ans(word_search: &Vec2D<char>) -> u32 {
    let mut p2_ans = 0;

    for y in 1..word_search.height() - 1 {
        for x in 1..word_search.width() - 1 {
            if word_search[(y, x)] != 'A' {
                continue;
            }

            if check_x_mas(word_search, y, x) {
                p2_ans += 1;
            }
        }
    }

    p2_ans
}

#[rustfmt::skip]
fn check_x_mas(word_search: &Vec2D<char>, y: usize, x: usize) -> bool {
    // Only four possibilities, A is always anchored in the middle
    (word_search[(y - 1, x - 1)] == 'M'
        && word_search[(y + 1, x - 1)] == 'M'
        && word_search[(y - 1, x + 1)] == 'S'
        && word_search[(y + 1, x + 1)] == 'S') ||
    (word_search[(y - 1, x - 1)] == 'S'
        && word_search[(y + 1, x - 1)] == 'S'
        && word_search[(y - 1, x + 1)] == 'M'
        && word_search[(y + 1, x + 1)] == 'M') ||
    (word_search[(y - 1, x - 1)] == 'M'
        && word_search[(y + 1, x - 1)] == 'S'
        && word_search[(y - 1, x + 1)] == 'M'
        && word_search[(y + 1, x + 1)] == 'S') ||
    (word_search[(y - 1, x - 1)] == 'S'
        && word_search[(y + 1, x - 1)] == 'M'
        && word_search[(y - 1, x + 1)] == 'S'
        && word_search[(y + 1, x + 1)] == 'M')
}

fn parse_puzzle_input() -> Vec2D<char> {
    let mut word_search = Vec2D::new(None, None);

    read_lines("input/day_4.txt")
        .unwrap()
        .flatten()
        .for_each(|line| {
            word_search.push_row(line.chars().collect_vec());
        });

    word_search
}
