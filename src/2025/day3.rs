use std::fs;
use std::io;

use aocutils::timeln;

fn bank_max(bank: &str, digits: usize) -> usize {
    let mut mdigs: Vec<usize> = vec![0; digits];

    for (i, n) in bank
        .char_indices()
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
    {
        for j in 0..mdigs.len() {
            if n > mdigs[j] && (mdigs.len() - j) <= (bank.len() - i) {
                mdigs[j] = n;

                for m in mdigs.iter_mut().skip(j + 1) {
                    *m = 0;
                }

                break;
            }
        }
    }

    mdigs
        .iter()
        .enumerate()
        .map(|(i, n)| 10_usize.pow((mdigs.len() - 1 - i) as u32) * n)
        .sum()
}

fn part1(input: &str) -> usize {
    input.lines().map(|s| bank_max(s, 2)).sum()
}

fn part2(input: &str) -> usize {
    input.lines().map(|s| bank_max(s, 12)).sum()
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day3.txt");

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
