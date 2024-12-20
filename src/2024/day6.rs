use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::grid::{direction::GridDirection, in_bounds};
use aocutils::timing::Timer;

fn dir_to_usize(dir: GridDirection) -> usize {
    use GridDirection::*;

    match dir {
        Up => 0,
        Down => 1,
        Left => 2,
        Right => 3,
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    pos: Coord,
    dir: GridDirection,
}

impl Guard {
    fn new(pos: Coord, dir: GridDirection) -> Self {
        Guard { pos, dir }
    }

    fn simulate(&mut self, grid: &[Vec<char>]) -> Vec<Vec<bool>> {
        let mut visited: Vec<Vec<bool>> = Vec::new();

        for _ in 0..grid.len() {
            visited.push(vec![false; grid[0].len()]);
        }

        while in_bounds(grid, self.pos) {
            let (px, py) = self.pos.as_unsigned().unwrap();
            visited[px][py] = true;

            let next = self.pos + self.dir.into();
            match next.as_unsigned() {
                Some((nx, ny)) if in_bounds(grid, next) => {
                    if grid[nx][ny] == '#' {
                        self.dir = self.dir.rotate_right();
                    } else {
                        self.pos = next;
                    }
                }
                _ => break,
            }
        }

        visited
    }

    fn simulate_loop(&mut self, grid: &Vec<Vec<char>>) -> bool {
        let mut past: Vec<Vec<[bool; 4]>> = Vec::new();

        for r in grid {
            past.push(vec![[false; 4]; r.len()]);
        }

        while in_bounds(grid, self.pos) {
            let idx = dir_to_usize(self.dir);
            let (x, y) = match self.pos.as_unsigned() {
                Some(t) => t,
                _ => continue,
            };
            let dirs = past.get_mut(x).unwrap().get_mut(y).unwrap();

            if dirs[idx] {
                return true;
            } else {
                dirs[idx] = true;
            }

            let next = self.pos + self.dir.into();
            match next.as_unsigned() {
                Some((nx, ny)) if in_bounds(grid, next) => {
                    if grid[nx][ny] == '#' {
                        self.dir = self.dir.rotate_right();
                    } else {
                        self.pos = next;
                    }
                }
                _ => break,
            }
        }

        false
    }
}

fn part1(input: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut guard = Guard::new(Coord::new(0, 0), GridDirection::Up);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                guard.pos = Coord::new(i as i64, j as i64);
                break;
            }
        }
    }

    print!(
        "part1: {}",
        guard
            .simulate(&grid)
            .into_iter()
            .map(|v| v.into_iter().filter(|e| *e).count())
            .sum::<usize>()
    );
}

fn part2(input: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut guard = Guard::new(Coord::new(0, 0), GridDirection::Up);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                guard.pos = Coord::new(i as i64, j as i64);
                break;
            }
        }
    }

    let visited = guard.clone().simulate(&grid);
    let mut cnt = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' || !visited[i][j] {
                continue;
            }

            grid[i][j] = '#';

            if guard.clone().simulate_loop(&grid) {
                cnt += 1;
            }

            grid[i][j] = '.';
        }
    }

    print!("part2: {}", cnt);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day6.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
