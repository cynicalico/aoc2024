use crate::util::io::read_lines_partitioned;
use crate::util::trie::*;
use hashbrown::HashMap;
use std::io;

type Input = (Trie, Vec<String>);

pub fn parse(filepath: &str) -> io::Result<Input> {
    let mut trie = Trie::new();
    let mut designs = Vec::new();

    read_lines_partitioned(
        filepath,
        |line| {
            line.split(", ").for_each(|pattern| trie.insert(pattern));
        },
        |line| {
            designs.push(line);
        },
    )?;

    Ok((trie, designs))
}

pub fn part1(input: &Input) -> Option<usize> {
    let (trie, designs) = input;
    let mut memo: HashMap<String, bool> = HashMap::new();
    designs
        .iter()
        .filter(|d| is_possible(&trie, d, &mut memo))
        .count()
        .into()
}

pub fn part2(input: &Input) -> Option<u64> {
    let (trie, designs) = input;
    let mut memo: HashMap<String, u64> = HashMap::new();
    designs
        .iter()
        .map(|d| count_possible(&trie, d, &mut memo))
        .sum::<u64>()
        .into()
}

fn is_possible(trie: &Trie, d: &str, memo: &mut HashMap<String, bool>) -> bool {
    if let Some(b) = memo.get(d) {
        *b
    } else {
        let res = d.len() <= trie.max_key_len && trie.find_terminal(d)
            || (1..d.len().min(trie.max_key_len + 1))
                .rev()
                .any(|i| trie.find_terminal(&d[..i]) && is_possible(trie, &d[i..], memo));
        memo.insert(d.to_string(), res);
        res
    }
}

fn count_possible(trie: &Trie, d: &str, memo: &mut HashMap<String, u64>) -> u64 {
    if let Some(c) = memo.get(d) {
        *c
    } else {
        let mut total = if trie.find_terminal(d) { 1 } else { 0 };
        for i in (1..d.len().min(trie.max_key_len + 1)).rev() {
            if trie.find_terminal(&d[..i]) {
                total += count_possible(trie, &d[i..], memo);
            }
        }
        memo.insert(d.to_string(), total);
        total
    }
}
