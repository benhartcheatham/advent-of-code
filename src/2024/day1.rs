use std::collections::HashMap;
use std::fs;
use std::io;

use aocutils::timeln;

fn part1(input: &str) -> u64 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let parts: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        left.push(parts[0]);
        right.push(parts[1]);
    }

    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(n1, n2)| n1.abs_diff(*n2))
        .sum::<u64>()
}

fn part2(input: &str) -> i64 {
    let mut left = Vec::new();
    let mut right = HashMap::new();

    for line in input.lines() {
        let parts: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        left.push(parts[0]);
        right
            .entry(parts[1])
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
    }

    let mut map: HashMap<i64, i64> = HashMap::new();
    for n in &left {
        if let Some(cnt) = right.get(n) {
            map.entry(*n)
                .and_modify(|e| *e += n * cnt)
                .or_insert(n * cnt);
        }
    }

    map.values().sum::<i64>()
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day1.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
