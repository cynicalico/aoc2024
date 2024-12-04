use aoc2024::{read_lines, Arr2D};
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let word_search = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(&word_search));
    println!("P2: {}", calculate_p2_ans(&word_search));
}

fn calculate_p1_ans(word_search: &Arr2D<char>) -> u32 {
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

fn calculate_p2_ans(word_search: &Arr2D<char>) -> u32 {
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
fn check_x_mas(word_search: &Arr2D<char>, y: usize, x: usize) -> bool {
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

#[derive(PartialEq, Eq, Hash)]
enum SearchDirection {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn count_xmas(word_search: &Arr2D<char>, y: usize, x: usize) -> u32 {
    let word: Vec<char> = vec!['X', 'M', 'A', 'S'];

    let delta: HashMap<SearchDirection, (i32, i32)> = HashMap::from([
        (SearchDirection::Left, (0, -1)),
        (SearchDirection::Right, (0, 1)),
        (SearchDirection::Up, (-1, 0)),
        (SearchDirection::Down, (1, 0)),
        (SearchDirection::UpLeft, (-1, -1)),
        (SearchDirection::UpRight, (-1, 1)),
        (SearchDirection::DownLeft, (1, -1)),
        (SearchDirection::DownRight, (1, 1)),
    ]);

    let mut n = 0;

    let search_directions = get_valid_directions(word_search, y, x, word.len());
    for d in search_directions {
        let (dy, dx) = delta[&d];

        let mut found_word = true;
        for i in 0..word.len() {
            let pos = (
                (y as i32 + (i as i32 * dy)) as usize,
                (x as i32 + (i as i32 * dx)) as usize,
            );

            if word_search[pos] != word[i] {
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

fn get_valid_directions(
    word_search: &Arr2D<char>,
    y: usize,
    x: usize,
    word_len: usize,
) -> Vec<SearchDirection> {
    let mut search_directions = Vec::<SearchDirection>::new();

    let left_clearance = x >= word_len - 1;
    let right_clearance = x < word_search.width() - word_len + 1;
    let up_clearance = y >= word_len - 1;
    let down_clearance = y < word_search.height() - word_len + 1;

    if left_clearance {
        search_directions.push(SearchDirection::Left);
        if up_clearance {
            search_directions.push(SearchDirection::UpLeft);
        }
        if down_clearance {
            search_directions.push(SearchDirection::DownLeft);
        }
    }
    if right_clearance {
        search_directions.push(SearchDirection::Right);
        if up_clearance {
            search_directions.push(SearchDirection::UpRight);
        }
        if down_clearance {
            search_directions.push(SearchDirection::DownRight);
        }
    }
    if up_clearance {
        search_directions.push(SearchDirection::Up);
    }
    if down_clearance {
        search_directions.push(SearchDirection::Down);
    }

    search_directions
}

fn parse_puzzle_input() -> Arr2D<char> {
    let mut word_search = Arr2D::new(None, None);

    read_lines("input/day_4.txt")
        .unwrap()
        .flatten()
        .for_each(|line| {
            word_search.push_row(line.chars().collect_vec());
        });

    word_search
}
