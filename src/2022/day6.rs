use std::collections::{HashSet, VecDeque};
use std::fs;
use std::io;
use std::io::Error;

fn part1(input: &str) -> io::Result<()> {
    let mut last_chars: VecDeque<char> = VecDeque::new();

    for (i, c) in input.char_indices() {
        last_chars.push_back(c);

        if last_chars.len() == 4 {
            if HashSet::<&char>::from_iter(last_chars.iter()).len() == 4 {
                println!("part1: {}", i + 1);
                return Ok(());
            }

            last_chars.pop_front();
        }
    }

    Err(Error::new(io::ErrorKind::InvalidData, ""))
}

fn part2(input: &str) -> io::Result<()> {
    let mut last_chars: VecDeque<char> = VecDeque::new();

    for (i, c) in input.char_indices() {
        last_chars.push_back(c);

        if last_chars.len() == 14 {
            if HashSet::<&char>::from_iter(last_chars.iter()).len() == 14 {
                println!("part2: {}", i + 1);
                return Ok(());
            }

            last_chars.pop_front();
        }
    }

    Err(Error::new(io::ErrorKind::InvalidData, ""))
}

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day6.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}
