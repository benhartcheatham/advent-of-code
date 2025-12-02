use std::fs;
use std::io;

use aocutils::timeln;

struct Dial {
    pos: i32,
}

impl Dial {
    fn new() -> Self {
        Dial { pos: 50 }
    }

    fn turn(&mut self, inst: &str) -> u32 {
        let neg = inst.starts_with("L");
        let mut n = inst
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let mut passed = 0;

        passed += n as u32 / 100;
        n %= 100;

        if neg {
            if self.pos < n && self.pos > 0 {
                passed += 1;
            }

            n *= -1;
        } else if self.pos + n > 100 {
            passed += 1;
        }

        self.pos = (self.pos + n) % 100;
        if self.pos < 0 {
            self.pos += 100;
        }

        passed
    }
}

fn part1(input: &str) -> usize {
    let mut dial = Dial::new();

    input
        .lines()
        .filter(|l| {
            dial.turn(l);
            dial.pos == 0
        })
        .count()
}

fn part2(input: &str) -> u32 {
    let mut dial = Dial::new();

    input
        .lines()
        .map(|l| dial.turn(l) + if dial.pos == 0 { 1 } else { 0 })
        .sum::<u32>()
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day1.txt");

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
