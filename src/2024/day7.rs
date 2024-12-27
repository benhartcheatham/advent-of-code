use std::fs;
use std::io;

use aocutils::timeln;

enum Ops {
    Add,
    Mul,
    Concat,
}

impl Ops {
    fn apply(&self, lhs: usize, rhs: usize) -> usize {
        use Ops::*;

        match self {
            Add => lhs - rhs,
            Mul => lhs / rhs,
            Concat => lhs
                .to_string()
                .strip_suffix(&rhs.to_string())
                .unwrap()
                .parse::<usize>()
                .unwrap_or(0),
        }
    }

    fn is_applicable(&self, lhs: usize, rhs: usize) -> bool {
        use Ops::*;

        match self {
            Add => lhs > rhs,
            Mul => lhs % rhs == 0,
            Concat => {
                let lhs = lhs.to_string();
                let rhs = rhs.to_string();

                lhs != rhs && lhs.ends_with(&rhs)
            }
        }
    }
}

struct Equation {
    goal: usize,
    operands: Vec<usize>,
    allowed_ops: Vec<Ops>,
}

impl Equation {
    fn new(goal: usize, operands: Vec<usize>, allowed: Vec<Ops>) -> Self {
        Equation {
            goal,
            operands,
            allowed_ops: allowed,
        }
    }

    fn solve_helper(&self, curr: usize, idx: usize) -> bool {
        if idx == 0 {
            return self
                .allowed_ops
                .iter()
                .filter(|op| op.is_applicable(curr, self.operands[idx]))
                .any(|op| match op {
                    Ops::Mul => op.apply(curr, self.operands[idx]) == 1,
                    _ => op.apply(curr, self.operands[idx]) == 0,
                });
        }

        self.allowed_ops
            .iter()
            .filter(|op| op.is_applicable(curr, self.operands[idx]))
            .map(|op| self.solve_helper(op.apply(curr, self.operands[idx]), idx - 1))
            .any(|e| e)
    }

    fn solve(&self) -> bool {
        if self.operands.is_empty() {
            false
        } else if self.operands.len() == 1 {
            self.operands[0] == self.goal
        } else {
            let idx = self.operands.len() - 1;
            self.solve_helper(self.goal, idx)
        }
    }
}

fn part1(input: &str) -> usize {
    let mut eqs = Vec::new();

    for line in input.lines() {
        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|s| {
                s.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();

        eqs.push(Equation::new(
            parts[0],
            Vec::from(&parts[1..]),
            vec![Ops::Add, Ops::Mul],
        ));
    }

    eqs.iter()
        .filter(|e| e.solve())
        .map(|e| e.goal)
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let mut eqs = Vec::new();

    for line in input.lines() {
        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|s| {
                s.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();

        eqs.push(Equation::new(
            parts[0],
            Vec::from(&parts[1..]),
            vec![Ops::Add, Ops::Mul, Ops::Concat],
        ));
    }

    eqs.iter()
        .filter(|e| e.solve())
        .map(|e| e.goal)
        .sum::<usize>()
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day7.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
