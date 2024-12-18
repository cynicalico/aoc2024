use crate::util::io::read_lines;
use std::io;

type Input = Vec<Vec<char>>;

pub fn parse(filepath: &str) -> io::Result<Input> {
    Ok(read_lines(filepath)?
        .flatten()
        .map(|line| line.chars().collect())
        .collect())
}

pub fn part1(input: &Input) -> Option<u32> {
    let mut ans = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] != 'X' {
                continue;
            }
            ans += count_xmas(input, y, x);
        }
    }
    Some(ans)
}

pub fn part2(input: &Input) -> Option<u32> {
    let mut ans = 0;
    for y in 1..input.len() - 1 {
        for x in 1..input[y].len() - 1 {
            if input[y][x] != 'A' {
                continue;
            }

            if check_x_mas(input, y, x) {
                ans += 1;
            }
        }
    }
    Some(ans)
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
