use std::collections::VecDeque;
use std::fs;
use std::io;

use aocutils::timeln;

fn min_presses(light: u64, buttons: &[u64]) -> usize {
    let mut queue: VecDeque<(usize, u64, u64)> = VecDeque::new();
    let mut min = usize::MAX;

    for b in buttons {
        queue.push_back((0, 0, *b));
    }

    while let Some((n, l, b)) = queue.pop_front()
        && (n + 1) < min
    {
        let l = l ^ b;
        if l == light {
            min = n + 1;
            continue;
        }

        for button in buttons {
            queue.push_back((n + 1, l, *button));
        }
    }

    min
}

fn part1(input: &str) -> usize {
    let mut lights = Vec::new();
    let mut buttons = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let light = parts[0]
            .chars()
            .skip(1)
            .take_while(|c| *c != ']')
            .enumerate()
            .map(|(i, c)| if c == '#' { 1 << i } else { 0 })
            .sum::<u64>();
        lights.push(light);

        buttons.push(Vec::new());
        let i = buttons.len() - 1;
        for button in parts.iter().take(parts.len() - 1).skip(1) {
            let b = button
                .split(&['(', ')', ','])
                .filter_map(|s| s.parse::<u64>().ok())
                .map(|n| 1 << n)
                .sum::<u64>();
            buttons[i].push(b);
        }
    }

    lights
        .into_iter()
        .zip(buttons.iter())
        .map(|(l, b)| min_presses(l, b))
        .sum::<usize>()
}

fn part2(input: &str) -> i32 {
    let mut joltages = Vec::new();
    let mut buttons = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let line_joltage: Vec<usize> = parts
            .last()
            .unwrap()
            .split(&['{', '}', ','])
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        joltages.push(line_joltage);

        buttons.push(Vec::new());
        let i = buttons.len() - 1;
        for button in parts.iter().take(parts.len() - 1).skip(1) {
            let b = button
                .split(&['(', ')', ','])
                .filter_map(|s| s.parse::<u64>().ok())
                .map(|n| 1 << n)
                .sum::<u64>();
            buttons[i].push(b);
        }
    }

    0
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day10.txt");

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
