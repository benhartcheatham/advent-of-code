use std::collections::HashSet;
use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::timing::Timer;

struct Antenna {
    pos: Coord,
    freq: char,
}

impl Antenna {
    fn new(pos: Coord, freq: char) -> Self {
        Antenna { pos, freq }
    }

    fn find_antinode(&self, other: &Antenna) -> Option<Coord> {
        if self.freq != other.freq || self.pos == other.pos {
            return None;
        }

        let slope = self.pos - other.pos;
        Some(self.pos - slope.mult_scalar(2))
    }

    fn find_all_antinodes(
        &self,
        other: &Antenna,
        (xbound, ybound): (usize, usize),
    ) -> Option<Vec<Coord>> {
        if self.freq != other.freq || self.pos == other.pos {
            return None;
        }

        let mut nodes = Vec::new();
        let slope = self.pos - other.pos;

        let mut node = self.pos - slope;
        while Antenna::in_bounds(node, xbound, ybound) {
            nodes.push(node);
            node -= slope;
        }

        Some(nodes)
    }

    fn in_bounds(pos: Coord, xbound: usize, ybound: usize) -> bool {
        if pos.x < 0 || pos.y < 0 {
            false
        } else {
            (pos.x as usize) < xbound && (pos.y as usize) < ybound
        }
    }
}

fn part1(input: &str) {
    let mut antenna = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let (xlen, ylen) = (lines.len(), lines[0].len());

    for (i, line) in lines.into_iter().enumerate() {
        for (j, ch) in line.char_indices() {
            if ch != '.' {
                antenna.push(Antenna::new(Coord::new(i as i64, j as i64), ch));
            }
        }
    }

    let mut antinodes = HashSet::new();
    for i in 0..antenna.len() {
        for j in 0..antenna.len() {
            if let Some(anti) = antenna[i].find_antinode(&antenna[j]) {
                if Antenna::in_bounds(anti, xlen, ylen) {
                    antinodes.insert(anti);
                }
            }
        }
    }

    print!("part1: {}", antinodes.len());
}

fn part2(input: &str) {
    let mut antenna = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let (xlen, ylen) = (lines.len(), lines[0].len());

    for (i, line) in lines.into_iter().enumerate() {
        for (j, ch) in line.char_indices() {
            if ch != '.' {
                antenna.push(Antenna::new(Coord::new(i as i64, j as i64), ch));
            }
        }
    }

    let mut antinodes = HashSet::new();
    for i in 0..antenna.len() {
        for j in 0..antenna.len() {
            if let Some(nodes) = antenna[i].find_all_antinodes(&antenna[j], (xlen, ylen)) {
                for n in nodes {
                    antinodes.insert(n);
                }
            }
        }
    }

    print!("part2: {}", antinodes.len());
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day8.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
