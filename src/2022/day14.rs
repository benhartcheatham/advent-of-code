use std::fs;
use std::io;

use aocutils::coord::*;
use aocutils::grid::{direction::*, in_bounds};
use aocutils::timing::Timer;

const EXTRA_COLS: i64 = 200;

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
    let mut rocks: Vec<Vec<Coord>> = Vec::new();

    for (i, line) in input
        .lines()
        .map(|s| s.split(" -> ").collect::<Vec<&str>>())
        .enumerate()
    {
        rocks.push(Vec::new());

        for coord in line {
            let parts: Vec<i64> = coord
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            rocks[i].push(Coord::new(parts[1], parts[0]));
        }
    }

    let xdim = rocks.iter().flatten().map(|c| c.x).max().unwrap() + 1;
    let ydim = rocks.iter().flatten().map(|c| c.y).max().unwrap() + 1;

    let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::Air; ydim as usize]; xdim as usize];

    // fill in rocks
    let origin = Coord::new(0, 0);
    for path in rocks {
        for i in 0..(path.len() - 1) {
            let start = path[i];
            let mut diff = path[i + 1] - path[i];

            grid[start.x as usize][start.y as usize] = Cell::Rock;
            while diff != origin {
                let c = Coord::new(start.x + diff.x, start.y + diff.y).abs();

                if !in_bounds(&grid, c) {
                    continue;
                }

                grid[c.x as usize][c.y as usize] = Cell::Rock;

                let temp: (i64, i64) = diff.unit().into();
                match temp {
                    (0, -1) => diff += GridDirection::Down.into(),
                    (0, 1) => diff += GridDirection::Up.into(),
                    (-1, 0) => diff += GridDirection::Right.into(),
                    (1, 0) => diff += GridDirection::Left.into(),
                    _ => panic!("Invalid direction: {:?}", diff.unit()),
                }
            }
        }
    }

    if !part2 {
        return grid;
    }

    for row in &mut grid {
        let mut left = vec![Cell::Air; EXTRA_COLS as usize];

        left.append(row);
        *row = left;
        row.append(&mut vec![Cell::Air; EXTRA_COLS as usize]);
    }

    grid.push(vec![Cell::Air; grid[0].len()]);
    grid.push(vec![Cell::Rock; grid[0].len()]);

    grid
}

fn drop_in_bounds(grid: &[Vec<Cell>], c: Coord) -> bool {
    use GridDirection::*;

    let below = Coord::new(c.x, c.y) + Down.into();
    let botleft = below + Left.into();
    let botright = below + Right.into();

    if !in_bounds(grid, below) {
        return false;
    }

    if !in_bounds(grid, botleft) {
        return false;
    }

    if !in_bounds(grid, botright) {
        return false;
    }

    true
}

fn part1(input: &str) {
    use GridDirection::*;
    let mut grid = create_grid(input, false);
    let mut sand = Coord::new(0, 500);

    while in_bounds(&grid, sand) {
        if !drop_in_bounds(&grid, sand) {
            break;
        }

        // no clue if this still works after all the changes I've made in utils crates
        let below = sand + Down.into();
        let botleft = (below + Left.into()).abs().as_unsigned().unwrap();
        let botright = (below + Right.into()).abs().as_unsigned().unwrap();
        let below = below.abs().as_unsigned().unwrap();

        if grid[below.0][below.1] == Cell::Air {
            sand = Coord::from_unsigned(&below).unwrap();
        } else if grid[botleft.0][botleft.1] == Cell::Air {
            sand = Coord::from_unsigned(&botleft).unwrap();
        } else if grid[botright.0][botright.1] == Cell::Air {
            sand = Coord::from_unsigned(&botright).unwrap();
        } else {
            let (sx, sy) = sand.as_unsigned().unwrap();
            grid[sx][sy] = Cell::Sand;
            sand = Coord::new(0, 500);
        }
    }

    print!(
        "part1: {}",
        grid.into_iter()
            .map(|r| r.into_iter().filter(|c| *c == Cell::Sand).count())
            .sum::<usize>()
    );
}

fn part2(input: &str) {
    use GridDirection::*;
    let mut grid = create_grid(input, true);
    let mut sand = Coord::new(0, 500 + EXTRA_COLS);

    while in_bounds(&grid, sand) {
        let below = sand + Down.into();

        let botleft = (below + Left.into()).abs().as_unsigned().unwrap();
        let botright = (below + Right.into()).abs().as_unsigned().unwrap();
        let below = below.abs().as_unsigned().unwrap();

        if grid[below.0][below.1] == Cell::Air {
            sand = Coord::from_unsigned(&below).unwrap();
        } else if grid[botleft.0][botleft.1] == Cell::Air {
            sand = Coord::from_unsigned(&botleft).unwrap();
        } else if grid[botright.0][botright.1] == Cell::Air {
            sand = Coord::from_unsigned(&botright).unwrap();
        } else {
            let (sx, sy) = sand.as_unsigned().unwrap();

            grid[sx][sy] = Cell::Sand;
            if sand == Coord::new(0, 500 + EXTRA_COLS) {
                break;
            }

            sand = Coord::new(0, 500 + EXTRA_COLS);
        }
    }

    print!(
        "part2: {}",
        grid.iter()
            .map(|r| r.iter().filter(|c| **c == Cell::Sand).count())
            .sum::<usize>()
    );
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day14.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
