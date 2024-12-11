use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod ds;

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

pub fn get_neighbors(
    pos: (usize, usize),
    w: usize,
    h: usize,
    offsets: &[(i32, i32)],
) -> Vec<(usize, usize)> {
    offsets
        .iter()
        .flat_map(|(dy, dx)| {
            let new_pos = (pos.0 as i32 + dy, pos.1 as i32 + dx);
            (new_pos.0 >= 0 && new_pos.0 < h as i32 && new_pos.1 >= 0 && new_pos.1 < w as i32)
                .then_some((new_pos.0 as usize, new_pos.1 as usize))
        })
        .collect_vec()
}

pub fn get_neighbors_4(pos: (usize, usize), w: usize, h: usize) -> Vec<(usize, usize)> {
    get_neighbors(pos, w, h, &[(-1, 0), (1, 0), (0, -1), (0, 1)])
}

pub fn get_neighbors_8(pos: (usize, usize), w: usize, h: usize) -> Vec<(usize, usize)> {
    get_neighbors(
        pos,
        w,
        h,
        &[
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ],
    )
}
