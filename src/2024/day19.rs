use std::collections::HashMap;
use std::fs;
use std::io;

use aocutils::timing::Timer;

fn is_possible(haystack: &Vec<&str>, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }

    for h in haystack {
        if needle.starts_with(h) && is_possible(haystack, &needle[h.len()..]) {
            return true;
        }
    }

    false
}

fn possible_ways<'a>(
    haystack: &Vec<&str>,
    needle: &'a str,
    map: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(cnt) = map.get(needle) {
        return *cnt;
    } else if needle.is_empty() {
        return 1;
    }

    let mut cnt = 0;
    for h in haystack {
        if let Some(s) = needle.strip_prefix(h) {
            cnt += possible_ways(haystack, s, map);
        }
    }

    map.insert(needle, cnt);
    cnt
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let available: Vec<&str> = lines.next().unwrap().split(", ").collect();
    let mut patterns = Vec::new();

    for line in lines.skip(1) {
        patterns.push(line.trim());
    }

    print!(
        "part1: {}",
        patterns
            .iter()
            .filter(|p| is_possible(&available, p))
            .count()
    );
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let available: Vec<&str> = lines.next().unwrap().split(", ").collect();
    let mut patterns = Vec::new();
    let mut map = HashMap::new();

    for line in lines.skip(1) {
        patterns.push(line.trim());
    }

    print!(
        "part2: {}",
        patterns
            .iter()
            .map(|p| possible_ways(&available, p, &mut map))
            .sum::<usize>()
    );
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day19.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
