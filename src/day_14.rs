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

/**
 * Not my idea to solve it this way, I copied the solution from this comment
 * This problem sucks, I have no idea what they were thinking. Even this solution is
 * effectively relying on knowing what the final solution looks like.
 *
 * https://www.reddit.com/r/adventofcode/comments/1he0asr/comment/m1zzfsh
 */
fn calculate_p2_ans(mut robots: Vec<((i32, i32), (i32, i32))>, w: i32, h: i32) -> i32 {
    let mut bx = 0;
    let mut by = 0;
    let mut bxvar = f32::INFINITY;
    let mut byvar = f32::INFINITY;

    for t in 1..=w.max(h) {
        for ((x, y), (dx, dy)) in &mut robots {
            *x = (*x + *dx).rem_euclid(w);
            *y = (*y + *dy).rem_euclid(h);
        }

        let xs = robots.iter().map(|((x, _), _)| *x as f32).collect_vec();
        let ys = robots.iter().map(|((_, y), _)| *y as f32).collect_vec();
        let xvar = variance(&xs);
        let yvar = variance(&ys);
        if xvar < bxvar {
            bx = t;
            bxvar = xvar;
        }
        if yvar < byvar {
            by = t;
            byvar = yvar;
        }
    }

    // TODO: Make a 3 argument pow function to replicate Python's
    //       We're taking advantage of the fact that we know inverse(w) = 51
    //       In Python: pow(w, -1, h)
    bx + (((51 * (by - bx)) % h) * w)
}

fn variance(xs: &[f32]) -> f32 {
    let mean: f32 = xs.iter().sum::<f32>() / xs.len() as f32;
    xs.iter().map(|v| (v - mean).powf(2.0)).sum::<f32>() / (xs.len() - 1) as f32
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
