use std::fs;
use std::io;

use aocutils::timeln;
use iter_tools::Itertools;

fn to_ranges(input: &str) -> Vec<(usize, usize)> {
    input
        .trim_end()
        .split(",")
        .map(|s| {
            let v = s
                .split("-")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (v[0], v[1])
        })
        .collect()
}

fn is_valid(n: &str) -> bool {
    if n.starts_with("0") && n.ne("0") {
        return false;
    }

    let len = n.len();
    if !len.is_multiple_of(2) {
        return true;
    }

    n.chars()
        .take(len / 2)
        .zip(n.chars().skip(len / 2))
        .any(|(n1, n2)| n1 != n2)
}

fn is_valid_part2(n: &str) -> bool {
    if n.starts_with("0") {
        return false;
    }

    if n.len() > 1 && n.chars().all_equal() {
        return false;
    }

    for i in (2..=(n.len() / 2)).filter(|i| n.len().is_multiple_of(*i)) {
        let mut v = Vec::new();
        let mut j = 0;

        while j < n.len() {
            v.push(n.chars().skip(j).take(i).collect::<String>());
            j += i;
        }

        if v.iter().all_equal() {
            return false;
        }
    }

    true
}

fn part1(input: &str) -> usize {
    to_ranges(input)
        .iter()
        .map(|r| {
            (r.0..=r.1)
                .filter(|n| !is_valid(&n.to_string()))
                .collect::<Vec<usize>>()
        })
        .map(|v| v.iter().sum::<usize>())
        .sum()
}

fn part2(input: &str) -> usize {
    to_ranges(input)
        .iter()
        .map(|r| {
            (r.0..=r.1)
                .filter(|n| !is_valid_part2(&n.to_string()))
                .collect::<Vec<usize>>()
        })
        .map(|v| v.iter().sum::<usize>())
        .sum()
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day2.txt");

    if let Err(e) = input {
        if e.kind() == io::ErrorKind::NotFound {
            println!("Input file not found!");
        }

        Err(e)
    } else {
        if benchmark {
            timeln!("part1: {}", part1(&input.as_ref().unwrap()));
            timeln!("part2: {}", part2(&input.as_ref().unwrap()));
        } else {
            println!("part1: {}", part1(&input.as_ref().unwrap()));
            println!("part2: {}", part2(&input.as_ref().unwrap()));
        }

        Ok(())
    }
}
