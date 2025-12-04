use std::collections::HashSet;
use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::timeln;

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

fn part1(input: &str) -> usize {
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
            if let Some(anti) = antenna[i].find_antinode(&antenna[j])
                && Antenna::in_bounds(anti, xlen, ylen)
            {
                antinodes.insert(anti);
            }
        }
    }

    antinodes.len()
}

fn part2(input: &str) -> usize {
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

    antinodes.len()
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day8.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
