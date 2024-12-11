/* https://adventofcode.com/2024/day/10
 */

use aoc2024::{ds::Graph, get_neighbors_4, read_lines};
use itertools::Itertools;

fn main() {
    let start = std::time::Instant::now();

    let map = parse_puzzle_input();

    println!("P1: {}", calculate_p1_ans(&map));
    println!("P2: {}", calculate_p2_ans(&map));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn calculate_p1_ans(map: &Vec<Vec<i32>>) -> i32 {
    get_trailheads(map)
        .into_iter()
        .map(|start| dfs_score(&map, start))
        .sum()
}

fn calculate_p2_ans(map: &Vec<Vec<i32>>) -> i32 {
    let mut graph = Graph::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            for neighbor in get_neighbors_4((y, x), map.len(), map[y].len())
                .into_iter()
                .filter(|(ny, nx)| map[*ny][*nx] == map[y][x] + 1)
            {
                graph.add_edge((y, x), neighbor);
            }
        }
    }

    let trailheads = get_trailheads(map);

    0
}

fn get_trailheads(map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(|(c, _)| (r, c))
                .collect_vec()
        })
        .collect_vec()
}

fn dfs_score(map: &Vec<Vec<i32>>, start: (usize, usize)) -> i32 {
    let mut score = 0;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

    let mut stack: Vec<(usize, usize)> = Vec::from([start]);
    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        if !visited[v.0][v.1] {
            visited[v.0][v.1] = true;

            if map[v.0][v.1] == 9 {
                score += 1;
            } else {
                for neighbor in get_neighbors_4(v, map.len(), map[0].len())
                    .into_iter()
                    .filter(|(y, x)| map[*y][*x] == map[v.0][v.1] + 1)
                {
                    stack.push(neighbor);
                }
            }
        }
    }

    score
}

fn parse_puzzle_input() -> Vec<Vec<i32>> {
    read_lines("input/example/day_10.txt")
        .expect("Failed to open input file")
        .flatten()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec()
}
