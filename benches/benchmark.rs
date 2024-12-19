#![allow(unstable_features)]
#![feature(test)]

extern crate test;

macro_rules! benchmark {
    ($year:tt $($day:tt),*) => {
        mod $year {$(
            mod $day {
                use aoc::$year::$day::*;
                use std::path::{Path, PathBuf};
                use std::sync::LazyLock;
                use test::Bencher;

                static FILEPATH: LazyLock<PathBuf> = LazyLock::new(|| {
                    let year = stringify!($year);
                    let day = stringify!($day);
                    Path::new("input").join(year).join(day).with_extension("txt")
                });

                #[bench]
                fn solve(b: &mut Bencher) {
                    b.iter(|| {
                        let input = parse(&FILEPATH.to_str().unwrap()).unwrap();
                        part1(&input);
                        part2(&input);
                    })
                }
            }
        )*}
    }
}

benchmark!(y2024
    day01, day02, day03, day04, day05,
    day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15,
    day16, day17, day18, day19
);
