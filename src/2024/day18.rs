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

    corrupted.reverse();

    while let Some((x, y)) = corrupted.pop() {
        grid[x][y] = '#';

        if djikstra(
            &grid,
            Coord::new(0, 0),
            Coord::new(GRIDSIZE as i64 - 1, GRIDSIZE as i64 - 1),
            cost_fn,
        ).is_none() {
            print!("part2: {:?}", (y, x));
            break;
        }
    }
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day18.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
