use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::grid::direction::GridDirection;
use aocutils::timeln;

#[derive(Debug)]
struct Tree {
    height: u8,
    is_visible: bool,
    scenic_score: usize,
}

impl Tree {
    fn new(height: u8) -> Self {
        Tree {
            height,
            is_visible: false,
            scenic_score: 0,
        }
    }
}

fn mark_visible(grid: &mut Vec<Vec<Tree>>, start: (i64, i64), dir: GridDirection, mut max: u8) {
    use GridDirection::*;

    let idx = (start.0 as usize, start.1 as usize);
    if idx.0 >= grid.len() || idx.1 >= grid.len() {
        return;
    }

    if grid[idx.0][idx.1].height > max {
        grid[idx.0][idx.1].is_visible = true;
        max = grid[idx.0][idx.1].height;
    }

    match dir {
        Up | Down => mark_visible(
            grid,
            (Into::<Coord>::into(dir).x + start.0, start.1),
            dir,
            max,
        ),
        Left | Right => mark_visible(
            grid,
            (start.0, Into::<Coord>::into(dir).y + start.1),
            dir,
            max,
        ),
    }
}

fn find_scenic_score(grid: &[Vec<Tree>], idx: (i64, i64)) -> usize {
    let uidx = (idx.0 as usize, idx.1 as usize);
    let treeh = grid[uidx.0][uidx.1].height;
    let mut nums: [usize; 4] = [0; 4];

    // Direction::Up
    for i in (0..uidx.0).rev() {
        nums[0] += 1;

        if grid[i][uidx.1].height >= treeh {
            break;
        }
    }

    // Direction::Down
    for r in grid.iter().skip(uidx.0 + 1) {
        nums[1] += 1;

        if r[uidx.1].height >= treeh {
            break;
        }
    }

    // Direction::Left
    for j in (0..uidx.1).rev() {
        nums[2] += 1;

        if grid[uidx.0][j].height >= treeh {
            break;
        }
    }

    // Direction::Right
    for tree in grid[uidx.0].iter().skip(uidx.1 + 1) {
        nums[3] += 1;

        if tree.height >= treeh {
            break;
        }
    }

    nums.into_iter().reduce(|acc, e| acc * e).unwrap()
}

fn part1(input: &str) -> usize {
    let mut grid = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(Tree::new((c.to_digit(10).unwrap() + 1) as u8));
        }

        grid.push(row);
    }

    let clen = grid[0].len() as i64;
    for i in 0..grid.len() {
        mark_visible(&mut grid, (i as i64, 0), GridDirection::Right, 0);
        mark_visible(&mut grid, (i as i64, clen - 1), GridDirection::Left, 0);
    }

    for i in 0..clen {
        let len = (grid.len() - 1) as i64;

        mark_visible(&mut grid, (0, i), GridDirection::Down, 0);
        mark_visible(&mut grid, (len, i), GridDirection::Up, 0);
    }

    grid.iter()
        .map(|r| r.iter().filter(|t| t.is_visible).count())
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let mut grid = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(Tree::new(c.to_digit(10).unwrap() as u8));
        }

        grid.push(row);
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j].scenic_score = find_scenic_score(&grid, (i as i64, j as i64));
        }
    }

    grid.iter()
        .map(|r| r.iter().map(|t| t.scenic_score).max().unwrap())
        .max()
        .unwrap()
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day8.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
