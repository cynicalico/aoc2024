use crate::util::graph::Graph;
use crate::util::grid::get_neighbors_4;
use crate::util::io::read_lines;
use hashlink::LinkedHashSet;
use std::collections::HashSet;
use std::io;

type G = Graph<(usize, usize), u32>;
type Input = (G, Vec<(usize, usize)>);

pub fn parse(filename: &str) -> io::Result<Input> {
    let map: Vec<Vec<u32>> = read_lines(filename)?
        .flatten()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    let mut graph = G::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                trailheads.push((y, x));
            }

            graph.add_node((y, x), map[y][x]);
            for neighbor in get_neighbors_4((y, x), map.len(), map[y].len())
                .into_iter()
                .filter(|(ny, nx)| map[*ny][*nx] == map[y][x] + 1)
            {
                graph.add_edge((y, x), neighbor);
            }
        }
    }

    Ok((graph, trailheads))
}

pub fn part1(input: &Input) -> Option<u32> {
    let ans = input
        .1
        .iter()
        .map(|start| score_trailhead(&input.0, start))
        .sum();
    Some(ans)
}

pub fn part2(input: &Input) -> Option<u32> {
    Some(input.1.iter().map(|start| rate_path(&input.0, start)).sum())
}

fn score_trailhead(graph: &G, start: &(usize, usize)) -> u32 {
    let mut score = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut stack: Vec<(usize, usize)> = Vec::from([*start]);
    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        if visited.insert(v) {
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
