use crate::util::{io::read_lines, trie::*};
use hashbrown::HashMap;
use std::io;

type Input = Vec<u64>;

pub fn parse(filepath: &str) -> io::Result<Input> {
    let mut lines = read_lines(filepath)?.flatten().filter(|l| !l.is_empty());

    let trie = Trie::from(lines.next().unwrap().split(", "));

    let mut memo: HashMap<String, u64> = HashMap::new();
    Ok(lines.map(|l| count_possible(&trie, &l, &mut memo)).filter(|&n| n > 0).collect())
}

pub fn part1(input: &Input) -> Option<usize> { input.len().into() }

pub fn part2(input: &Input) -> Option<u64> { input.iter().sum::<u64>().into() }

fn count_possible(trie: &Trie, d: &str, memo: &mut HashMap<String, u64>) -> u64 {
    *match memo.get(d) {
        Some(c) => c,
        None => {
            let mut total = if trie.find(d) { 1 } else { 0 };
            for i in (1..d.len().min(trie.max_key_len + 1)).rev() {
                if trie.find(&d[..i]) {
                    total += count_possible(trie, &d[i..], memo);
                }
            }
            memo.entry_ref(d).or_insert(total)
        }
    }
}
