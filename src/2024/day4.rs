use std::fs;
use std::io;

use crate::utils::coord::Coord;
use crate::utils::direction::DIRECTIONS;
use crate::utils::grid::in_ibounds;

fn search(grid: &Vec<Vec<char>>, mut coord: Coord, xdir: i64, ydir: i64, needle: &str) -> u64 {
    for ch in needle.chars() {
        if !in_ibounds(grid, coord) {
            return 0;
        }

        let (x, y): (usize, usize) = Into::<Option<(usize, usize)>>::into(coord).unwrap();
        if grid[x][y] != ch {
            return 0;
        }

        coord += Coord::new(xdir, ydir);
    }

    1
}

fn search_cross(grid: &Vec<Vec<char>>, coord: Coord) -> u64 {
    if !in_ibounds(grid, coord) {
        return 0;
    }

    let (x, y): (usize, usize) = Into::<Option<(usize, usize)>>::into(coord).unwrap();
    if grid[x][y] != 'A' {
        return 0;
    }

    let to_check = [
        coord + Coord::new(-1, -1),
        coord + Coord::new(1, -1),
        coord + Coord::new(-1, 1),
        coord + Coord::new(1, 1),
    ];

    for c in to_check {
        if !in_ibounds(grid, c) {
            return 0;
        }
    }

    let to_check: Vec<char> = to_check
        .iter()
        .map(|c| {
            let (x, y): (usize, usize) = Into::<Option<(usize, usize)>>::into(*c).unwrap();
            grid[x][y]
        })
        .collect();

    for pair in [(to_check[0], to_check[3]), (to_check[1], to_check[2])] {
        match pair {
            ('M', 'S') | ('S', 'M') => continue,
            _ => return 0,
        }
    }

    1
}

fn part1(input: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut cnt = 0;

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            cnt += DIRECTIONS
                .into_iter()
                .map(Into::<(i64, i64)>::into)
                .fold(0, |acc, (x, y)| {
                    acc + search(&grid, Coord::new(i as i64, j as i64), x, y, "XMAS")
                });
        }
    }

    println!("part1: {}", cnt);
}

fn part2(input: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut cnt = 0;

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            cnt += search_cross(&grid, Coord::new(i as i64, j as i64));
        }
    }

    println!("part2: {}", cnt);
}

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day4.txt")?;

    part1(&input);
    part2(&input);
    Ok(())
}
