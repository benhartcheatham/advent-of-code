use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::grid::in_bounds;
use aocutils::grid::{coord::GridCoord, direction::GridDirection, in_ibounds};
use aocutils::timing;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Guard {
    pos: Coord,
    dir: GridDirection,
}

impl Guard {
    fn new(pos: Coord, dir: GridDirection) -> Self {
        Guard { pos, dir }
    }

    fn simulate(&mut self, grid: &Vec<Vec<char>>) -> usize {
        let mut visited: Vec<Vec<bool>> = Vec::new();

        for _ in 0..grid.len() {
            visited.push(vec![false; grid[0].len()]);
        }

        while in_ibounds(grid, self.pos) {
            let p = GridCoord::from_coord(self.pos).unwrap();
            visited[p.x][p.y] = true;

            match GridCoord::from_coord(self.pos + self.dir.into()) {
                Some(next) if in_bounds(grid, next) => {
                    if grid[next.x][next.y] == '#' {
                        self.dir = self.dir.rotate_right();
                    } else {
                        self.pos = next.to_coord().unwrap();
                    }
                }
                _ => break,
            }
        }

        visited
            .into_iter()
            .map(|v| v.into_iter().filter(|e| *e).count())
            .sum::<usize>()
    }

    fn simulate_loop(&mut self, grid: &Vec<Vec<char>>) -> bool {
        let mut past: HashMap<Coord, Vec<GridDirection>> = HashMap::new();

        while in_ibounds(grid, self.pos) {
            if let Some(dirs) = past.get_mut(&self.pos) {
                if dirs.iter().any(|d| *d == self.dir) {
                    return true;
                }

                dirs.push(self.dir);
            } else {
                past.insert(self.pos, vec![self.dir]);
            }

            match GridCoord::from_coord(self.pos + self.dir.into()) {
                Some(next) if in_bounds(grid, next) => {
                    if grid[next.x][next.y] == '#' {
                        self.dir = self.dir.rotate_right();
                    } else {
                        self.pos = next.to_coord().unwrap();
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

    print!("part1: {}", guard.simulate(&grid));
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

    let mut cnt = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                continue;
            }

            let mut temp = grid.clone();
            temp[i][j] = '#';

            if guard.clone().simulate_loop(&temp) {
                cnt += 1;
            }
        }
    }

    print!("part2: {}", cnt);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day6.txt")?;
    let mut timer = timing::start_benchmark(benchmark);

    part1(&input);
    timing::print_time(&mut timer);
    part2(&input);
    timing::print_time(&mut timer);
    Ok(())
}
