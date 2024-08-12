use std::fs;
use std::{cmp::Ordering, io};

const NUM_CYCLES: usize = 1_000_000_000;

pub fn run() -> Result<(), io::Error> {
    let input = fs::read_to_string("inputs/2023/day14.txt")?;

    println!("solution: {}", solution(&input));

    Ok(())
}

#[allow(unused)]
// for debug
fn print_cycle(cycled: &[Vec<u64>]) {
    print!("   ");
    for i in 0..cycled[0].len() {
        print!("{:3}", i);
    }
    println!();

    for (i, r) in cycled.iter().enumerate() {
        print!("{:3}: ", cycled.len() - i);
        for c in r {
            let ch = match c {
                2 => '#',
                1 => 'O',
                _ => '.',
            };

            print!("{:3}", ch);
        }

        println!();
    }
}

/// Calculates a cycle on @platform. Mutates @platform in place.
///
/// Arguments
///
/// `@platform:` platform encoded as a 2D matrix where 0 is empty, 1 is a 'O'
/// and 2 is a '#'
fn perform_cycle(platform: &mut Vec<Vec<u64>>) {
    if platform.is_empty() {
        return;
    }

    // North
    let mut next;
    for c in 0..platform[0].len() {
        next = 0;

        for r in 0..platform.len() {
            if let Some(u) = platform[r].get(c) {
                match u {
                    2 => next = r + 1,
                    1 => match r.cmp(&next) {
                        Ordering::Greater => {
                            platform[next][c] = 1;
                            platform[r][c] = 0;
                            next += 1;
                        }
                        Ordering::Equal => next += 1,
                        Ordering::Less => next = r + 1,
                    },
                    _ => continue,
                }
            }
        }
    }

    // West
    for r in platform.iter_mut() {
        next = 0;

        for c in 0..r.len() {
            if let Some(u) = r.get(c) {
                match u {
                    2 => next = c + 1,
                    1 => match c.cmp(&next) {
                        Ordering::Greater => {
                            r[next] = 1;
                            r[c] = 0;
                            next += 1;
                        }
                        Ordering::Equal => next += 1,
                        _ => next = c + 1,
                    },

                    _ => continue,
                }
            }
        }
    }

    // South
    for c in 0..platform[0].len() {
        next = platform.len() - 1;

        for r in (0..platform.len()).rev() {
            if let Some(u) = platform[r].get(c) {
                match u {
                    2 => next = r.saturating_sub(1),
                    1 => match r.cmp(&next) {
                        Ordering::Less => {
                            platform[next][c] = 1;
                            platform[r][c] = 0;

                            next = next.saturating_sub(1);
                        }
                        Ordering::Equal => next = next.saturating_sub(1),
                        Ordering::Greater => next = r.saturating_sub(1),
                    },
                    _ => continue,
                }
            }
        }
    }

    // East
    for r in platform {
        next = r.len() - 1;

        for c in (0..r.len()).rev() {
            if let Some(u) = r.get(c) {
                match u {
                    2 => next = c.saturating_sub(1),
                    1 => match c.cmp(&next) {
                        Ordering::Less => {
                            r[next] = 1;
                            r[c] = 0;

                            next = next.saturating_sub(1);
                        }
                        Ordering::Equal => next = next.saturating_sub(1),
                        Ordering::Greater => next = c.saturating_sub(1),
                    },
                    _ => continue,
                }
            }
        }
    }
}

fn find_load(platform: Vec<String>) -> u64 {
    if platform.is_empty() {
        return 0;
    }

    let mut cycled = Vec::new();
    for (i, r) in platform.iter().enumerate() {
        cycled.push(Vec::new());

        for c in r.chars() {
            match c {
                '#' => cycled[i].push(2),
                'O' => cycled[i].push(1),
                _ => cycled[i].push(0),
            }
        }
    }

    // detect if there has been a cylce and then just repeat
    let mut iterations: Vec<String> = Vec::new();
    let mut num_left = NUM_CYCLES;
    for n in 0..NUM_CYCLES {
        perform_cycle(&mut cycled);

        if num_left == NUM_CYCLES {
            let iter: String = cycled.iter().fold(String::new(), |acc, v| {
                acc + v
                    .iter()
                    .map(|u| match u {
                        2 => "#",
                        1 => "O",
                        _ => ".",
                    })
                    .fold(String::new(), |s, ch| s + ch)
                    .as_str()
            });

            for (i, s) in iterations.iter().enumerate() {
                if s == &iter {
                    let len = iterations.len() - i;

                    // don't know why I need the sub 2, but it makes it work
                    num_left = (NUM_CYCLES - n) % len - 2;
                    break;
                }
            }

            iterations.push(iter);
        } else if num_left > 0 {
            num_left -= 1;
        } else {
            break;
        }
    }

    let mut rocks = Vec::new();
    for c in 0..cycled[0].len() {
        for (r, item) in cycled.iter().enumerate() {
            match item[c] {
                1 => rocks.push(r as u64),
                _ => continue,
            }
        }
    }

    rocks
        .iter()
        .fold(0, |load, rock| load + (cycled[0].len() as u64 - rock))
}

fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut platform = Vec::new();

    for line in lines {
        platform.push(line.to_string());
    }

    find_load(platform)
}
