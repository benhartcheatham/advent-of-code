use std::collections::HashMap;
use std::fs;
use std::io;

use aocutils::coord::*;
use aocutils::timeln;

struct Sensor {
    coord: Coord,
    radius: u64,
}

impl Sensor {
    fn new(coord: Coord, beacon: Coord) -> Self {
        Sensor {
            coord,
            radius: coord.manhattan(&beacon) as u64,
        }
    }

    fn in_radius(&self, coord: &Coord) -> bool {
        self.coord.manhattan(coord) as u64 <= self.radius
    }

    fn row_cover(&self, row: i64) -> Vec<Coord> {
        let mut cover = Vec::new();

        let coord = Coord::new(self.coord.x, row);
        if !self.in_radius(&coord) {
            return Vec::new();
        }

        for i in (self.coord.x - self.radius as i64)..=(self.coord.x + self.radius as i64) {
            let coord = Coord::new(i, row);

            if self.in_radius(&coord) {
                cover.push(coord);
            }
        }

        cover
    }

    fn cover(&self) -> HashMap<i64, Coord> {
        let mut cover = HashMap::new();
        let (ly, hy) = (
            self.coord.y - self.radius as i64,
            self.coord.y + self.radius as i64,
        );

        for i in 0..=((hy - ly) / 2) {
            cover.insert(ly + i, Coord::new(self.coord.x - i, self.coord.x + i));
        }

        for i in 0..((hy - ly) / 2) {
            cover.insert(hy - i, Coord::new(self.coord.x - i, self.coord.x + i));
        }

        cover
    }

    fn skip_cover(coord: Coord, cover: &HashMap<i64, Coord>) -> Option<Coord> {
        let (x, y) = coord.into();

        if let Some(c) = cover.get(&y) {
            if c.x <= x && x <= c.y {
                Some(Coord::new(c.y + 1, y))
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn part1(input: &str, row: i64) -> i64 {
    let mut sensors = Vec::new();

    for line in input.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| {
                if s.contains('=') {
                    s.chars()
                        .skip(2)
                        .filter(|c| c.is_ascii_digit() || *c == '-')
                        .collect::<String>()
                        .parse::<i64>()
                        .ok()
                } else {
                    None
                }
            })
            .collect();

        sensors.push(Sensor::new(
            Coord::new(nums[0], nums[1]),
            Coord::new(nums[2], nums[3]),
        ));
    }

    let mut min = i64::MAX;
    let mut max = i64::MIN;
    for s in sensors {
        let cover = s.row_cover(row);

        for c in cover {
            if c.x < min {
                min = c.x;
            }

            if c.x > max {
                max = c.x;
            }
        }
    }

    max - min
}

fn part2(input: &str, low: Coord, high: Coord) -> i64 {
    let mut sensors = Vec::new();

    for line in input.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| {
                if s.contains('=') {
                    s.chars()
                        .skip(2)
                        .filter(|c| c.is_ascii_digit() || *c == '-')
                        .collect::<String>()
                        .parse::<i64>()
                        .ok()
                } else {
                    None
                }
            })
            .collect();

        sensors.push(Sensor::new(
            Coord::new(nums[0], nums[1]),
            Coord::new(nums[2], nums[3]),
        ));
    }

    let covers: Vec<HashMap<i64, Coord>> = sensors.iter().map(|s| s.cover()).collect();
    let (mut x, mut y) = low.into();
    let mut skipped = true;
    while y <= high.y && !skipped {
        if x > high.x {
            y += 1;
            x = low.x;
        }

        skipped = false;
        for c in &covers {
            if let Some(coord) = Sensor::skip_cover(Coord::new(x, y), c) {
                (x, y) = coord.into();
                skipped = true;
                break;
            }
        }
    }

    x * 4_000_000 + y
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day15.txt")?;
    timeln!("part1: {}", part1(&input, 2_000_000));
    timeln!(
        "part2: {}",
        part2(&input, Coord::new(0, 0), Coord::new(4_000_000, 4_000_000))
    );

    Ok(())
}
