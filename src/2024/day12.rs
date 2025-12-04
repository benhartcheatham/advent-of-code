use std::collections::HashSet;
use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::grid::direction::DIRECTIONS;
use aocutils::grid::direction::GridDirection;
use aocutils::grid::in_bounds;
use aocutils::timeln;

fn flood(grid: &[Vec<char>], coord: Coord) -> HashSet<Coord> {
    let mut points = HashSet::new();
    let mut to_visit = Vec::new();
    let (x, y) = coord.as_unsigned().unwrap();
    let this_ch = grid[x][y];

    to_visit.push(coord);
    while let Some(c) = to_visit.pop() {
        if points.contains(&c) {
            continue;
        }

        points.insert(c);

        for dir in DIRECTIONS {
            let next = c + dir.into();

            if let Some((x, y)) = next.as_unsigned()
                && in_bounds(grid, next)
                && grid[x][y] == this_ch
            {
                to_visit.push(next);
            }
        }
    }

    points
}

fn perimeter(points: &HashSet<Coord>) -> usize {
    let mut perimeter = 0;

    for p in points {
        for dir in DIRECTIONS {
            let adj = *p + dir.into();

            if points.get(&adj).is_none() {
                perimeter += 1;
            }
        }
    }

    perimeter
}

/// Gets the amount of sides of shape described by @points in @dir.
/// @min should be the smallest x coordinate in points if @dir == Up or Down,
/// and smallest y coordinate if @dir == Left or Right.
fn sides(points: &HashSet<Coord>, dir: GridDirection, mut min: i64) -> usize {
    use GridDirection::*;

    let xfilter = |x| points.iter().filter(|c| c.x == x).copied().collect();
    let xcmp = |a: &Coord, b: &Coord| a.y.cmp(&b.y);
    let xgap =
        |i: usize, row: &Vec<Coord>, void: bool| i > 0 && row[i - 1].y + 1 != row[i].y && void;

    let yfilter = |y| points.iter().filter(|c| c.y == y).copied().collect();
    let ycmp = |a: &Coord, b: &Coord| a.x.cmp(&b.x);
    let ygap =
        |i: usize, col: &Vec<Coord>, void: bool| i > 0 && col[i - 1].x + 1 != col[i].x && void;

    let mut sides = 0;
    let mut v: Vec<Coord> = match dir {
        Up | Down => xfilter(min),
        Left | Right => yfilter(min),
    };

    while !v.is_empty() {
        match dir {
            Up | Down => v.sort_by(xcmp),
            Left | Right => v.sort_by(ycmp),
        };

        let mut void = false;
        for i in 0..v.len() {
            let filled = points.contains(&(v[i] + dir.into()));

            // End of side
            if filled && void {
                sides += 1;
                void = false;
            }

            // Gap detected
            match dir {
                Up | Down => {
                    if xgap(i, &v, void) {
                        sides += 1;
                    }
                }
                Left | Right => {
                    if ygap(i, &v, void) {
                        sides += 1;
                    }
                }
            }

            void = !filled;
        }

        // Handle edge case where there is nothing on side
        // of last point
        if void {
            sides += 1;
        }

        min += 1;
        v = match dir {
            Up | Down => xfilter(min),
            Left | Right => yfilter(min),
        };
    }

    sides
}

fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.trim().chars().collect());
    }

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut cost = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if visited[i][j] {
                continue;
            }

            let points = flood(&grid, Coord::new(i as i64, j as i64));
            let perimeter = perimeter(&points);
            cost += points.len() * perimeter;

            for (x, y) in points.iter().map(|p| p.as_unsigned().unwrap()) {
                visited[x][y] = true;
            }
        }
    }

    cost
}

fn part2(input: &str) -> usize {
    use GridDirection::*;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.trim().chars().collect());
    }

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut cost = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if visited[i][j] {
                continue;
            }

            let points = flood(&grid, Coord::new(i as i64, j as i64));
            let x = points.iter().map(|c| c.x).min().unwrap();
            let y = points.iter().map(|c| c.y).min().unwrap();

            cost += points.len()
                * DIRECTIONS
                    .iter()
                    .map(|dir| match dir {
                        Up | Down => sides(&points, *dir, x),
                        Left | Right => sides(&points, *dir, y),
                    })
                    .sum::<usize>();

            for (x, y) in points.iter().map(|p| p.as_unsigned().unwrap()) {
                visited[x][y] = true;
            }
        }
    }

    cost
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day12.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
