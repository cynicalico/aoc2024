use aoc::{util::parse::ParseOps, *};
use clap::Parser;
use std::{
    io,
    iter::empty,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

/// AoC Solutions Runner
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Run specific year
    #[arg(short, long, required = false)]
    year: Option<u32>,

    /// Run specific day
    #[arg(short, long, required = false)]
    day: Option<u32>,

    /// Input file to use
    #[arg(short, long, required = false, requires = "year", requires = "day")]
    file: Option<PathBuf>,

    /// Print totals
    #[arg(short, long, required = false)]
    totals: bool,
}

fn main() {
    let args = Args::parse();

    let solutions = empty()
        .chain(y2024())
        .filter(|s| args.year.is_none_or(|y| y == s.year))
        .filter(|s| args.day.is_none_or(|y| y == s.day));

    let mut solved = 0;
    let mut duration = Duration::ZERO;

    for Solution { year, day, filepath, wrapper } in solutions {
        let filepath = &args.file.as_ref().unwrap_or(&filepath);

        let instant = Instant::now();
        if let Ok((part1, part2)) = wrapper(filepath.to_str().unwrap()) {
            let elapsed = instant.elapsed();

            solved += if part1.is_some() { 1 } else { 0 };
            solved += if part2.is_some() { 1 } else { 0 };
            duration += elapsed;

            println!("{year} Day {day:02}");
            println!("  Part 1: {}", part1.unwrap_or("unsolved".to_owned()));
            println!("  Part 2: {}", part2.unwrap_or("unsolved".to_owned()));
            println!("  Elapsed: {:.03} s", elapsed.as_nanos() as f64 / 1e9);
            println!();
        } else {
            println!("{year} Day {day:02}");
            println!("  Missing input!");
            println!("  Place input file in {}", filepath.display());
            println!();
        }
    }

    if args.totals {
        println!("⭐ {solved}");
        println!("🕓 {:.03} s", duration.as_nanos() as f64 / 1e9);
        println!();
    }
}

struct Solution {
    year: u32,
    day: u32,
    filepath: PathBuf,
    wrapper: fn(&str) -> io::Result<(Option<String>, Option<String>)>,
}

macro_rules! make_solutions {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year);
                let day = stringify!($day);

                let filepath = Path::new("input").join(year).join(day).with_extension("txt");

                let wrapper = |filepath: &str| {
                    use $year::$day::*;

                    let input = parse(filepath)?;
                    let part1 = part1(&input);
                    let part2 = part2(&input);

                    Ok((part1.map(|v| v.to_string()), part2.map(|v| v.to_string())))
                };

                Solution { year: year.unsigned(), day: day.unsigned(), filepath, wrapper }
            },)*]
        }
    }
}

make_solutions!(y2024
    day01, day02, day03, day04, day05,
    day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15,
    day16, day17, day18, day19, day20
);
