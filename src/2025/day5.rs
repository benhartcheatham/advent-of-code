use std::fs;
use std::io;
use std::ops::RangeInclusive;

use aocutils::timeln;

fn overlap(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    (r2.start() <= r1.end() && r2.end() >= r1.end())
        || (r1.start() <= r2.end() && r1.end() >= r2.end())
}

fn merge(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    loop {
        let mut merged = Vec::new();

        for r in ranges.iter() {
            match merged.iter_mut().find(|m| overlap(r, m)) {
                Some(m) => *m = usize::min(*r.start(), *m.start())..=usize::max(*r.end(), *m.end()),
                None => merged.push(r.clone()),
            }
        }

        if ranges.eq(&merged) {
            break;
        }

        ranges = merged;
    }

    ranges
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let mut fresh = Vec::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next()
        && !line.trim_end().is_empty()
    {
        let nums = line
            .split("-")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        fresh.push(nums[0]..=nums[1]);
    }

    (
        merge(fresh),
        lines
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
    )
}

fn part1(input: &str) -> usize {
    let (fresh, ids) = parse_input(input);
    ids.iter()
        .filter(|n| fresh.iter().any(|r| r.contains(n)))
        .count()
}

fn part2(input: &str) -> usize {
    let (fresh, _) = parse_input(input);
    fresh.iter().map(|r| r.clone().count()).sum()
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day5.txt");

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
