use std::fs;
use std::io;

use aocutils::timeln;

fn replace_min(maxes: &mut [u32], val: u32) {
    let (idx, min) = maxes
        .iter()
        .enumerate()
        .min_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();

    if val > *min {
        maxes[idx] = val;
    }
}

fn part1(input: &str) -> i32 {
    let mut highest = 0;
    let mut current = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            highest = highest.max(current);
            current = 0;
        } else {
            current += line.parse().unwrap_or(0);
        }
    }

    highest
}

fn part2(input: &str) -> u32 {
    let mut highest = [0; 3];
    let mut current = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            replace_min(&mut highest, current);
            current = 0;
        } else {
            current += line.parse().unwrap_or(0);
        }
    }

    replace_min(&mut highest, current);
    highest.iter().sum::<u32>()
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day1.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
