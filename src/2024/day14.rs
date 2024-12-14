use std::collections::HashSet;
use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::timing::Timer;

struct Robot {
    pos: Coord,
    vel: Coord,
}

impl Robot {
    fn new(pos: Coord, vel: Coord) -> Self {
        Robot { pos, vel }
    }

    fn simulate(&mut self, (xbound, ybound): (usize, usize), seconds: usize) {
        let (xbound, ybound) = (xbound as i64, ybound as i64);
        if xbound < 0 || ybound < 0 {
            return;
        }

        for _ in 0..seconds {
            self.pos += self.vel;

            self.pos.x = match self.pos.x {
                x if x < 0 => xbound + x,
                x if x >= xbound => x - xbound,
                _ => self.pos.x,
            };

            self.pos.y = match self.pos.y {
                y if y < 0 => ybound + y,
                y if y >= ybound => y - ybound,
                _ => self.pos.y,
            };
        }
    }
}

fn part1(input: &str) {
    let mut robots = Vec::new();
    let (rows, cols) = (103, 101);

    for line in input.lines() {
        let coords: Vec<Vec<i64>> = line
            .split_whitespace()
            .map(|s| s.split('=').nth(1).unwrap())
            .map(|s| {
                s.split(',')
                    .map(|d| d.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect();

        robots.push(Robot::new(
            Coord::new(coords[0][1], coords[0][0]),
            Coord::new(coords[1][1], coords[1][0]),
        ));
    }

    for r in robots.iter_mut() {
        r.simulate((rows, cols), 100);
    }

    let mut quads = [0, 0, 0, 0];
    let (xhalf, yhalf) = (rows as i64 / 2, cols as i64 / 2);
    for r in robots
        .iter()
        .filter(|r| r.pos.x != xhalf && r.pos.y != yhalf)
    {
        let mut i = 0;

        if r.pos.x > xhalf {
            i += 2;
        }

        if r.pos.y > yhalf {
            i += 1;
        }

        quads[i] += 1;
    }

    print!("part1: {}", quads.iter().product::<usize>());
}

fn part2(input: &str) {
    let mut robots = Vec::new();
    let (rows, cols) = (103, 101);

    for line in input.lines() {
        let coords: Vec<Vec<i64>> = line
            .split_whitespace()
            .map(|s| s.split('=').nth(1).unwrap())
            .map(|s| {
                s.split(',')
                    .map(|d| d.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect();

        robots.push(Robot::new(
            Coord::new(coords[0][1], coords[0][0]),
            Coord::new(coords[1][1], coords[1][0]),
        ));
    }

    let mut i = 0;
    let mut found = false;
    loop {
        for r in robots.iter_mut() {
            r.simulate((rows, cols), 1);
        }

        let set: HashSet<Coord> = HashSet::from_iter(robots.iter().map(|r| r.pos));
        for p in &set {
            let mut j = 1;
            for _ in 1..8 {
                if !set.contains(&Coord::new(p.x, p.y + j)) {
                    break;
                }

                j += 1;
            }

            if j == 8 {
                found = true;
                break;
            }
        }

        if found {
            break;
        }

        i += 1;
    }

    print!("part2: {}", i + 1);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day14.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
