use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_partitioned<P>(filename: P) -> io::Result<(Vec<String>, Vec<String>)>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines().flatten().collect_vec();
    let (a, b) = lines.split_at(lines.iter().position(|s| s.is_empty()).unwrap());
    Ok((a.to_vec(), b[1..].to_vec()))
}
