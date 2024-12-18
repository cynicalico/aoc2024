use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use std::path::Path;

pub fn read<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_single_line<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut buf = String::new();
    io::BufReader::new(file).read_line(&mut buf)?;
    Ok(buf.trim_end().to_owned())
}

pub fn read_lines_partitioned<P, F1, F2>(filename: P, mut f1: F1, mut f2: F2) -> io::Result<()>
where
    P: AsRef<Path>,
    F1: FnMut(String),
    F2: FnMut(String),
{
    let mut seen_partition = false;
    for line in read_lines(filename)?.flatten() {
        if line.is_empty() {
            seen_partition = true;
            continue;
        }

        if !seen_partition {
            f1(line);
        } else {
            f2(line);
        }
    }

    Ok(())
}
