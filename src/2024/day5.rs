use std::collections::HashMap;
use std::fs;
use std::io;

use aocutils::timing;

fn validate_pages(rules: &HashMap<usize, Vec<usize>>, pages: &Vec<usize>) -> usize {
    let mut printed = [false; 100];

    for p in pages {
        if let Some(rules) = rules.get(p) {
            if rules.iter().any(|p| printed[*p]) {
                return 0;
            }
        }

        printed[*p] = true;
    }

    pages[pages.len() / 2]
}

fn fix_pages(rules: &HashMap<usize, Vec<usize>>, pages: &Vec<usize>) -> usize {
    if validate_pages(rules, pages) > 0 {
        return 0;
    }

    let keys = rules.keys();
    let mut rules = rules.clone();
    for k in keys {
        if !pages.contains(k) {
            rules.remove(k);
        }
    }

    let mut fixed: Vec<usize> = Vec::new();
    for p in pages {
        let mut idx = usize::MAX;

        for r in rules.get(p).unwrap_or(&vec![]) {
            if let Some(pos) = fixed.iter().position(|page| page == r) {
                idx = idx.min(pos);
            }
        }

        if idx == usize::MAX {
            fixed.push(*p);
        } else {
            fixed.insert(idx, *p);
        }
    }

    fixed[fixed.len() / 2]
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        let pages: Vec<usize> = line
            .split('|')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        rules
            .entry(pages[0])
            .and_modify(|r| r.push(pages[1]))
            .or_insert(vec![pages[1]]);
    }

    let mut pages: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        pages.push(
            line.split(',')
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect(),
        );
    }

    (rules, pages)
}

fn part1(input: &str) {
    let (rules, pages) = parse_input(input);

    print!(
        "part1: {}",
        pages
            .iter()
            .map(|p| validate_pages(&rules, p))
            .sum::<usize>()
    );
}

fn part2(input: &str) {
    let (rules, pages) = parse_input(input);

    print!(
        "part2: {}",
        pages.iter().map(|p| fix_pages(&rules, p)).sum::<usize>()
    );
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day5.txt")?;
    let mut timer = timing::start_benchmark(benchmark);

    part1(&input);
    timing::print_time(&mut timer);
    part2(&input);
    timing::print_time(&mut timer);
    Ok(())
}
