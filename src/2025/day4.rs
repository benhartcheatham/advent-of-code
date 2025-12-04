use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::direction;
use aocutils::grid::in_bounds;
use aocutils::timeln;

fn count_movable(grid: &mut [Vec<char>], remove: bool) -> usize {
    let mut total = Vec::new();

    for i in 0..grid.len() {
        for j in (0..grid.len()).filter(|j| grid[i][*j] == '@') {
            let mut dirs = direction::DIRECTIONS.into_iter();
            let mut rolls = 0;

            while let Some(dir) = dirs.next()
                && rolls < 4
            {
                let c = Coord::new(i as i64, j as i64) + dir.into();

                if in_bounds(grid, c) && grid[c.x as usize][c.y as usize] == '@' {
                    rolls += 1;
                }
            }

            if rolls < 4 {
                total.push((i, j));
            }
        }
    }

    if remove {
        for (i, j) in &total {
            grid[*i][*j] = '.';
        }
    }

    total.len()
}

fn part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    count_movable(&mut grid, false)
}

fn part2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut cnt = usize::MAX;
    let mut total = 0;

    while cnt > 0 {
        cnt = count_movable(&mut grid, true);
        total += cnt;
    }

    total
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day4.txt");

    if let Err(e) = input {
        if e.kind() == io::ErrorKind::NotFound {
            println!("Input file not found!");
        }

        Err(e)
    } else {
        if benchmark {
            timeln!("part1: {}", part1(input.as_ref().unwrap()));
            timeln!("part2: {}", part2(input.as_ref().unwrap()));
        } else {
            println!("part1: {}", part1(input.as_ref().unwrap()));
            println!("part2: {}", part2(input.as_ref().unwrap()));
        }

        Ok(())
    }
}
