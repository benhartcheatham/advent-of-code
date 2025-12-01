use std::fs;
use std::io;

use aocutils::timeln;

fn is_lock(input: &[Vec<char>]) -> bool {
    input[0].iter().all(|c| *c == '#')
}

fn to_heights(input: &[Vec<char>], is_lock: bool) -> Vec<usize> {
    let mut heights = if is_lock {
        vec![0; input[0].len()]
    } else {
        vec![input.len() - 1; input[0].len()]
    };

    for (i, row) in input.iter().enumerate() {
        for j in (0..input[i].len()).filter(|j| row[*j] == '#') {
            if is_lock {
                heights[j] = heights[j].max(i);
            } else {
                heights[j] = heights[j].min(i);
            }
        }
    }

    heights
}

fn insert(input: &[Vec<char>], locks: &mut Vec<Vec<usize>>, keys: &mut Vec<Vec<usize>>) {
    let is_lock = is_lock(input);
    let heights = to_heights(input, is_lock);

    if is_lock {
        locks.push(heights);
    } else {
        keys.push(heights);
    }
}

fn part1(input: &str) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut curr = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            insert(&curr, &mut locks, &mut keys);
            curr = Vec::new();
            continue;
        }

        curr.push(line.chars().collect::<Vec<char>>());
    }

    insert(&curr, &mut locks, &mut keys);

    locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| (0..lock.len()).all(|j| key[j] > lock[j]))
                .count()
        })
        .sum::<usize>()
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day25.txt")?;

    timeln!("part1: {}", part1(&input));

    Ok(())
}
