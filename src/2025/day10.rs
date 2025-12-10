extern crate lp_modeler;

use lp_modeler::dsl::*;
use lp_modeler::solvers::{CbcSolver, SolverTrait};

use std::collections::{HashMap, VecDeque};
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
        } else {
            for button in buttons {
                queue.push_back((n + 1, l, *button));
            }
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

fn part2(input: &str) -> usize {
    let mut presses = 0;

    for parts in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        let mut problem = LpProblem::new("Joltages", LpObjective::Minimize);
        let mut equations: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut buttons = Vec::new();
        let joltages: Vec<i32> = parts
            .last()
            .unwrap()
            .split(&['{', '}', ','])
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        for (i, button) in parts.iter().take(parts.len() - 1).skip(1).enumerate() {
            buttons.push(LpInteger::new(&format!("b{}", i + 1)));
            problem += buttons.last().unwrap().ge(0);

            for n in button
                .split(&['(', ')', ','])
                .filter_map(|s| s.parse::<usize>().ok())
            {
                equations
                    .entry(n)
                    .and_modify(|e| e.push(i))
                    .or_insert(vec![i]);
            }
        }

        problem += buttons
            .iter()
            .skip(2)
            .fold(&buttons[0] + &buttons[1], |expr, b| expr + b);

        for (i, j) in joltages.iter().enumerate() {
            problem += sum(equations.get(&i).unwrap(), |&b| &buttons[b]).equal(*j);
        }

        // crate expects the binary "cbc", but the fedora package provides a binary
        // called "Cbc".
        let solver = CbcSolver::new().command_name(String::from("Cbc"));
        match solver.run(&problem) {
            Ok(solution) => {
                presses += solution.results.values().sum::<f32>() as usize;
            }
            Err(msg) => {
                println!("{}", msg);
                return 0;
            }
        }
    }

    presses
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
