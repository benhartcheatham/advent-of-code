use std::fs;
use std::io;

use crate::utils::{coord::*, grid::*};

fn cost_fn(grid: &[Vec<u8>], u: UCoord, v: UCoord) -> i64 {
    let (ux, uy) = u.into();
    let (vx, vy) = v.into();

    if grid[ux][uy] == grid[vx][vy]
        || grid[ux][uy] + 1 == grid[vx][vy]
        || grid[ux][uy] > grid[vx][vy]
    {
        1
    } else {
        i64::MAX
    }
}

fn part1(input: &str) {
    let mut cells = Vec::new();
    let (mut start, mut end) = (UCoord::new(0, 0), UCoord::new(0, 0));
    for (i, line) in input.lines().map(|l| l.as_bytes()).enumerate() {
        cells.push(Vec::new());

        for (j, b) in line.iter().enumerate() {
            match *b {
                b'a'..=b'z' => cells[i].push(*b),
                b'S' => {
                    start = UCoord::new(i, j);
                    cells[i].push(b'a');
                }
                b'E' => {
                    end = UCoord::new(i, j);
                    cells[i].push(b'z');
                }
                _ => panic!(),
            }
        }
    }

    let path = djikstra(&cells, start, end, cost_fn);

    println!("part1: {}", path.len());
}

fn part2(input: &str) {
    let mut cells = Vec::new();
    let mut end = UCoord::new(0, 0);
    let mut starts = Vec::new();

    for (i, line) in input.lines().map(|l| l.as_bytes()).enumerate() {
        cells.push(Vec::new());

        for (j, b) in line.iter().enumerate() {
            match *b {
                b'b'..=b'z' => cells[i].push(*b),
                b'a' | b'S' => {
                    starts.push(UCoord::new(i, j));
                    cells[i].push(b'a');
                }
                b'E' => {
                    end = UCoord::new(i, j);
                    cells[i].push(b'z');
                }
                _ => panic!(),
            }
        }
    }

    println!(
        "part2: {}",
        starts
            .iter()
            .filter_map(|s| {
                let len = djikstra(&cells, *s, end, cost_fn).len();
                if len != 0 {
                    Some(len)
                } else {
                    None
                }
            })
            .min()
            .unwrap_or(0)
    );
}

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day12.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}
