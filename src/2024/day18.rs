use std::fs;
use std::io;
use std::usize;

use aocutils::coord::Coord;
use aocutils::grid::algo::*;
use aocutils::timing::Timer;

static GRIDSIZE: usize = 71;
static TAKE_BYTES: usize = 1024;

fn cost_fn(grid: &[Vec<char>], _current: Coord, curr_cost: usize, next: Coord) -> usize {
    let (nx, ny) = next.as_unsigned().unwrap();

    if grid[nx][ny] == '#' {
        usize::MAX
    } else {
        curr_cost + 1
    }
}

fn part1(input: &str) {
    let mut grid = vec![vec!['.'; GRIDSIZE]; GRIDSIZE];

    for line in input.lines().take(TAKE_BYTES) {
        let coords: Vec<usize> = line
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect();
        grid[coords[1]][coords[0]] = '#';
    }

    print!(
        "part1: {}",
        djikstra(
            &grid,
            Coord::new(0, 0),
            Coord::new(GRIDSIZE as i64 - 1, GRIDSIZE as i64 - 1),
            cost_fn,
        )
        .unwrap()
        .len()
            - 1,
    );
}

fn binary_search(grid: &[Vec<char>], corrupted: &Vec<(usize, usize)>) -> usize {
    let (mut left, mut right) = (0, corrupted.len() - 1);
    let mut middle = 1;

    while left <= right {
        middle = (left + right) / 2;

        let mut g = grid.to_owned();
        for (x, y) in corrupted.iter().take(middle) {
            g[*x][*y] = '#';
        }

        let exists = djikstra(
            &g,
            Coord::new(0, 0),
            Coord::new(GRIDSIZE as i64 - 1, GRIDSIZE as i64 - 1),
            cost_fn,
        )
        .is_some();

        if exists && left == right {
            return middle;
        } else if exists {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    middle - 1
}

fn part2(input: &str) {
    let mut grid = vec![vec!['.'; GRIDSIZE]; GRIDSIZE];
    let mut corrupted = Vec::new();

    for line in input.lines() {
        let coords: Vec<usize> = line
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect();
        corrupted.push((coords[1], coords[0]));
    }

    for (x, y) in corrupted.drain(0..TAKE_BYTES) {
        grid[x][y] = '#';
    }

    let idx = binary_search(&grid, &corrupted);
    let c = corrupted[idx];
    print!("part2: {},{}", c.1, c.0);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day18.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
