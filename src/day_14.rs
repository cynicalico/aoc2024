use aoc2024::read_lines;
use itertools::Itertools;
use regex::Regex;

/** https://adventofcode.com/2024/day/14 */
fn main() {
    let start = std::time::Instant::now();

    let robots = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(robots.clone(), 101, 103));
    println!("P2: {}", calculate_p2_ans(robots.clone(), 101, 103));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn calculate_p1_ans(mut robots: Vec<((i32, i32), (i32, i32))>, w: i32, h: i32) -> usize {
    for _ in 0..100 {
        for ((x, y), (dx, dy)) in &mut robots {
            *x = (*x + *dx).rem_euclid(w);
            *y = (*y + *dy).rem_euclid(h);
        }
    }

    robots
        .into_iter()
        .flat_map(|((x, y), _)| {
            if x == w / 2 || y == h / 2 {
                None
            } else {
                let lx = x < w / 2;
                let ly = y < h / 2;
                match (lx, ly) {
                    (true, true) => Some(1),
                    (true, false) => Some(2),
                    (false, true) => Some(3),
                    (false, false) => Some(4),
                }
            }
        })
        .counts()
        .values()
        .product()
}

fn calculate_p2_ans(mut robots: Vec<((i32, i32), (i32, i32))>, w: i32, h: i32) -> usize {
    let mut dist_avgs = Vec::<f32>::new();
    for _ in 0..w * h {
        for ((x, y), (dx, dy)) in &mut robots {
            *x = (*x + *dx).rem_euclid(w);
            *y = (*y + *dy).rem_euclid(h);
        }

        let mut dist_sum: i32 = 0;
        let mut n = 0;
        for i in 0..robots.len() {
            for j in i..robots.len() {
                dist_sum += (robots[j].0 .0 - robots[i].0 .0).pow(2)
                    + (robots[j].0 .1 - robots[i].0 .1).pow(2);
                n += 1;
            }
        }
        dist_avgs.push(dist_sum as f32 / n as f32);
    }

    dist_avgs
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.total_cmp(b.1))
        .unwrap()
        .0
        + 1
}

fn parse_puzzle_input() -> Vec<((i32, i32), (i32, i32))> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    read_lines("input/day_14.txt")
        .expect("Failed to open input file")
        .flatten()
        .map(|line| {
            let (_, [x, y, dx, dy]) = re.captures(&line).map(|c| c.extract()).unwrap();
            (
                (x.parse().unwrap(), y.parse().unwrap()),
                (dx.parse().unwrap(), dy.parse().unwrap()),
            )
        })
        .collect()
}
