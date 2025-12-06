use std::fs;
use std::io;

use aocutils::timeln;

fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let ops: Vec<char> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();
    let mut answers: Vec<usize> = ops
        .iter()
        .map(|c| match c {
            '*' => 1,
            '+' => 0,
            _ => 0,
        })
        .collect();

    for line in lines.iter().take(lines.len() - 1) {
        for (i, n) in line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .enumerate()
        {
            answers[i] = match ops[i] {
                '*' => answers[i] * n,
                '+' => answers[i] + n,
                _ => panic!("Invalid op: {}", ops[i]),
            };
        }
    }

    answers.iter().sum()
}

fn part2(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut numbers = Vec::new();
    let mut answers = Vec::new();
    let mut skip = false;

    for j in (0..(lines[0].len())).rev() {
        if skip {
            numbers.clear();
            skip = false;
            continue;
        }

        numbers.push(String::new());
        for i in 0..lines.len() {
            match lines[i][j] {
                '+' => {
                    answers.push(
                        numbers
                            .iter()
                            .filter_map(|s| s.parse::<usize>().ok())
                            .sum::<usize>(),
                    );
                    skip = true;
                }
                '*' => {
                    answers.push(
                        numbers
                            .iter()
                            .map(|s| s.parse::<usize>().unwrap())
                            .reduce(|acc, e| acc * e)
                            .unwrap(),
                    );
                    skip = true;
                }
                ' ' => continue,
                _ => numbers.last_mut().unwrap().push(lines[i][j]),
            }
        }
    }

    answers.iter().sum()
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day6.txt");

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
