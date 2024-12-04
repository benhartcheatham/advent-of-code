use std::fs;
use std::io;

use crate::utils::coord::*;
use crate::utils::grid::{in_bounds, in_ibounds};
use crate::utils::Direction;

const EXTRA_COLS: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Sand,
    Rock,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;

        match self {
            Air => write!(f, "."),
            Sand => write!(f, "o"),
            Rock => write!(f, "#"),
        }
    }
}

fn create_grid(input: &str, part2: bool) -> Vec<Vec<Cell>> {
    let mut rocks: Vec<Vec<UCoord>> = Vec::new();

    for (i, line) in input
        .lines()
        .map(|s| s.split(" -> ").collect::<Vec<&str>>())
        .enumerate()
    {
        rocks.push(Vec::new());

        for coord in line {
            let parts: Vec<usize> = coord
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            rocks[i].push(UCoord::new(parts[1], parts[0]));
        }
    }

    let xdim = rocks.iter().flatten().map(|c| c.get_x()).max().unwrap() + 1;
    let ydim = rocks.iter().flatten().map(|c| c.get_y()).max().unwrap() + 1;

    let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::Air; ydim]; xdim];

    // fill in rocks
    let origin = Coord::new(0, 0);
    for path in rocks {
        for i in 0..(path.len() - 1) {
            let start = path[i];
            let mut diff = path[i + 1] - path[i];

            grid[start.get_x()][start.get_y()] = Cell::Rock;
            while diff != origin {
                let c = Coord::new(
                    start.get_x() as i64 + diff.get_x(),
                    start.get_y() as i64 + diff.get_y(),
                );

                if let Some(c) = c.as_unsigned() {
                    if !in_bounds(&grid, c) {
                        continue;
                    }

                    grid[c.get_x()][c.get_y()] = Cell::Rock;
                }

                match diff.as_direction().unwrap() {
                    Direction::Up => diff = diff + Direction::Down.into(),
                    Direction::Down => diff = diff + Direction::Up.into(),
                    Direction::Left => diff = diff + Direction::Right.into(),
                    Direction::Right => diff = diff + Direction::Left.into(),
                }
            }
        }
    }

    if !part2 {
        return grid;
    }

    for row in &mut grid {
        let mut left = vec![Cell::Air; EXTRA_COLS];

        left.append(row);
        *row = left;
        row.append(&mut vec![Cell::Air; EXTRA_COLS]);
    }

    grid.push(vec![Cell::Air; grid[0].len()]);
    grid.push(vec![Cell::Rock; grid[0].len()]);

    grid
}

fn drop_in_bounds(grid: &Vec<Vec<Cell>>, c: UCoord) -> bool {
    use Direction::*;

    let below = c.as_signed().unwrap() + Down.into();
    let botleft = below + Left.into();
    let botright = below + Right.into();

    if !in_ibounds(grid, below) {
        return false;
    }

    if !in_ibounds(grid, botleft) {
        return false;
    }

    if !in_ibounds(grid, botright) {
        return false;
    }

    true
}

fn part1(input: &str) {
    use Direction::*;
    let mut grid = create_grid(input, false);
    let mut sand = UCoord::new(0, 500);

    while in_bounds(&grid, sand) {
        if !drop_in_bounds(&grid, sand) {
            break;
        }

        let below = sand.as_signed().unwrap() + Down.into();
        let botleft = (below + Left.into()).as_unsigned().unwrap();
        let botright = (below + Right.into()).as_unsigned().unwrap();
        let below = below.as_unsigned().unwrap();

        if grid[below.get_x()][below.get_y()] == Cell::Air {
            sand = below;
        } else if grid[botleft.get_x()][botleft.get_y()] == Cell::Air {
            sand = botleft;
        } else if grid[botright.get_x()][botright.get_y()] == Cell::Air {
            sand = botright;
        } else {
            grid[sand.get_x()][sand.get_y()] = Cell::Sand;
            sand = UCoord::new(0, 500);
        }
    }

    println!(
        "part1: {}",
        grid.into_iter()
            .map(|r| r.into_iter().filter(|c| *c == Cell::Sand).count())
            .sum::<usize>()
    );
}

fn part2(input: &str) {
    use Direction::*;
    let mut grid = create_grid(input, true);
    let mut sand = UCoord::new(0, 500 + EXTRA_COLS);

    while in_bounds(&grid, sand) {
        let below = sand.as_signed().unwrap() + Down.into();

        let botleft = (below + Left.into()).as_unsigned().unwrap();
        let botright = (below + Right.into()).as_unsigned().unwrap();
        let below = below.as_unsigned().unwrap();

        if grid[below.get_x()][below.get_y()] == Cell::Air {
            sand = below;
        } else if grid[botleft.get_x()][botleft.get_y()] == Cell::Air {
            sand = botleft;
        } else if grid[botright.get_x()][botright.get_y()] == Cell::Air {
            sand = botright;
        } else {
            grid[sand.get_x()][sand.get_y()] = Cell::Sand;
            if sand == UCoord::new(0, 500 + EXTRA_COLS) {
                break;
            }

            sand = UCoord::new(0, 500 + EXTRA_COLS);
        }
    }

    println!(
        "part2: {}",
        grid.iter()
            .map(|r| r.iter().filter(|c| **c == Cell::Sand).count())
            .sum::<usize>()
    );
}

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day14.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}
