use std::collections::{HashSet, VecDeque};
use std::fs;
use std::io;
use std::io::Error;

use aocutils::timing::Timer;

fn part1(input: &str) -> io::Result<()> {
    let mut last_chars: VecDeque<char> = VecDeque::new();

    for (i, c) in input.char_indices() {
        last_chars.push_back(c);

        if last_chars.len() == 4 {
            if HashSet::<&char>::from_iter(last_chars.iter()).len() == 4 {
                print!("part1: {}", i + 1);
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
                print!("part2: {}", i + 1);
                return Ok(());
            }

            last_chars.pop_front();
        }
    }

    Err(Error::new(io::ErrorKind::InvalidData, ""))
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day6.txt")?;
    let mut timer = Timer::start(benchmark);

    part1(&input)?;
    timer.print();
    timer.reset();
    part2(&input)?;
    timer.print();

    Ok(())
}
