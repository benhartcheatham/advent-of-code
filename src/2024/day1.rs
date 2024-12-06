use std::collections::HashMap;
use std::fs;
use std::io;

use aocutils::timing;

fn part1(input: &str) {
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
    print!(
        "part1: {}",
        left.iter()
            .zip(right.iter())
            .map(|(n1, n2)| n1.abs_diff(*n2))
            .sum::<u64>()
    );
}

fn part2(input: &str) {
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

    print!("part2: {}", map.values().sum::<i64>());
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day1.txt")?;
    let mut timer = timing::start_benchmark(benchmark);

    part1(&input);
    timing::print_time(&mut timer);
    part2(&input);
    timing::print_time(&mut timer);

    Ok(())
}
