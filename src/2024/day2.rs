use std::fs;
use std::io;

fn check_safety(level: &Vec<i64>) -> bool {
    let increasing = level[0] < level[1];

    for i in 0..(level.len() - 1) {
        let diff = level[i] - level[i + 1];

        if (increasing && diff >= 0) || (!increasing && diff <= 0) || diff.abs() > 3 {
            return false;
        }
    }

    true
}

fn part1(input: &str) {
    let mut levels: Vec<Vec<i64>> = Vec::new();

    for line in input.lines() {
        levels.push(
            line.split_whitespace()
                .map(|d| d.parse::<i64>().unwrap())
                .collect(),
        );
    }

    println!(
        "part1: {}",
        levels.iter().filter(|l| check_safety(l)).count()
    );
}

fn part2(input: &str) {
    let mut levels: Vec<Vec<i64>> = Vec::new();
    let mut count = 0;

    for line in input.lines() {
        levels.push(
            line.split_whitespace()
                .map(|d| d.parse::<i64>().unwrap())
                .collect(),
        );
    }

    for l in levels.iter_mut() {
        let mut safe = false;

        for i in 0..(l.len()) {
            let mut l2 = l.clone();
            l2.remove(i);

            if check_safety(&l2) {
                safe = true;
                break;
            }
        }

        if safe {
            count += 1;
        }
    }

    println!("part2: {}", count);
}

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day2.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}
