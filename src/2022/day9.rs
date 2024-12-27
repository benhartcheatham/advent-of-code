use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::direction::*;
use aocutils::grid::direction::GridDirection;
use aocutils::timeln;

fn move_head(mut hdx: (i64, i64), dir: Direction) -> (i64, i64) {
    use Direction::*;

    match dir {
        N | S => hdx.0 += Into::<Coord>::into(dir).x,
        W | E => hdx.1 += Into::<Coord>::into(dir).y,
        _ => panic!("Invalid direction {:?}", dir),
    }

    hdx
}

fn update_tail(hdx: (i64, i64), mut tdx: (i64, i64)) -> (i64, i64) {
    let xdiff = hdx.0 - tdx.0;
    let ydiff = hdx.1 - tdx.1;

    if xdiff.abs() > 1 {
        tdx.0 = if xdiff.signum() == 1 {
            tdx.0 + 1
        } else {
            tdx.0 - 1
        };

        if ydiff.abs() >= 1 {
            tdx.1 = if ydiff.signum() == 1 {
                tdx.1 + 1
            } else {
                tdx.1 - 1
            };
        }

        return tdx;
    }

    if ydiff.abs() > 1 {
        tdx.1 = if ydiff.signum() == 1 {
            tdx.1 + 1
        } else {
            tdx.1 - 1
        };

        if xdiff.abs() >= 1 {
            tdx.0 = if xdiff.signum() == 1 {
                tdx.0 + 1
            } else {
                tdx.0 - 1
            };
        }

        return tdx;
    }

    tdx
}

fn part1(input: &str) -> usize {
    let mut instructions: Vec<(Direction, usize)> = Vec::new();
    let mut seen: Vec<(i64, i64)> = Vec::new();

    for line in input.lines().map(|l| l.split(' ').collect::<Vec<&str>>()) {
        instructions.push((
            line[0].parse::<GridDirection>().unwrap().into(),
            line[1].parse::<usize>().unwrap(),
        ));
    }

    let mut head = (0, 0);
    let mut tail = head;

    seen.push(tail);
    for (dir, n) in instructions {
        for _ in 0..n {
            head = move_head(head, dir);
            tail = update_tail(head, tail);

            if !seen.contains(&tail) {
                seen.push(tail);
            }
        }
    }

    seen.len()
}

fn part2(input: &str) -> usize {
    let mut instructions: Vec<(Direction, usize)> = Vec::new();
    let mut seen: Vec<(i64, i64)> = Vec::new();

    for line in input.lines().map(|l| l.split(' ').collect::<Vec<&str>>()) {
        instructions.push((
            line[0].parse::<GridDirection>().unwrap().into(),
            line[1].parse::<usize>().unwrap(),
        ));
    }

    let mut knots: [(i64, i64); 10] = [(0, 0); 10];

    seen.push(knots[9]);
    for (dir, n) in instructions {
        for _ in 0..n {
            knots[0] = move_head(knots[0], dir);

            for i in 1..knots.len() {
                knots[i] = update_tail(knots[i - 1], knots[i]);
            }

            if !seen.contains(&knots[9]) {
                seen.push(knots[9]);
            }
        }
    }

    seen.len()
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day9.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
