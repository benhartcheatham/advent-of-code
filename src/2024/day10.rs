use std::collections::HashSet;
use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::graph::Graph;
use aocutils::grid::direction::*;
use aocutils::grid::in_bounds;
use aocutils::timing::Timer;

fn can_traverse(grid: &[Vec<u32>], curr: Coord, next: Coord) -> bool {
    if !in_bounds(grid, next) {
        return false;
    }

    let (x, y) = curr.as_unsigned().unwrap();
    let (nx, ny) = next.as_unsigned().unwrap();

    grid[x][y] + 1 == grid[nx][ny]
}

fn build_graph(grid: &[Vec<u32>], head: Coord) -> Graph<(u32, Coord)> {
    let mut graph = Graph::new();
    let mut to_visit = Vec::new();

    let hv = graph.add_vertex((0, head), None);
    to_visit.push((head, hv));

    while let Some((curr, v)) = to_visit.pop() {
        for dir in DIRECTIONS {
            let next = curr + dir.into();

            if can_traverse(grid, curr, next) {
                let (nx, ny) = next.as_unsigned().unwrap();
                let v2 = graph.add_vertex((grid[nx][ny], next), None);
                graph.add_edge(v, v2, 1);

                if grid[nx][ny] < 9 {
                    to_visit.push((next, v2));
                }
            }
        }
    }

    graph
}

fn part1(input: &str) {
    let mut grid = Vec::new();
    let mut heads = Vec::new();

    for (i, line) in input.lines().enumerate() {
        grid.push(Vec::new());

        for (j, n) in line
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(u32::MAX))
            .enumerate()
        {
            if n == 0 {
                heads.push(Coord::new(i as i64, j as i64));
            }

            grid[i].push(n);
        }
    }

    let mut cnt = 0;
    for h in heads {
        let mut seen = HashSet::new();
        let graph = build_graph(&grid, h);

        for (_, coord) in graph
            .iter()
            .filter(|v| v.data.0 == 9)
            .map(|v| v.data)
        {
            seen.insert(coord);
        }

        cnt += seen.len();
    }

    print!("part1: {}", cnt);
}

fn part2(input: &str) {
    let mut grid = Vec::new();
    let mut heads = Vec::new();

    for (i, line) in input.lines().enumerate() {
        grid.push(Vec::new());

        for (j, n) in line
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(u32::MAX))
            .enumerate()
        {
            if n == 0 {
                heads.push(Coord::new(i as i64, j as i64));
            }

            grid[i].push(n);
        }
    }

    let mut cnt = 0;
    for h in heads {
        let graph = build_graph(&grid, h);
        cnt += graph
            .iter()
            .filter(|v| v.data.0 == 9)
            .count();
    }

    print!("part2: {}", cnt);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day10.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
