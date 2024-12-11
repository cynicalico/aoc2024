/* https://adventofcode.com/2024/day/10
 */

use aoc2024::{ds::Graph, get_neighbors_4, read_lines};
use hashlink::LinkedHashSet;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let start = std::time::Instant::now();

    let map = parse_puzzle_input();

    let trailheads = map
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, v)| (*v == 0).then_some((r, c)))
        })
        .collect_vec();

    let mut graph = G::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            graph.add_node((y, x), map[y][x]);
            for neighbor in get_neighbors_4((y, x), map.len(), map[y].len())
                .into_iter()
                .filter(|(ny, nx)| map[*ny][*nx] == map[y][x] + 1)
            {
                graph.add_edge((y, x), neighbor);
            }
        }
    }

    println!("P1: {}", calculate_p1_ans(&graph, &trailheads));
    println!("P2: {}", calculate_p2_ans(&graph, &trailheads));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

type G = Graph<(usize, usize), i32>;

fn calculate_p1_ans(graph: &G, trailheads: &[(usize, usize)]) -> u32 {
    trailheads
        .iter()
        .map(|start| score_trailhead(graph, start))
        .sum()
}

fn calculate_p2_ans(graph: &G, trailheads: &[(usize, usize)]) -> u32 {
    trailheads.iter().map(|start| rate_path(graph, start)).sum()
}

fn score_trailhead(graph: &G, start: &(usize, usize)) -> u32 {
    let mut score = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut stack: Vec<(usize, usize)> = Vec::from([*start]);
    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        if !visited.contains(&v) {
            visited.insert(v);

            if graph.val(&v).unwrap() == &9 {
                score += 1;
            } else {
                if let Some(neighbors) = graph.adj(&v) {
                    for n in neighbors {
                        stack.push(*n);
                    }
                }
            }
        }
    }

    score
}

fn rate_path(graph: &G, start: &(usize, usize)) -> u32 {
    fn inner(graph: &G, visited: &mut LinkedHashSet<(usize, usize)>, rating: &mut u32) {
        if let Some(adj) = graph.adj(visited.back().unwrap()) {
            for n in adj {
                if visited.contains(n) {
                    continue;
                }
                if graph.val(n).unwrap() == &9 {
                    *rating += 1;
                }
                visited.insert(*n);
                inner(graph, visited, rating);
                visited.pop_back();
            }
        }
    }

    let mut visited = LinkedHashSet::new();
    visited.insert(*start);
    let mut rating = 0;
    inner(graph, &mut visited, &mut rating);

    rating
}

fn parse_puzzle_input() -> Vec<Vec<i32>> {
    read_lines("input/day_10.txt")
        .expect("Failed to open input file")
        .flatten()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec()
}
