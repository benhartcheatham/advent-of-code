use std::fs;
use std::io;

use aocutils::timeln;

fn simulate(input: &str) -> (usize, usize) {
    let mut grid = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let start = (0..grid[0].len()).find(|i| grid[0][*i] == 'S').unwrap();
    let mut beams = vec![false; grid[0].len()];
    let mut counts = vec![0; grid[0].len()];
    let mut splits = 0;

    beams[start] = true;
    counts[start] = 1;
    for row in grid {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                counts[j - 1] += counts[j];
                counts[j + 1] += counts[j];
                counts[j] = 0;

                if beams[j] {
                    beams[j - 1] = true;
                    beams[j + 1] = true;
                    beams[j] = false;
                    splits += 1;
                }
            }
        }
    }

    (splits, counts.iter().sum())
}

fn part1(input: &str) -> usize {
    simulate(input).0
}

fn part2(input: &str) -> usize {
    simulate(input).1
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day7.txt");

    if let Err(e) = input {
        if e.kind() == io::ErrorKind::NotFound {
            println!("Input file not found!");
        }

        Err(e)
    } else {
        if benchmark {
            timeln!("part1: {}", part1(input.as_ref().unwrap()));
            timeln!("part2: {}", part2(input.as_ref().unwrap()));
        } else {
            println!("part1: {}", part1(input.as_ref().unwrap()));
            println!("part2: {}", part2(input.as_ref().unwrap()));
        }

        Ok(())
    }
}
