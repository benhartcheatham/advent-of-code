use std::fs;
use std::io;

use aocutils::timeln;
use regex::Regex;

fn part1(input: &str) -> i64 {
    let pattern = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut ret = 0;

    for (_, [n1, n2]) in pattern.captures_iter(input).map(|c| c.extract()) {
        ret += n1.parse::<i64>().unwrap() * n2.parse::<i64>().unwrap();
    }

    ret
}

fn part2(input: &str) -> i64 {
    let pattern = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(do\(\))|(don't\(\))").unwrap();
    let mulpat = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut enable = true;
    let mut ret = 0;

    for m in pattern.find_iter(input) {
        match m.as_str() {
            "do()" => {
                enable = true;
                continue;
            }
            "don't()" => {
                enable = false;
                continue;
            }
            _ => {
                if enable {
                    let (_, [n1, n2]) = mulpat.captures(m.as_str()).unwrap().extract();
                    ret += n1.parse::<i64>().unwrap() * n2.parse::<i64>().unwrap();
                }
            }
        }
    }

    ret
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day3.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
