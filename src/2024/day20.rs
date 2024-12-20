use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::grid::algo::djikstra;
use aocutils::timing::Timer;

fn cost_fn(grid: &[Vec<bool>], _curr: Coord, cost: usize, next: Coord) -> usize {
    let (x, y) = next.as_unsigned().unwrap();
    if grid[x][y] {
        usize::MAX
    } else {
        cost + 1
    }
}

fn cheats(
    grid: &[Vec<bool>],
    start: Coord,
    end: Coord,
    cheat_len: usize,
    time_save: usize,
) -> usize {
    let shortest = djikstra(grid, start, end, cost_fn).unwrap();
    let len = shortest.len();

    shortest
        .iter()
        .enumerate()
        .take(len - time_save)
        .map(|(i, c0)| {
            shortest
                .iter()
                .enumerate()
                .skip(time_save + i)
                .filter(|(j, c1)| {
                    let dist = c0.manhattan(c1) as usize;
                    dist <= cheat_len && j - i - dist >= time_save
                })
                .count()
        })
        .sum::<usize>()
}

fn part1(input: &str) {
    let mut grid = Vec::new();
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);

    for (i, line) in input.lines().enumerate() {
        grid.push(Vec::new());

        for (j, ch) in line.char_indices() {
            match ch {
                'S' => {
                    start = Coord::new(i as i64, j as i64);
                    grid[i].push(false);
                }
                'E' => {
                    end = Coord::new(i as i64, j as i64);
                    grid[i].push(false);
                }
                '.' => grid[i].push(false),
                _ => grid[i].push(true),
            }
        }
    }

    print!("part1: {}", cheats(&grid, start, end, 2, 100));
}

fn part2(input: &str) {
    let mut grid = Vec::new();
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);

    for (i, line) in input.lines().enumerate() {
        grid.push(Vec::new());

        for (j, ch) in line.char_indices() {
            match ch {
                'S' => {
                    start = Coord::new(i as i64, j as i64);
                    grid[i].push(false);
                }
                'E' => {
                    end = Coord::new(i as i64, j as i64);
                    grid[i].push(false);
                }
                '.' => grid[i].push(false),
                _ => grid[i].push(true),
            }
        }
    }

    print!("part2: {}", cheats(&grid, start, end, 20, 100));
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day20.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
