use std::collections::HashMap;
use std::fs;
use std::io;

use aocutils::timeln;

fn calc_stone(stone: u64) -> (u64, Option<u64>) {
    match stone {
        0 => (1, None),
        x if x.ilog10() % 2 != 0 => {
            let s = x.to_string();
            let (top, bot) = s.split_at(s.len() / 2);

            (
                top.parse::<u64>().unwrap(),
                Some(bot.parse::<u64>().unwrap()),
            )
        }
        _ => (stone * 2024, None),
    }
}

fn update_stones_map((stone, blinks): (u64, usize), past: &mut HashMap<(u64, usize), u64>) -> u64 {
    if let Some(n) = past.get(&(stone, blinks - 1)) {
        *n
    } else {
        let n = get_stones((stone, blinks - 1), past);
        past.insert((stone, blinks - 1), n);
        n
    }
}

fn get_stones((stone, blinks): (u64, usize), past: &mut HashMap<(u64, usize), u64>) -> u64 {
    if let Some(n) = past.get(&(stone, blinks)) {
        return *n;
    }

    if blinks == 0 {
        return 1;
    }

    let (s1, s2) = calc_stone(stone);
    let cnt1 = update_stones_map((s1, blinks), past);
    let cnt2 = if let Some(s2) = s2 {
        update_stones_map((s2, blinks), past)
    } else {
        0
    };

    cnt1 + cnt2
}

fn part1(input: &str) -> u64 {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut map = HashMap::new();
    stones
        .into_iter()
        .map(|s| get_stones((s, 25), &mut map))
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut map = HashMap::new();
    stones
        .into_iter()
        .map(|s| get_stones((s, 75), &mut map))
        .sum::<u64>()
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day11.txt")?;

    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
