use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::timeln;

fn determinant(a: Coord, b: Coord) -> i64 {
    a.x * b.y - b.x * a.y
}

fn presses(buttons: (Coord, Coord), target: Coord) -> i64 {
    // Uses Cramer's Rule, with non-integer solutions ruled out
    let (a, b) = (buttons.0, buttons.1);
    let det = determinant(a, b);

    if det == 0 {
        return 0;
    }

    let dx = determinant(target, b);
    let dy = determinant(a, target);

    if dx % det != 0 || dy % det != 0 {
        0
    } else {
        (dx / det) * 3 + (dy / det)
    }
}

fn part1(input: &str) -> i64 {
    let mut info = [Coord::new(0, 0), Coord::new(0, 0)];
    let mut cnt = 0;
    let mut i = 0;

    for line in input.lines().filter(|s| !s.trim().is_empty()) {
        let nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| {
                let n: String = s.chars().filter(char::is_ascii_digit).collect();
                if n.is_empty() {
                    None
                } else {
                    Some(n.parse::<i64>().unwrap())
                }
            })
            .collect();

        match i {
            0 | 1 => {
                info[i] = Coord::new(nums[0], nums[1]);
                i += 1;
            }
            2 => {
                cnt += presses(info.into(), Coord::new(nums[0], nums[1]));
                i = 0;
            }
            _ => panic!(),
        }
    }

    cnt
}

fn part2(input: &str) -> i64 {
    let mut info = [Coord::new(0, 0), Coord::new(0, 0)];
    let mut cnt = 0;
    let mut i = 0;

    for line in input.lines().filter(|s| !s.trim().is_empty()) {
        let nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| {
                let n: String = s.chars().filter(char::is_ascii_digit).collect();
                if n.is_empty() {
                    None
                } else {
                    Some(n.parse::<i64>().unwrap())
                }
            })
            .collect();

        match i {
            0 | 1 => {
                info[i] = Coord::new(nums[0], nums[1]);
                i += 1;
            }
            2 => {
                let x = 10_000_000_000_000;
                cnt += presses(info.into(), Coord::new(nums[0] + x, nums[1] + x));
                i = 0;
            }
            _ => panic!(),
        }
    }

    cnt
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day13.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
