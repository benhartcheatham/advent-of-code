use std::fs;
use std::io;

use aocutils::timing::Timer;

enum Ops {
    Add,
    Mul,
    Concat,
}

impl Ops {
    fn apply(&self, lhs: usize, rhs: usize) -> usize {
        use Ops::*;

        match self {
            Add => lhs + rhs,
            Mul => lhs * rhs,
            Concat => (lhs.to_string() + &rhs.to_string())
                .parse::<usize>()
                .unwrap(),
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
        if idx == self.operands.len() {
            return curr == self.goal;
        }

        self.allowed_ops
            .iter()
            .map(|op| self.solve_helper(op.apply(curr, self.operands[idx]), idx + 1))
            .any(|e| e)
    }

    fn solve(&self) -> bool {
        if self.operands.is_empty() {
            false
        } else if self.operands.len() == 1 {
            self.operands[0] == self.goal
        } else {
            self.solve_helper(self.operands[0], 1)
        }
    }
}

fn part1(input: &str) {
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

    print!(
        "part1: {}",
        eqs.iter()
            .filter(|e| e.solve())
            .map(|e| e.goal)
            .sum::<usize>()
    );
}

fn part2(input: &str) {
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

    print!(
        "part2: {}",
        eqs.iter()
            .filter(|e| e.solve())
            .map(|e| e.goal)
            .sum::<usize>()
    );
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day7.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
