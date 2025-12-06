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
            .map(|s| s.parse::<usize>().unwrap())
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
    let lines: Vec<&str> = input.lines().collect();
    let last = lines.last().unwrap();
    let ops: Vec<char> = last
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();
    let mut lens: Vec<usize> = last.split(&['+', '*']).map(|s| s.len()).skip(1).collect();
    let mut problems = Vec::new();

    *lens.last_mut().unwrap() += 1;
    for len in lens.iter().take(ops.len()) {
        problems.push(vec![String::new(); *len]);
    }

    for line in lines.iter().take(lines.len() - 1) {
        let mut n = 0;
        let mut p = 0;

        for c in line.chars().filter(|c| *c != '\n') {
            if c == ' ' {
                if n >= lens[p] {
                    p += 1;
                    n = 0;

                    continue;
                }
            } else {
                problems[p][n].push(c);
            }

            n += 1;
        }
    }

    problems
        .into_iter()
        .enumerate()
        .map(|(i, nums)| {
            nums.into_iter()
                .map(|s| s.parse::<usize>().unwrap())
                .reduce(|acc, e| match ops[i] {
                    '+' => acc + e,
                    '*' => acc * e,
                    _ => panic!("Invalid op: {}", ops[i]),
                })
                .unwrap()
        })
        .sum()
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
