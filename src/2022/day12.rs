use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::grid::algo::*;
use aocutils::timeln;

fn cost_fn(grid: &[Vec<u8>], u: Coord, _cost: usize, v: Coord) -> usize {
    let (ux, uy) = u.as_unsigned().unwrap();
    let (vx, vy) = v.as_unsigned().unwrap();

    if grid[ux][uy] == grid[vx][vy]
        || grid[ux][uy] + 1 == grid[vx][vy]
        || grid[ux][uy] > grid[vx][vy]
    {
        1
    } else {
        usize::MAX
    }
}

fn part1(input: &str) -> usize {
    let mut cells = Vec::new();
    let (mut start, mut end) = (Coord::new(0, 0), Coord::new(0, 0));
    for (i, line) in input.lines().map(|l| l.as_bytes()).enumerate() {
        cells.push(Vec::new());

        for (j, b) in line.iter().enumerate() {
            match *b {
                b'a'..=b'z' => cells[i].push(*b),
                b'S' => {
                    start = Coord::new(i as i64, j as i64);
                    cells[i].push(b'a');
                }
                b'E' => {
                    end = Coord::new(i as i64, j as i64);
                    cells[i].push(b'z');
                }
                _ => panic!(),
            }
        }
    }

    let path = djikstra(&cells, start, end, cost_fn).unwrap();

    path.len()
}

fn part2(input: &str) -> usize {
    let mut cells = Vec::new();
    let mut end = Coord::new(0, 0);
    let mut starts = Vec::new();

    for (i, line) in input.lines().map(|l| l.as_bytes()).enumerate() {
        cells.push(Vec::new());

        for (j, b) in line.iter().enumerate() {
            match *b {
                b'b'..=b'z' => cells[i].push(*b),
                b'a' | b'S' => {
                    starts.push(Coord::new(i as i64, j as i64));
                    cells[i].push(b'a');
                }
                b'E' => {
                    end = Coord::new(i as i64, j as i64);
                    cells[i].push(b'z');
                }
                _ => panic!(),
            }
        }
    }

    starts
        .iter()
        .filter_map(|s| {
            let len = djikstra(&cells, *s, end, cost_fn).unwrap().len();
            if len != 0 {
                Some(len)
            } else {
                None
            }
        })
        .min()
        .unwrap_or(0)
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day12.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
